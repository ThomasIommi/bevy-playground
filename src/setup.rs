use crate::components::{Ball, Paddle, Velocity, Wall};
use crate::constants::{BALL_COLOR, BALL_SIZE, BALL_SPEED, BALL_Y, PADDLE_COLOR, PADDLE_SIZE, PADDLE_Y, WALL_COLOR};
use crate::models::WallLocation;
use bevy::asset::Assets;
use bevy::math::{Vec2, Vec3};
use bevy::prelude::{Bundle, Camera2d, Circle, ColorMaterial, Commands, Mesh, Mesh2d, ResMut, Sprite, Transform};
use bevy::sprite::MeshMaterial2d;
use bevy::utils::default;

pub(crate) fn setup(mut commands: Commands,
                    mut meshes: ResMut<Assets<Mesh>>,
                    mut materials: ResMut<Assets<ColorMaterial>>) {
    commands.spawn(Camera2d);

    commands.spawn(make_puddle());
    commands.spawn(make_wall(WallLocation::Top));
    commands.spawn(make_wall(WallLocation::Right));
    commands.spawn(make_wall(WallLocation::Bottom));
    commands.spawn(make_wall(WallLocation::Left));

    commands.spawn(make_ball(&mut meshes, &mut materials));
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

fn make_ball(meshes: &mut ResMut<Assets<Mesh>>,
             materials: &mut ResMut<Assets<ColorMaterial>>) -> impl Bundle {
    (
        Ball,
        Velocity(Vec2::new(1., 1.).normalize() * BALL_SPEED),
        Mesh2d(meshes.add(Circle::default())),
        MeshMaterial2d(materials.add(BALL_COLOR)),
        Transform {
            translation: Vec3::new(0.0, BALL_Y, 0.0),
            scale: BALL_SIZE.extend(1.0),
            ..default()
        }
    )
}
