//! App view callback (i.e. "draw loop");

use super::*;
use crate::prelude::*;

pub fn view(app: &App, model: &AppModel, frame: Frame) {
    let draw = &app.draw();
    let frame = &frame;
    draw.background().color(BLACK);

    draw.rect()
        .xy(model.sequencer_rect.xy())
        .wh(model.sequencer_rect.wh())
        .color(Rgba::new(0.0, 1.0, 0.0, 0.1));

    model.tempo_ui.draw(draw, frame);

    _ = draw.to_frame(app, frame);
}
