//! Module for the app's state.

use super::*;
use crate::audio::voice::NoteEventData;
use crate::prelude::*;
use crate::ui::shape::Sequence;
use crate::ui::text_slider::TextSlider;
use crate::{
    audio::{model::AudioModel, voice::NoteEvent},
    ui::default_text_layout,
};
use std::sync::{
    atomic::AtomicU32,
    mpsc::{self, channel},
    Arc, Mutex,
};
use std::thread;
use std::time::Instant;

const DEFAULT_BPM: f32 = 120.0;
const DEFAULT_NUM_NODES: usize = 4;

/// The app's global state.
pub struct AppModel {
    /// The window ID.
    win: WindowId,
    /// User input data.
    pub input_data: InputData,

    /// The app's sample rate.
    pub sample_rate: Arc<Atomic<f32>>,
    /// The app's audio stream.
    audio_stream: Stream<AudioModel>,
    callback_timer: Arc<Atomic<f32>>,

    /// The bounding rect for the shape sequencer.
    pub sequencer_rect: Rect,
    pub sequencer: Sequence,
    /// The UI component for controlling the app's tempo.
    pub tempo_ui: TextSlider,
    tempo_param: Arc<Atomic<f32>>,

    pub time_signature_ui: TextSlider,
    time_signature_param: Arc<AtomicU32>,
    time_signature_params_last: u32,

    note_event_sender: Arc<mpsc::Sender<NoteEvent>>,
    note_data_receiver: Arc<Mutex<mpsc::Receiver<NoteEventData>>>,

    note_event_thread: thread::JoinHandle<()>,

    /// The timer for tracking the frame delta time.
    frame_timer: Instant,
}

impl AppModel {
    /// Initialises the app's window and state.
    pub fn build(app: &App) -> Self {
        let win = app
            .new_window()
            .size(800, 800)
            .resizable(false)
            .key_pressed(key_pressed)
            .view(super::view::view)
            .title("Rhythmic Shapes Demo")
            .msaa_samples(4)
            .build()
            .expect("failed to initialise app window!");

        let AudioSystem {
            audio_stream,
            sample_rate,
            note_event_sender,
            callback_timer,
        } = AudioSystem::build();

        let sequencer_rect = Rect::from_wh(pt2(650.0, 650.0));

        let tempo_param = Arc::new(Atomic::new(DEFAULT_BPM));
        let time_signature_param =
            Arc::new(AtomicU32::new(DEFAULT_NUM_NODES as u32));

        let note_event_sender = Arc::new(note_event_sender);

        let (note_data_tx, note_data_rx) = channel();
        let note_data_receiver = Arc::new(Mutex::new(note_data_rx));

        Self {
            win,
            input_data: InputData::default(),

            audio_stream,

            sequencer_rect,
            sequencer: {
                let sender = Arc::clone(&note_event_sender);
                Sequence::new(
                    sequencer_rect, note_data_tx, DEFAULT_NUM_NODES,
                    DEFAULT_BPM,
                )
            },
            tempo_ui: {
                let param = Arc::clone(&tempo_param);
                TextSlider::new(
                    0.0,
                    Rect::from_xy_wh(pt2(150.0, -380.0), pt2(60.0, 25.0)),
                )
                .with_label("Tempo")
                .with_label_layout(default_text_layout())
                .with_value_layout(default_text_layout())
                .with_value_chars(3)
                .with_integer_rounding()
                .with_output_range(60.0..=200.0)
                .with_default_value(DEFAULT_BPM)
                .with_sensitivity(0.002)
                .with_callback(move |_, value| {
                    param.sr(value);
                })
            },
            tempo_param,

            time_signature_ui: {
                let param = Arc::clone(&time_signature_param);
                TextSlider::new(
                    0.0,
                    Rect::from_xy_wh(pt2(-150.0, -380.0), pt2(60.0, 25.0)),
                )
                .with_label("Time signature")
                .with_label_layout(default_text_layout())
                .with_value_layout(default_text_layout())
                .with_value_chars(3)
                .with_suffix("/4")
                .with_integer_rounding()
                .with_output_range(3.0..=8.0)
                .with_default_value(DEFAULT_NUM_NODES as f32)
                .with_sensitivity(0.008)
                .with_callback(move |_, value| {
                    param.sr(value as u32);
                })
            },
            time_signature_params_last: time_signature_param.lr(),
            time_signature_param,

            note_event_thread: {
                let recv = Arc::clone(&note_data_receiver);
                let timer = Arc::clone(&callback_timer);
                let sr = Arc::clone(&sample_rate);
                let sender = Arc::clone(&note_event_sender);

                thread::spawn(move || loop {
                    if let Ok(guard) = recv.lock() {
                        if let Ok(msg) = guard.recv() {
                            let timer = timer.lr();
                            let samples_exact = timer * sr.lr();
                            let timing = samples_exact.round() as u32
                                % BUFFER_SIZE as u32;
                            sender
                                .send(NoteEvent::NoteOn { timing, data: msg });
                        }
                    }
                })
            },

            sample_rate,
            callback_timer,

            note_event_sender,
            note_data_receiver,

            frame_timer: Instant::now(),
        }
    }

    /// Updates the app's input data each frame.
    pub fn update_input_data(&mut self, app: &App, update: &Update) {
        self.input_data.delta_time = self.frame_timer.elapsed().as_secs_f32();
        self.frame_timer = Instant::now();

        self.input_data.mouse_pos = app.mouse.position();

        self.input_data.left_button_last = self.input_data.is_left_clicked;
        self.input_data.is_left_clicked = app.mouse.buttons.left().is_down();

        self.input_data.right_button_last = self.input_data.is_right_clicked;
        self.input_data.is_right_clicked = app.mouse.buttons.right().is_down();

        self.input_data.is_shift_pressed = app.keys.mods.shift();
        self.input_data.is_alt_pressed = app.keys.mods.alt();
        self.input_data.is_ctrl_pressed = app.keys.mods.ctrl();
        self.input_data.is_os_pressed = app.keys.mods.logo();
    }

    pub fn current_sample_idx(&self) -> u32 {
        let timer = self.callback_timer.lr();
        let samples_exact = timer * self.sample_rate.lr();

        samples_exact.round() as u32 % BUFFER_SIZE as u32
    }

    pub fn update_sequencer_params(&mut self) {
        self.sequencer.set_tempo(self.tempo_param.lr());

        let ts_param = self.time_signature_param.lr();
        if ts_param != self.time_signature_params_last {
            self.time_signature_params_last = ts_param;
            self.sequencer.set_num_nodes(ts_param as usize);
        }
    }
}

/// General user input data.
#[derive(Debug, Clone, Default)]
#[allow(clippy::struct_excessive_bools)]
pub struct InputData {
    /// The relative position of the mouse.
    pub mouse_pos: Vec2,
    pub scroll_delta: Vec2,
    /// Whether the left mouse button is pressed or not.
    pub is_left_clicked: bool,
    left_button_last: bool,
    /// Whether the right mouse button is pressed or not.
    pub is_right_clicked: bool,
    right_button_last: bool,

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

impl InputData {
    /// Returns `true` if the left mouse button was just pressed down.
    pub fn left_button_just_clicked(&self) -> bool {
        !self.left_button_last && self.is_left_clicked
    }

    /// Returns `true` if the left mouse button was just unpressed.
    pub fn left_button_just_lifted(&self) -> bool {
        self.left_button_last && !self.is_left_clicked
    }
}

fn key_pressed(_: &App, app_model: &mut AppModel, key: Key) {
    if key == Key::R {
        app_model.sequencer.reset();
    }
}
