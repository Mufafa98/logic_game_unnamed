use egui::Color32;

pub const IN_PORT_COLOR: Color32 = Color32::from_rgb(180, 220, 255);
pub const OUT_PORT_COLOR: Color32 = Color32::from_rgb(255, 200, 140);
pub const WIRE_OFF_COLOR: Color32 = Color32::from_gray(210);
pub const WIRE_ON_COLOR: Color32 = Color32::from_rgb(230, 70, 70);
pub const LOGIC_ON_COLOR: Color32 = Color32::from_rgb(230, 120, 120);

pub const PORT_RADIUS: f32 = 7.0;
pub const PORT_HIT_SIZE: f32 = PORT_RADIUS * 2.5;
pub const MIN_CURVE_OFFSET: f32 = 50.0;
pub const HIT_SAMPLES: usize = 24;

pub const NETWORK_MAX_ITER: usize = 64;
