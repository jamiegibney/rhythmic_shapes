//! Module for audio state.

use super::*;

pub struct AudioModel {
    pub voice_handler: VoiceHandler,

    pub note_receiver: mpsc::Receiver<NoteEvent>,

    pub sample_rate: Arc<Atomic<f32>>,
}

impl AudioModel {
    pub fn build(ctx: AudioContext) -> Self {
        Self {
            voice_handler: VoiceHandler::build(
                Arc::clone(&ctx.sample_rate),
            ),
            note_receiver: ctx.note_receiver,
            sample_rate: ctx.sample_rate,
        }
    }
}
