//! Module for the app's state.

use super::*;
use crate::audio::model::AudioModel;
use crate::prelude::*;
use std::sync::Arc;
use std::time::Instant;

/// The app's global state.
pub struct AppModel {
    win: WindowId,
    input_data: InputData,

    sample_rate: Arc<Atomic<f32>>,
    audio_stream: nannou_audio::Stream<AudioModel>,

    frame_timer: Instant,
}

impl AppModel {
    /// Initialises the app's window and state.
    pub fn build(app: &App) -> Self {
        let win = app
            .new_window()
            .size(800, 800)
            .resizable(false)
            .view(super::view::view)
            .title("Rhythmic Shapes DEMO")
            .msaa_samples(4)
            .build()
            .expect("failed to initialise app window!");

        let AudioSystem { audio_stream, sample_rate } = AudioSystem::build();

        Self {
            win,
            input_data: InputData::default(),

            sample_rate,
            audio_stream,

            frame_timer: Instant::now(),
        }
    }

    /// Updates the app's input data each frame.
    pub fn update_input_data(&mut self, app: &App, update: &Update) {
        self.input_data.delta_time = self.frame_timer.elapsed().as_secs_f32();
        self.frame_timer = Instant::now();

        self.input_data.mouse_pos = app.mouse.position();
        self.input_data.is_left_clicked = app.mouse.buttons.left().is_down();
        self.input_data.is_right_clicked = app.mouse.buttons.right().is_down();

        self.input_data.is_shift_pressed = app.keys.mods.shift();
        self.input_data.is_alt_pressed = app.keys.mods.alt();
        self.input_data.is_ctrl_pressed = app.keys.mods.ctrl();
        self.input_data.is_os_pressed = app.keys.mods.logo();
    }
}

/// General user input data.
#[derive(Debug, Clone, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct InputData {
    /// The relative position of the mouse.
    pub mouse_pos: Vec2,
    /// Whether the left mouse button is pressed or not.
    pub is_left_clicked: bool,
    /// Whether the right mouse button is pressed or not.
    pub is_right_clicked: bool,

    /// Whether a shift key is pressed.
    pub is_shift_pressed: bool,
    /// Whether an alt key is pressed.
    pub is_alt_pressed: bool,
    /// Whether the control key is pressed.
    pub is_ctrl_pressed: bool,
    /// Whether the OS button is pressed (command on Mac, Win on Windows).
    pub is_os_pressed: bool,

    /// The time delta since the last frame.
    pub delta_time: f32,
}
