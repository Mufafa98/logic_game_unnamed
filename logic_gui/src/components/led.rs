use bevy::prelude::*;

use crate::components::{LED_HEIGHT, LED_OFF, LED_WIDTH};

use super::LedId;

#[derive(Bundle)]
pub struct Led {
    pub id: LedId,
    pub node: Node,
    pub background: BackgroundColor,
}

impl Led {
    pub fn new(id: usize, x: f32, y: f32) -> Self {
        Self {
            id: LedId(id),
            node: Node {
                left: px(x),
                top: px(y),
                width: px(LED_WIDTH),
                height: px(LED_HEIGHT),
                ..default()
            },
            background: BackgroundColor(LED_OFF),
        }
    }
}
