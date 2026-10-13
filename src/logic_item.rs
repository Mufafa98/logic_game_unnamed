use egui::{Color32, Pos2, Rect, Vec2};

use crate::ports::PortKind;

pub struct LogicItem {
    pub id: u64,
    pub pos: Pos2,
    pub size: Vec2,
    pub label: String,
    pub color: Color32,
    pub no_in: u8,
    pub no_out: u8,

    pub clickable: bool,
    pub on: bool,
}

impl LogicItem {
    pub fn rect(&self) -> Rect {
        Rect::from_min_size(self.pos, self.size)
    }

    pub fn port_pos(&self, kind: PortKind) -> Pos2 {
        let (idx, count, x) = match kind {
            PortKind::Input(i) => (i, self.no_in, self.pos.x),
            PortKind::Output(i) => (i, self.no_out, self.pos.x + self.size.x),
        };

        let t = (idx as f32 + 1.0) / (count as f32 + 1.0);
        egui::pos2(x, self.pos.y + self.size.y * t)
    }

    pub fn clamp_to(&mut self, canvas: Rect) {
        if canvas.contains_rect(self.rect()) {
            return;
        }
        self.pos.x = self.pos.x.clamp(canvas.min.x, canvas.max.x - self.size.x);
        self.pos.y = self.pos.y.clamp(canvas.min.y, canvas.max.y - self.size.y);
    }

    pub fn toggle(&mut self) {
        self.on = !self.on;
    }
}
