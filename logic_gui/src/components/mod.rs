mod led;
mod switch;

use bevy::{color::Color, ecs::component::Component};
#[derive(Component)]
pub struct LedId(pub usize);

pub const LED_ON: Color = Color::srgb_u8(255, 123, 123);
pub const LED_OFF: Color = Color::srgb_u8(40, 5, 5);
pub const LED_WIDTH: f32 = 64.0;
pub const LED_HEIGHT: f32 = 64.0;

pub use led::Led;
pub use switch::{Switch, SwitchState};
