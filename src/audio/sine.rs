//! Module for sine wave generator.

use super::*;

/// Primitive sine wave oscillator.
#[derive(Debug, Clone, Copy)]
pub struct SineOsc {
    phase: f32,
    phase_increment: f32,
}

impl SineOsc {
    /// Creates a new `SineOsc`.
    pub fn new(freq_hz: f32, sample_rate: f32) -> Self {
        debug_assert!(0.0 < freq_hz && freq_hz <= sample_rate * 0.5);

        let phase_increment = freq_hz / sample_rate * TAU;

        Self { phase: 0.0, phase_increment }
    }

    /// Sets the frequency of the oscillator.
    pub fn set_frequency(&mut self, freq_hz: f32, sample_rate: f32) {
        debug_assert!(0.0 < freq_hz && freq_hz <= sample_rate * 0.5);

        self.phase_increment = freq_hz / sample_rate * TAU;
    }

    /// Produces the next sine wave value.
    pub fn process(&mut self) -> f32 {
        let out = self.phase.sin();
        self.increment();

        out
    }

    fn increment(&mut self) {
        self.phase += self.phase_increment;

        if TAU <= self.phase {
            self.phase -= TAU;
        }
    }
}
