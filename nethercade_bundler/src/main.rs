mod config;

use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
};

use clap::Parser;
use config::Config;
use nethercade_core::{Rom, ROM_FILE_EXTENSION};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Path to the bundler configuration
    #[arg(short, long)]
    bundler_config_path: String,
}

fn main() {
    let args = Args::parse();

    let bundle = match File::open(args.bundler_config_path) {
        Ok(mut file) => {
            let mut buf = Vec::new();
            file.read_to_end(&mut buf).unwrap();
            match sonic_rs::from_slice::<Config>(&buf) {
                Ok(bundle) => bundle,
                Err(e) => {
                    println!("Failed to parse rom settings file: {e:?}");
                    return;
                }
            }
        }
        Err(e) => {
            println!("Failed to load rom_settings_path{e:?}");
            return;
        }
    };

    let code = match File::open(&bundle.wasm_path) {
        Ok(mut code) => {
            let mut buf = Vec::new();
            code.read_to_end(&mut buf).unwrap();
            buf.into_boxed_slice()
        }
        Err(e) => {
            println!("Failed to load rom_settings_path{e:?}");
            return;
        }
    };

    let output_path = match bundle.output_file {
        Some(output_file) => output_file.with_extension(ROM_FILE_EXTENSION),
        None => PathBuf::new()
            .with_file_name(bundle.wasm_path.file_name().unwrap())
            .with_extension(ROM_FILE_EXTENSION),
    };

    let mut out_file = match File::options()
        .create(true)
        .write(true)
        .truncate(true)
        .open(&output_path)
    {
        Ok(file) => file,
        Err(e) => {
            println!("Failed to open output file: {e:?}");
            return;
        }
    };

    let rom = Rom {
        code,
        resolution: bundle.resolution.unwrap_or_default(),
        frame_rate: bundle.frame_rate.unwrap_or_default(),
    };

    let bytes = bitcode::encode(&rom);
    let bytes = zstd::bulk::compress(&bytes, 0).unwrap();
    out_file.write_all(&bytes).unwrap();
    println!("Output file {output_path:?} successfully.");
}
