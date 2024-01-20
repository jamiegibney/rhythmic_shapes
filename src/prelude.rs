//! Module for project-wide imports.

pub use crate::model::InputData;
pub use crate::ui::Drawable;
pub use crate::util::*;
pub use atomic::Atomic;
pub use nannou::prelude::*;
pub use nannou_audio::{Buffer, Stream};

pub type NoteSender =
    std::sync::Arc<std::sync::mpsc::Sender<crate::audio::NoteEvent>>;
