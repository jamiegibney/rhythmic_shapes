//! Module for UI components

use crate::prelude::*;
use nannou::text::{Align, Justify, Layout};
use std::marker::PhantomData as PD;

pub mod draw;
pub mod shape;
pub mod text_slider;

pub use draw::Drawable;

/// Value text labels.
pub const LABEL: Rgb = Rgb { red: 0.5, green: 0.5, blue: 0.5, standard: PD };

/// Non-selected background (main background color).
pub const BG_NON_SELECTED: Rgb =
    Rgb { red: 0.18, green: 0.18, blue: 0.18, standard: PD };

/// Value text labels.
pub const VALUE: Rgb =
    Rgb { red: 1.0, green: 1.0, blue: 1.0, standard: PD };

#[derive(Clone, Copy, Debug, Default)]
pub enum UIComponentState {
    #[default]
    Idle,
    Hovered,
    Disabled,
    Clicked,
}

pub fn str_to_option(s: &str) -> Option<String> {
    (!s.is_empty()).then_some(s.to_string())
}

pub fn default_text_layout() -> Layout {
    Layout {
        line_spacing: 0.0,
        line_wrap: None,
        justify: Justify::Center,
        font_size: 20,
        font: None,
        y_align: Align::Middle,
    }
}
