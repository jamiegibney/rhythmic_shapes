use super::*;
use crate::audio::sine::SineOsc;
use std::sync::Arc;

pub mod handler;
pub mod note;

pub use handler::{VoiceHandler, NUM_VOICES};
pub use note::{NoteEvent, NoteEventData};

/// A struct to represent each individual voice.
#[derive(Clone, Debug)]
pub struct Voice {
    /// The voice's unique ID.
    pub id: u64,
    /// The MIDI note of the voice.
    pub note: f32,

    pub envelope_data: Arc<[f32]>,

    /// The voice's ADSR envelope.
    pub envelope_idx: usize,

    pub sample_rate: Arc<Atomic<f32>>,

    pub oscillator: SineOsc,
}

impl Voice {
    pub fn new(
        id: u64,
        note: f32,
        sample_rate: Arc<Atomic<f32>>,
        envelope_ref: Arc<[f32]>,
    ) -> Self {
        Self {
            id,
            note,
            envelope_data: envelope_ref,
            envelope_idx: 0,
            oscillator: SineOsc::new(note_to_freq(note), sample_rate.lr()),
            sample_rate,
        }
    }

    pub fn envelope_is_finished(&self) -> bool {
        self.envelope_idx > self.envelope_data.len()
    }

    pub fn next_envelope_block(&mut self, block: &mut [f32], block_len: usize) {
        let env_len = self.envelope_data.len();
        let pos = self.envelope_idx;

        let rem = env_len - pos;
        let num_iters = rem.min(block_len);

        for i in 0..num_iters {
            block[i] = self.envelope_data[pos + i];
        }

        let excess = block_len - num_iters;

        for i in 0..excess {
            block[i] = 0.0;
        }

        self.envelope_idx += num_iters;
    }
}
