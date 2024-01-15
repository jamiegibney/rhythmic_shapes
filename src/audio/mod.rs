//! Module for audio processing and state.
use crate::prelude::*;
use std::sync::{mpsc, Arc};

pub mod model;
pub mod process;
pub mod sine;
pub mod voice;

pub use process::MAX_BLOCK_SIZE;
pub use voice::{NoteEvent, VoiceHandler};

/// Contextual data and channels to pass to the audio thread.
pub struct AudioContext {
    pub sample_rate: Arc<Atomic<f32>>,
    pub note_receiver: mpsc::Receiver<NoteEvent>,
}

