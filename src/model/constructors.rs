use super::*;
use crate::audio::model::AudioModel;
use crate::prelude::*;
use std::sync::Arc;

pub struct AudioSystem {
    audio_stream: Arc<Atomic<AudioModel>>,
    sample_rate: Arc<Atomic<f32>>,
}

impl AudioSystem {
    pub fn build() -> Self {
        // let audio_model = AudioModel::build();

        let audio_host = nannou_audio::Host::new();

        let stream = audio_host
            .new_output_stream(todo!())
            .render(crate::audio::process::process)
            .channels(2)
            .sample_rate(44100)
            .frames_per_buffer(512)
            .build()
            .unwrap();

        let sample_rate = Arc::new(Atomic::new(44100.0));

        Self { audio_stream: todo!(), sample_rate }
    }
}
