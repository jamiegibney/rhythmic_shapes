//! Module for the app's state.

pub mod app_model;
mod constructors;
mod event;
pub mod params;
pub mod update;
mod view;

pub use app_model::*;
pub use update::update;

use constructors::*;
