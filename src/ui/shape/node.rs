//! Module for individual nodes.

use crate::prelude::*;

const FLASH_TIME_SECS: f32 = 0.5;

pub struct Node {
    bounding_rect: Rect,
    rect: Rect,
    pos: Vec2,

    color: Rgba,
    radius: f32,

    flash_timer: f32,
    is_hovered: bool,
    is_clicked: bool,
}

impl Node {
    pub fn new(bounding_rect: Rect) -> Self {
        let pos = bounding_rect.xy();
        let radius = 2.0;
        Self {
            bounding_rect,
            rect: Rect::from_xy_wh(pos, pt2(radius * 2.0, radius * 2.0)),
            pos,
            color: Rgba::new(0.0, 0.0, 0.0, 0.5),
            radius,
            flash_timer: 0.0,

            is_hovered: false,
            is_clicked: false,
        }
    }
}

impl Drawable for Node {
    fn update(&mut self, input_data: &crate::model::InputData) {
        self.is_hovered = self.rect.contains(input_data.mouse_pos);

        if !self.is_clicked && input_data.is_left_clicked {
            self.is_clicked = true;
            self.flash_timer = FLASH_TIME_SECS;
        }
        else if self.is_clicked {
            self.flash_timer =
                (self.flash_timer - input_data.delta_time).max(0.0);
        }
        else if !input_data.is_left_clicked {
            self.is_clicked = false;
        }
    }

    fn force_redraw(&self, draw: &Draw, frame: &Frame) {
        self.draw(draw, frame);
    }

    fn draw(&self, draw: &Draw, _: &Frame) {
        todo!("proper flash logic");

        let mut col = self.color;
        col.alpha = FLASH_TIME_SECS / self.flash_timer;

        draw.ellipse()
            .xy(self.pos)
            .radius(self.radius)
            .color(col);
    }

    fn rect(&self) -> &Rect {
        &self.rect
    }
}
