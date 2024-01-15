use super::*;
use crate::audio::AudioContext;
use crate::audio::{self, model::AudioModel, NoteEvent};
use crate::prelude::*;
use std::sync::{mpsc, Arc};
use std::time::Instant;

pub const BUFFER_SIZE: usize = 512;

pub struct AudioSystem {
    pub audio_stream: Stream<AudioModel>,
    pub sample_rate: Arc<Atomic<f32>>,
    pub note_event_sender: mpsc::Sender<NoteEvent>,
    pub callback_timer: Arc<Atomic<f32>>,
}

impl AudioSystem {
    pub fn build() -> Self {
        let sample_rate = Arc::new(Atomic::new(44100.0));

        let (note_tx, note_rx) = mpsc::channel();

        let audio_ctx = AudioContext {
            sample_rate: Arc::clone(&sample_rate),
            note_receiver: note_rx,
        };

        let audio_model = AudioModel::build(audio_ctx);
        let callback_timer = audio_model.get_callback_timer();

        let audio_host = nannou_audio::Host::new();

        let audio_stream = audio_host
            .new_output_stream(audio_model)
            .render(audio::process::process)
            .channels(2)
            .sample_rate(44100)
            .frames_per_buffer(BUFFER_SIZE)
            .build()
            .unwrap();

        Self {
            audio_stream,
            sample_rate,
            note_event_sender: note_tx,
            callback_timer,
        }
    }
}
