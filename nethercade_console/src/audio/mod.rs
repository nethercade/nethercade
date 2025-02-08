// TODO: Write this

use std::array;

use rodio::{buffer::SamplesBuffer, OutputStream, OutputStreamHandle, Sink};

pub const AUDIO_SINK_COUNT: usize = 32;
pub struct AudioUnit {
    sinks: [Sink; AUDIO_SINK_COUNT],
    _stream_handle: OutputStreamHandle,
    _stream: OutputStream,
}

impl AudioUnit {
    pub fn append_data(&self, index: usize, channels: u16, data: &[f32], sample_rate: u32) {
        if let Some(sink) = self.sinks.get(index) {
            sink.append(SamplesBuffer::new(channels, sample_rate, data));
        }
    }

    // TODO: Add this to the audio context
    pub fn set_volume(&self, volume: f32) {
        for sink in self.sinks.iter() {
            sink.set_volume(volume);
        }
    }

    pub fn new() -> Self {
        let (_stream, _stream_handle) = OutputStream::try_default().unwrap();
        let sinks = array::from_fn(|_| Sink::try_new(&_stream_handle).unwrap());
        Self {
            sinks,
            _stream_handle,
            _stream,
        }
    }
}
