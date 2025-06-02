use crate::constants::BACKGROUND_CLEAR_COLOR;
use crate::event::CollisionEvent;
use crate::setup::setup;
use crate::updates::{apply_velocity, detect_ball_collision, handle_ball_bounce, move_paddle};
use bevy::app::{App, FixedUpdate, Startup};
use bevy::DefaultPlugins;

mod components;
mod constants;
mod models;
mod setup;
mod updates;
mod event;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(BACKGROUND_CLEAR_COLOR)
        .add_event::<CollisionEvent>()
        .add_systems(Startup, setup)
        .add_systems(FixedUpdate, (
            move_paddle,
            apply_velocity,
            detect_ball_collision,
            handle_ball_bounce,
        ))
        .run();
}
