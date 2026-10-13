use egui::{Pos2, Rect, Vec2};

#[derive(Clone, Copy)]
pub struct Camera {
    pub pan: Vec2,
    pub zoom: f32,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            pan: Vec2::ZERO,
            zoom: 1.0,
        }
    }
}

impl Camera {
    pub fn world_to_screen(&self, world: Pos2) -> Pos2 {
        egui::pos2(
            world.x * self.zoom + self.pan.x,
            world.y * self.zoom + self.pan.y,
        )
    }

    pub fn screen_to_world(&self, screen: Pos2) -> Pos2 {
        egui::pos2(
            (screen.x - self.pan.x) / self.zoom,
            (screen.y - self.pan.y) / self.zoom,
        )
    }

    pub fn world_rect_to_screen(&self, rect: Rect) -> Rect {
        Rect::from_min_max(
            self.world_to_screen(rect.min),
            self.world_to_screen(rect.max),
        )
    }

    pub fn zoom_at(&mut self, screen_pos: Pos2, factor: f32) {
        let world = self.screen_to_world(screen_pos);
        self.zoom = (self.zoom * factor).clamp(0.15, 6.0);
        self.pan = egui::vec2(
            screen_pos.x - world.x * self.zoom,
            screen_pos.y - world.y * self.zoom,
        );
    }
}
