use crate::models::Collision;
use bevy::prelude::{Entity, Event};

#[derive(Event)]
pub(crate) struct CollisionEvent {
    pub entity: Entity,
    pub collision: Collision,
}

impl CollisionEvent {
    pub(crate) fn new(entity: Entity, collision: Collision) -> Self {
        CollisionEvent { entity, collision }
    }
}