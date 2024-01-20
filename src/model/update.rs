//! App update callback.
use super::*;
use crate::prelude::*;

pub fn update(app: &App, model: &mut AppModel, update: Update) {
    model.update_input_data(app, &update);
    let input_data = &model.input_data;

    model.tempo_ui.update(input_data);
    model.time_signature_ui.update(input_data);

    model.update_sequencer_params();

    model.sequencer.update(&model.input_data);
}
