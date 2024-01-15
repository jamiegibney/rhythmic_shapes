//! Module for audio state.

use super::*;
use std::time::Instant;

pub struct AudioModel {
    pub voice_handler: VoiceHandler,

    pub note_receiver: mpsc::Receiver<NoteEvent>,
    callback_timer: Instant,
    pub callback_delta_time: Arc<Atomic<f32>>,

    pub sample_rate: Arc<Atomic<f32>>,
}

impl AudioModel {
    pub fn build(ctx: AudioContext) -> Self {
        Self {
            voice_handler: VoiceHandler::build(Arc::clone(&ctx.sample_rate)),
            note_receiver: ctx.note_receiver,
            callback_timer: Instant::now(),
            callback_delta_time: Arc::new(Atomic::new(0.0)),
            sample_rate: ctx.sample_rate,
        }
    }

    pub fn get_callback_timer(&self) -> Arc<Atomic<f32>> {
        Arc::clone(&self.callback_delta_time)
    }

    pub fn set_callback_timer(&mut self) {
        let delta_time = self.callback_timer.elapsed().as_secs_f32();
        if delta_time <= 0.0001 {
            return;
        }

        self.callback_delta_time.sr(delta_time);
        self.callback_timer = Instant::now();
    }
}
