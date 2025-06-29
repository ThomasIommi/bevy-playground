use bevy::math::Vec2;
use bevy::prelude::{Component, Deref, DerefMut};

#[derive(Component)]
pub(crate) struct Paddle;

#[derive(Component)]
pub(crate) struct Wall;

#[derive(Component)]
pub(crate) struct Ball;

#[derive(Component)]
pub(crate) struct Brick;

#[derive(Component, Deref, DerefMut)]
pub(crate) struct Velocity(pub(crate) Vec2);

#[derive(Component)]
pub(crate) struct Collider;