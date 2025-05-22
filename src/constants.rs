use bevy::color::Color;
use bevy::math::Vec2;
use bevy::prelude::ClearColor;

pub(crate) const BACKGROUND_CLEAR_COLOR: ClearColor = ClearColor(Color::oklch(0.2267, 0.0587, 273.18));
pub(crate) const WALL_TOP: f32 = 300.;
pub(crate) const WALL_RIGHT: f32 = 450.;
pub(crate) const WALL_BOTTOM: f32 = -300.;
pub(crate) const WALL_LEFT: f32 = -450.;
pub(crate) const WALL_THICKNESS: f32 = 15.;
pub(crate) const WALL_COLOR: Color = Color::oklch(0.7, 0.0419, 313.41);
pub(crate) const PADDLE_Y: f32 = WALL_BOTTOM + 60.;
pub(crate) const PADDLE_SIZE: Vec2 = Vec2::new(120., 20.);
pub(crate) const PADDLE_COLOR: Color = Color::oklch(0.7, 0.1, 115.);
pub(crate) const PADDLE_SPEED: f32 = 750.;
