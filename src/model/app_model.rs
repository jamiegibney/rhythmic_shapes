//! Module for the app's state.

use crate::prelude::*;

pub struct AppModel {
    input_data: InputData,
}

impl AppModel {
    pub fn build() -> Self {
        Self { input_data: InputData::default() }
    }
}

pub fn event() {
    // 
}

#[derive(Debug, Clone, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct InputData {
    pub mouse_pos: Vec2,
    pub is_left_clicked: bool,
    pub is_right_clicked: bool,

    pub is_shift_pressed: bool,
    pub is_alt_pressed: bool,
    pub is_ctrl_pressed: bool,
    pub is_os_pressed: bool,

    pub delta_time: f32,
}
