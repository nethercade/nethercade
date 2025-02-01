// TODO: Write this

use nethercade_core::AUDIO_SAMPLE_RATE;
use rodio::{buffer::SamplesBuffer, OutputStream, Sink};

pub struct AudioUnit {
    sink: Sink,
}

impl AudioUnit {
    pub fn append_data(&self, channels: usize, data: &[f32]) {
        self.sink
            .append(SamplesBuffer::new(channels as u16, AUDIO_SAMPLE_RATE, data));
    }

    pub fn set_volume(&self, volume: f32) {
        self.sink.set_volume(volume);
    }

    pub fn new() -> Self {
        let (_stream, stream_handle) = OutputStream::try_default().unwrap();
        let sink = Sink::try_new(&stream_handle).unwrap();

        Self { sink }
    }
}
