#![allow(unused)]

mod model;
mod util;
mod ui;
mod sequencer;
mod prelude;
mod audio;

use prelude::*;

fn main() {
    nannou::app(model::AppModel::build)
        .loop_mode(nannou::LoopMode::RefreshSync)
        .update(model::update)
        .run();
}
