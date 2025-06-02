use crate::components::{Ball, Collider, Paddle, Velocity, Wall};
use crate::constants::{BALL_DIAMETER, PADDLE_SIZE, PADDLE_SPEED, WALL_LEFT, WALL_RIGHT, WALL_THICKNESS};
use crate::event::CollisionEvent;
use crate::models::Collision;
use bevy::input::ButtonInput;
use bevy::math::bounding::{Aabb2d, BoundingCircle, BoundingVolume, IntersectsVolume};
use bevy::math::ops::abs;
use bevy::prelude::{Entity, EventReader, EventWriter, KeyCode, Query, Res, Single, Transform, With};
use bevy::time::Time;

pub(crate) fn move_paddle(
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

pub(crate) fn apply_velocity(
    mut query: Query<(&mut Transform, &Velocity)>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

pub(crate) fn detect_ball_collision(
    ball_query: Single<(&Transform, &mut Velocity), With<Ball>>,
    colliders_query: Query<(Entity, &Transform), With<Collider>>,
    mut collision_events_writer: EventWriter<CollisionEvent>,
) {
    let (ball_transform, mut ball_velocity) = ball_query.into_inner();
    for (entity, collider_transform) in colliders_query {
        let collision = detect_collision(
            BoundingCircle::new(ball_transform.translation.truncate(), BALL_DIAMETER / 2.),
            Aabb2d::new(collider_transform.translation.truncate(), collider_transform.scale.truncate() / 2.),
        );
        if let Some(collision) = collision {
            collision_events_writer.write(CollisionEvent::new(entity, collision));
        }
    }
}

fn detect_collision(ball: BoundingCircle, bounding_box: Aabb2d) -> Option<Collision> {
    if !ball.intersects(&bounding_box) {
        return None;
    }

    let closest_point = bounding_box.closest_point(ball.center());
    let offset = ball.center() - closest_point;
    let collision = if offset.x.abs() > offset.y.abs() {
        if (offset.x < 0.) {
            Collision::LEFT
        } else {
            Collision::RIGHT
        }
    } else {
        if (offset.y > 0.) {
            Collision::TOP
        } else {
            Collision::BOTTOM
        }
    };
    Some(collision)
}

pub(crate) fn handle_ball_bounce(
    ball_query: Single<&mut Velocity, With<Ball>>,
    mut collision_event: EventReader<CollisionEvent>,
) {
    let mut velocity = ball_query.into_inner();
    for event in collision_event.read() {
        match event.collision {
            Collision::TOP => velocity.y = abs(velocity.y),
            Collision::RIGHT => velocity.x = abs(velocity.x),
            Collision::BOTTOM => velocity.y = -abs(velocity.y),
            Collision::LEFT => velocity.x = -abs(velocity.x),
        }
    }
}
