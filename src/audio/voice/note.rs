use super::*;

/// Raw note events to be sent to the audio thread.
#[derive(Clone, Copy, Debug)]
pub enum NoteEvent {
    NoteOn {
        timing: u32,
        data: NoteEventData,
    },
    NoteOff {
        timing: u32,
        data: NoteEventData,
    },
}

impl NoteEvent {
    pub fn timing(&self) -> u32 {
        match self {
            NoteEvent::NoteOn { timing, .. } => *timing,
            NoteEvent::NoteOff { timing, .. } => *timing,
        }
    }

    pub fn note(&self) -> f32 {
        match self {
            NoteEvent::NoteOn { data, .. } => data.note,
            NoteEvent::NoteOff { data, .. } => data.note,
        }
    }
}

/// The data encoded into each note event.
#[derive(Clone, Copy, Debug)]
pub struct NoteEventData {
    note: f32,
}
