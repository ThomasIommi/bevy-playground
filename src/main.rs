use crate::constants::BACKGROUND_CLEAR_COLOR;
use crate::setup::setup;
use crate::updates::{velocity_update, puddle_movement};
use bevy::app::{App, FixedUpdate, Startup};
use bevy::DefaultPlugins;

mod components;
mod constants;
mod models;
mod setup;
mod updates;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(BACKGROUND_CLEAR_COLOR)
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            puddle_movement,
            velocity_update
        ))
        .run();
}
