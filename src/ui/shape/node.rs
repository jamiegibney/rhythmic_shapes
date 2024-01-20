//! Module for individual node objects.
use crate::{audio::voice::NoteEventData, prelude::*};

const FLASH_TIME_SECS: f32 = 0.40;

#[derive(Clone, Debug)]
pub struct Node {
    bounding_rect: Rect,
    rect: Rect,
    pub pos: Vec2,
    mouse_offset: Option<Vec2>,

    pub color: Rgba,
    radius: f32,

    flash_timer: f32,
    is_hovered: bool,
    is_clicked: bool,

    note_data: NoteEventData,
}

impl Node {
    pub fn new(bounding_rect: Rect) -> Self {
        let pos = bounding_rect.xy();
        let radius = 10.0;

        Self {
            bounding_rect,
            rect: Rect::from_xy_wh(pos, pt2(radius * 2.0, radius * 2.0)),
            pos,
            mouse_offset: None,

            color: Rgba::new(0.0, 1.0, 0.0, 1.0),
            radius,
            flash_timer: 0.0,

            is_hovered: false,
            is_clicked: false,

            note_data: NoteEventData { note: 69.0 },
        }
    }

    pub fn tap(&mut self) {
        self.flash_timer = 1.0;
    }

    pub fn is_clicked(&self) -> bool {
        self.is_clicked
    }

    pub fn update_flash_timer(&mut self, input_data: &InputData) {
        self.flash_timer = f32::max(
            self.flash_timer - input_data.delta_time / FLASH_TIME_SECS,
            0.0,
        );
    }

    pub fn note_data(&self) -> NoteEventData {
        self.note_data
    }

    pub fn note_data_mut(&mut self) -> &mut NoteEventData {
        &mut self.note_data
    }
}

impl Drawable for Node {
    fn update(&mut self, input_data: &InputData) {
        let mp = input_data.mouse_pos;
        let lmb_down = input_data.left_button_just_clicked();
        let lmb_up = input_data.left_button_just_lifted();

        let r = self.radius * 2.0;
        let rect = Rect::from_xy_wh(self.pos, pt2(r, r));
        self.is_hovered = rect.contains(mp);

        if !input_data.is_left_clicked {
            self.is_clicked = false;
        }

        if self.is_hovered {
            if input_data.left_button_just_clicked() {
                self.is_clicked = true;
                self.mouse_offset = Some(self.pos - mp);
            }
            else if input_data.left_button_just_lifted() {
                self.is_clicked = false;
            }
        }
        else if lmb_down {
            self.is_clicked = false;
        }

        if self.is_clicked {
            let delta = self.mouse_offset.unwrap_or(Vec2::ZERO);
            self.pos = (mp + delta).clamp(
                self.bounding_rect.bottom_left(),
                self.bounding_rect.top_right(),
            );
        }
    }

    fn force_redraw(&self, draw: &Draw, frame: &Frame) {
        self.draw(draw, frame);
    }

    fn draw(&self, draw: &Draw, _: &Frame) {
        let col =
            blend(Rgba::new(1.0, 1.0, 1.0, 1.0), self.color, self.flash_timer);

        draw.ellipse()
            .xy(self.pos)
            .radius(self.radius)
            .color(col)
            .stroke_color(BLACK)
            .stroke_weight(3.0);
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }
}

fn blend(col1: Rgba, col2: Rgba, t: f32) -> Rgba {
    Rgba::new(
        lerp(col1.red, col2.red, t),
        lerp(col1.green, col2.green, t),
        lerp(col1.blue, col2.blue, t),
        lerp(col1.alpha, col2.alpha, t),
    )
}
