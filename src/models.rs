use crate::constants::{WALL_BOTTOM, WALL_LEFT, WALL_RIGHT, WALL_THICKNESS, WALL_TOP};
use bevy::math::Vec2;

pub(crate) enum WallLocation {
    Top,
    Right,
    Bottom,
    Left,
}

impl WallLocation {
    /// Location of the *center* of the wall, used in `transform.translation()`
    pub(crate) fn position(&self) -> Vec2 {
        match self {
            WallLocation::Left => Vec2::new(WALL_LEFT, 0.),
            WallLocation::Right => Vec2::new(WALL_RIGHT, 0.),
            WallLocation::Bottom => Vec2::new(0., WALL_BOTTOM),
            WallLocation::Top => Vec2::new(0., WALL_TOP),
        }
    }

    pub(crate) fn size(&self) -> Vec2 {
        let playground_height = WALL_TOP - WALL_BOTTOM;
        let playground_width = WALL_RIGHT - WALL_LEFT;

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec2::new(WALL_THICKNESS, playground_height + WALL_THICKNESS)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec2::new(playground_width + WALL_THICKNESS, WALL_THICKNESS)
            }
        }
    }
}
