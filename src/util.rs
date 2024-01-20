//! Module for utility functions.

pub mod atomic_ops;
pub mod ramp;

pub use atomic_ops::AtomicOps;
pub use ramp::Ramp;

#[derive(Debug, Copy, Clone, Default)]
pub enum SmoothingType {
    /// Linear mapping from `a -> b`
    #[default]
    Linear,
    /// Cosine function mapping from `a -> b`
    Cosine,
    /// Quarter-sine function mapping from `a -> b`, biased towards b
    SineTop,
    /// Quarter-sine function mapping from `a -> b`, biased towards a
    SineBottom,
    /// Standard curve mapping from `a -> b` with tension argument
    CurveNormal(f64),
    /// Curved mapping from `a -> b` with tension argument and a linear start
    CurveLinearStart(f64),
    /// Rounder curve mapping from `a -> b` with tension argument
    CurveRounder(f64),
}

/// Returns true if `value` is equal to `target`, with a tolerance of
/// ±`f32::EPSILON`.
#[inline]
pub fn epsilon_eq(value: f32, target: f32) -> bool {
    (target - value).abs() < f32::EPSILON
}

/// Calculates the frequency value of the provided MIDI note value.
#[inline]
pub fn note_to_freq(note_value: f32) -> f32 {
    ((note_value - 69.0) / 12.0).exp2() * 440.0
}

/// Calculates the MIDI note value of the provided frequency value.
#[inline]
pub fn freq_to_note(freq: f32) -> f32 {
    12.0f32.mul_add((freq / 440.0).log2(), 69.0)
}

/// Calculates amplitude in decibels from a linear power level.
#[inline]
pub fn level_to_db(level: f32) -> f32 {
    20.0 * level.log10()
}

/// Calculates the linear power level from amplitude as decibels.
#[inline]
pub fn db_to_level(db_value: f32) -> f32 {
    10.0f32.powf(db_value / 20.0)
}

/// Maps a value from the provided input range to the provided output range.
#[inline]
pub fn map(
    value: f32,
    in_min: f32,
    in_max: f32,
    out_min: f32,
    out_max: f32,
) -> f32 {
    scale(normalize(value, in_min, in_max), out_min, out_max)
}

/// Scales a value to a provided range, assuming it is normalised.
///
/// Like `map()`, but with no input range.
#[inline]
pub fn scale(value: f32, min: f32, max: f32) -> f32 {
    value.mul_add(max - min, min)
}

/// Normalizes a value from a provided range.
///
/// Like `map()`, but with the output range set to `0.0 - 1.0`.
#[inline]
pub fn normalize(value: f32, min: f32, max: f32) -> f32 {
    (value - min) / (max - min)
}

/// Linearly interpolates between `a` and `b` based on the value of `t`.
///
/// `t` is clamped between `0` and `1`.
pub fn lerp(a: f32, b: f32, t: f32) -> f32 {
    let t = t.clamp(0.0, 1.0);
    if t == 0.0 {
        return a;
    } else if t == 1.0 {
        return b;
    }

    t.mul_add(b - a, a)
}

/// "Inverse linear interpolation": finds the interpolation value
/// within a range.
pub fn ilerp(a: f32, b: f32, val: f32) -> f32 {
    if b == a {
        return 0.0;
    }

    (val - a) / (b - a)
}
