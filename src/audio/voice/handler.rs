//! Polyphonic voice types and management.

use super::*;
use atomic::Atomic;
use nannou_audio::Buffer;
use std::sync::{mpsc, Arc, Mutex};

pub const NUM_VOICES: usize = 16;

/// A struct to handle all voices, i.e. the spawning and termination of voices.
#[derive(Debug)]
pub struct VoiceHandler {
    /// The array of voices.
    pub voices: [Option<Voice>; NUM_VOICES],

    /// Internal counter for assigning new IDs.
    id_counter: u64,

    /// Sample rate for setting the correct frequency of voice oscillators.
    sample_rate: Arc<Atomic<f32>>,

    envelope_data: Arc<[f32]>,
}

impl VoiceHandler {
    /// Builds a new `VoiceHandler` with a reference to the `NoteHandler`.
    ///
    /// The `NoteHandler` reference is used to obtain new note events
    /// automatically.
    pub fn build(sample_rate_ref: Arc<Atomic<f32>>) -> Self {
        Self {
            // note_handler_ref,
            voices: std::array::from_fn(|_| None),
            id_counter: 0,
            envelope_data: build_envelope(sample_rate_ref.lr(), 0.3),
            sample_rate: sample_rate_ref,
        }
    }

    /// Attaches a reference to the sample rate to the `VoiceHandler`.
    pub fn attach_sample_rate_ref(
        &mut self,
        sample_rate_ref: Arc<Atomic<f32>>,
    ) {
        self.sample_rate = sample_rate_ref;
    }

    pub fn process_block(
        &mut self,
        buffer: &mut Buffer,
        block_start: usize,
        block_end: usize,
    ) {
        let block_len = block_end - block_start;
        let mut voice_amp_envelope = [0.0; MAX_BLOCK_SIZE];

        for voice in self.voices.iter_mut().filter_map(|v| v.as_mut()) {
            voice.next_envelope_block(&mut voice_amp_envelope, block_len);

            for (value_idx, sample_idx) in (block_start..block_end).enumerate()
            {
                let amp = voice_amp_envelope[value_idx];

                let out = voice.oscillator.process();

                // * 2 because the channels are interleaved
                buffer[sample_idx * 2] += out * amp;
                buffer[sample_idx * 2 + 1] += out * amp;
            }
        }
    }

    /// Starts a new voice.
    #[allow(clippy::missing_panics_doc)] // this function should not panic
    pub fn start_voice(&mut self, note: f32) -> &mut Voice {
        let next_voice_id = self.next_voice_id();

        let new_voice = Voice {
            id: next_voice_id,
            note,
            envelope_data: Arc::clone(&self.envelope_data),
            envelope_idx: 0,
            sample_rate: Arc::clone(&self.sample_rate),
            oscillator: SineOsc::new(note_to_freq(note), self.sample_rate.lr()),
        };

        // is there a free voice?
        if let Some(free_idx) =
            self.voices.iter().position(|voice| voice.is_none())
        {
            self.voices[free_idx] = Some(new_voice);
            return self.voices[free_idx].as_mut().unwrap();
        }

        // as we know voices are in use, we can use unwrap_unchecked()
        // to avoid some unnecessary checks.
        let oldest_voice = unsafe {
            self.voices
                .iter_mut()
                .min_by_key(|voice| voice.as_ref().unwrap_unchecked().id)
                .unwrap_unchecked()
        };

        *oldest_voice = Some(new_voice);
        return oldest_voice.as_mut().unwrap();
    }

    /// Immediately terminates all active voices.
    pub fn kill_active_voices(&mut self) {
        self.voices.iter_mut().for_each(|v| {
            if v.is_some() {
                *v = None;
            }
        });
    }

    /// Terminates all voices which are releasing and which have an
    /// idle envelope.
    pub fn terminate_finished_voices(&mut self) {
        for voice in &mut self.voices {
            match voice {
                Some(v) if v.envelope_is_finished() => {
                    *voice = None;
                }
                _ => (),
            }
        }
    }

    /// Returns whether there is at least one voice active or not.
    pub fn is_voice_active(&self) -> bool {
        self.voices.iter().any(|v| v.is_some())
    }

    fn next_voice_id(&mut self) -> u64 {
        self.id_counter = self.id_counter.wrapping_add(1);
        self.id_counter
    }
}

/// Builds the envelope data.
fn build_envelope(sample_rate: f32, envelope_time_secs: f32) -> Arc<[f32]> {
    let num_steps = sample_rate * envelope_time_secs;

    (0..num_steps as usize)
        .rev()
        .map(|i| (i as f32 / num_steps) * 0.125)
        .collect()
}
