use egui::{Color32, Pos2, Rect, Vec2};

use crate::ports::PortKind;

#[derive(Clone, Copy)]
pub enum GateKind {
    Nand,
}

#[derive(Clone, Copy)]
pub enum ItemKind {
    Source,
    Led,
    Gate(GateKind),
    // TODO Compound(some subgraph id)
}

impl GateKind {
    pub fn eval(self, inputs: &[bool]) -> Vec<bool> {
        match self {
            GateKind::Nand => vec![!inputs.iter().all(|&b| b)],
        }
    }
}

pub struct LogicItem {
    pub id: u64,
    pub pos: Pos2,
    pub size: Vec2,
    pub label: String,
    pub color: Color32,

    pub no_in: u8,
    pub no_out: u8,

    pub kind: ItemKind,
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

    pub fn toggle(&mut self) {
        self.on = !self.on;
    }

    pub fn is_source(&self) -> bool {
        matches!(self.kind, ItemKind::Source)
    }

    pub fn eval(&self, inputs: &[bool]) -> Vec<bool> {
        match self.kind {
            ItemKind::Source => vec![self.on],
            ItemKind::Led => Vec::new(),
            ItemKind::Gate(gate_kind) => gate_kind.eval(inputs),
        }
    }
}
