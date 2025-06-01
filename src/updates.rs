use crate::components::{Ball, Paddle};
use crate::constants::{PADDLE_SIZE, PADDLE_SPEED, WALL_LEFT, WALL_RIGHT, WALL_THICKNESS};
use bevy::input::ButtonInput;
use bevy::prelude::{KeyCode, Res, Single, Transform, With};
use bevy::time::Time;

pub(crate) fn puddle_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut paddle_transform: Single<&mut Transform, With<Paddle>>,
    time: Res<Time>,
) {
    if keyboard_input.any_pressed([KeyCode::ArrowLeft, KeyCode::ArrowRight]) {
        let mut direction: f32 = 0.;
        if keyboard_input.pressed(KeyCode::ArrowLeft) {
            direction = -1.;
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) {
            direction = 1.;
        }
        let new_position = paddle_transform.translation.x + direction * PADDLE_SPEED * time.delta_secs();
        let left_bound = WALL_LEFT + WALL_THICKNESS / 2. + PADDLE_SIZE.x / 2.;
        let right_bound = WALL_RIGHT - WALL_THICKNESS / 2. - PADDLE_SIZE.x / 2.;
        paddle_transform.translation.x = new_position.clamp(left_bound, right_bound);
    }
}

pub(crate) fn ball_movement(
    mut paddle_transform: Single<&mut Transform, With<Ball>>,
    time: Res<Time>,
) {
    
}
