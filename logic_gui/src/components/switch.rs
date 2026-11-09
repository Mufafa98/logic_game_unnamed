use bevy::{picking::hover::Hovered, prelude::*, ui_widgets::Button};

use crate::components::Led;

#[derive(Bundle)]
pub struct Switch {
    pub led: Led,
    pub led_state: SwitchState,
    pub button: Button,
    pub hovered: Hovered,
}

impl Switch {
    pub fn new(id: usize, x: f32, y: f32) -> Self {
        Self {
            led: Led::new(id, x, y),
            led_state: SwitchState(false),
            button: Button,
            hovered: Hovered::default(),
        }
    }
}

#[derive(Component, Default)]
pub struct SwitchState(pub bool);

impl SwitchState {
    pub fn toggle(&mut self) {
        self.0 = !self.0;
    }
}
