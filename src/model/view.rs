//! App view callback (i.e. "draw loop");

use super::*;
use crate::prelude::*;

pub fn view(app: &App, model: &AppModel, frame: Frame) {
    let draw = &app.draw();
    let frame = &frame;
    if frame.nth() == 0 {
        draw.background().color(WHITE);
        draw.text("Press \"R\" to reset shape")
            .xy(pt2(0.0, 380.0))
            .wh(pt2(300.0, 50.0))
            .color(BLACK)
            .font_size(20);
    }

    draw.rect()
        .xy(model.sequencer_rect.xy())
        .wh(model.sequencer_rect.wh() * 1.05)
        .color(Rgba::new(1.0, 1.0, 1.0, 1.0));

    model.tempo_ui.draw(draw, frame);
    model.time_signature_ui.draw(draw, frame);
    model.sequencer.draw(draw, frame);

    _ = draw.to_frame(app, frame);
}
