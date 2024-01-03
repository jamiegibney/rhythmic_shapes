use super::*;
use crate::model::InputData;

pub trait Drawable {
    fn update(&mut self, input_data: &InputData);
    fn force_redraw(&self, draw: &Draw);
    fn draw(&self, draw: &Draw, frame: &Frame);
    fn rect(&self) -> &Rect;

    fn draw_bounding_rect(&self, draw: &Draw) {
        let r = self.rect();

        draw.rect()
            .xy(r.xy())
            .wh(r.wh())
            .color(Rgba::new(0.0, 1.0, 0.0, 1.0));
    }

    fn should_update(&self) -> bool {
        true
    }

    fn should_redraw(&self) -> bool {
        true
    }
}
