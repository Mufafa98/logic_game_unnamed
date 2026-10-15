use egui::{Color32, Pos2, Rect, Vec2};
use serde::{Deserialize, Serialize};

use crate::{
    consts::{IN_PORT_COLOR, OUT_PORT_COLOR, PORT_HIT_SIZE},
    logic_item::LogicItem,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub enum PortKind {
    Input(u8),
    Output(u8),
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Serialize, Deserialize)]
pub struct PortId {
    pub item_id: u64,
    pub kind: PortKind,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct Connection {
    pub from: PortId,
    pub to: PortId,
}

impl Connection {
    pub fn between(a: PortId, b: PortId) -> Option<Self> {
        if a.item_id == b.item_id {
            return None;
        }
        match (a.kind, b.kind) {
            (PortKind::Output(_), PortKind::Input(_)) => Some(Self { from: a, to: b }),
            (PortKind::Input(_), PortKind::Output(_)) => Some(Self { from: b, to: a }),
            _ => None,
        }
    }
}

#[derive(Clone, Copy)]
pub struct PortGeom {
    pub id: PortId,
    pub center: Pos2,
    pub rect: Rect,
}

/// Rebuild the port geometry for every item.
pub fn collect(items: &[LogicItem]) -> Vec<PortGeom> {
    let mut ports = Vec::with_capacity(items.len() * 2);
    for item in items {
        for i in 0..item.no_in {
            ports.push(geom_for(item, PortKind::Input(i)));
        }
        for i in 0..item.no_out {
            ports.push(geom_for(item, PortKind::Output(i)));
        }
    }
    ports
}

fn geom_for(item: &LogicItem, kind: PortKind) -> PortGeom {
    let center = item.port_pos(kind);
    PortGeom {
        id: PortId {
            item_id: item.id,
            kind,
        },
        center,
        rect: Rect::from_center_size(center, Vec2::splat(PORT_HIT_SIZE)),
    }
}

pub fn center_of(ports: &[PortGeom], id: PortId) -> Option<Pos2> {
    ports.iter().find(|p| p.id == id).map(|p| p.center)
}

pub fn base_color(kind: PortKind) -> Color32 {
    match kind {
        PortKind::Input(_) => IN_PORT_COLOR,
        PortKind::Output(_) => OUT_PORT_COLOR,
    }
}
