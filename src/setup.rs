use crate::components::{Paddle, Wall};
use crate::constants::{PADDLE_COLOR, PADDLE_SIZE, PADDLE_Y, WALL_COLOR};
use crate::models::WallLocation;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, Camera2d, Commands, Sprite, Transform};
use bevy::utils::default;

pub(crate) fn setup(mut commands: Commands) {
    commands.spawn(Camera2d);

    commands.spawn(make_puddle());
    commands.spawn(make_wall(WallLocation::Top));
    commands.spawn(make_wall(WallLocation::Right));
    commands.spawn(make_wall(WallLocation::Bottom));
    commands.spawn(make_wall(WallLocation::Left));
}

// todo change into a struct Puddle, that derives Component and has an impl for a default init
fn make_puddle() -> impl Bundle {
    (
        Paddle,
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, PADDLE_Y, 0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
    )
}

fn make_wall(location: WallLocation) -> impl Bundle {
    (
        Wall,
        Sprite::from_color(WALL_COLOR, Vec2::ONE),
        Transform {
            translation: location.position().extend(0.0),
            scale: location.size().extend(1.0),
            ..default()
        },
    )
}
