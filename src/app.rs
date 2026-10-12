use eframe::egui;

use crate::consts::PORT_RADIUS;
use crate::logic_item::LogicItem;
use crate::ports::{self, Connection, PortGeom, PortId};
use crate::wire;

pub struct MyApp {
    items: Vec<LogicItem>,
    next_id: u64,
    connections: Vec<Connection>,
    pending: Option<PortId>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut app = Self {
            items: Vec::new(),
            next_id: 0,
            connections: Vec::new(),
            pending: None,
        };
        app.add_box(
            egui::pos2(60.0, 60.0),
            "A",
            egui::Color32::from_rgb(120, 170, 255),
            1,
            2,
        );
        app.add_box(
            egui::pos2(240.0, 140.0),
            "B",
            egui::Color32::from_rgb(255, 170, 120),
            2,
            1,
        );
        app.add_box(
            egui::pos2(120.0, 280.0),
            "C",
            egui::Color32::from_rgb(170, 255, 150),
            1,
            1,
        );
        app
    }
}

impl MyApp {
    fn add_box(
        &mut self,
        pos: egui::Pos2,
        label: &str,
        color: egui::Color32,
        no_in: u8,
        no_out: u8,
    ) {
        self.items.push(LogicItem {
            id: self.next_id,
            pos,
            size: egui::vec2(90.0, 60.0),
            label: label.to_owned(),
            color,
            no_in,
            no_out,
        });
        self.next_id += 1;
    }
}

#[derive(Default)]
struct PortInteraction {
    hovered: Option<PortId>,
    drag_started: Option<PortId>,
}

impl PortInteraction {
    fn gather(ui: &mut egui::Ui, ports: &[PortGeom]) -> Self {
        let mut out = Self::default();
        for p in ports {
            let r = ui.interact(
                p.rect,
                ui.id().with(("port", p.id.item_id, p.id.kind)),
                egui::Sense::click_and_drag(),
            );
            if r.hovered() {
                out.hovered = Some(p.id);
            }
            if r.drag_started() {
                out.drag_started = Some(p.id);
            }
        }
        out
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let ctx = ui.ctx().clone();
        ctx.request_repaint();

        let painter = ui.painter().clone();
        let canvas = ui.max_rect();

        let ports = ports::collect(&self.items);
        let box_responses = self.interact_boxes(ui, canvas);
        let port_interaction = PortInteraction::gather(ui, &ports);

        self.update_pending_wire(&ctx, &port_interaction);
        self.delete_wire_on_right_click(&ctx, &ports);

        self.draw_wires(&painter, &ports);
        self.draw_pending_wire(&painter, &ctx, &ports);
        self.draw_boxes(&painter, &ctx, &box_responses);
        self.draw_ports(&painter, &ctx, &port_interaction, &ports);

        self.draw_fps(&ctx);
    }
}

impl MyApp {
    fn interact_boxes(&mut self, ui: &mut egui::Ui, canvas: egui::Rect) -> Vec<egui::Response> {
        let mut responses = Vec::with_capacity(self.items.len());
        for item in &mut self.items {
            let response = ui.interact(
                item.rect(),
                ui.id().with(item.id),
                egui::Sense::click_and_drag(),
            );
            if response.dragged() {
                item.pos += response.drag_delta();
            }
            item.clamp_to(canvas);
            responses.push(response);
        }
        responses
    }

    fn update_pending_wire(&mut self, ctx: &egui::Context, port: &PortInteraction) {
        if self.pending.is_none() {
            self.pending = port.drag_started;
        }

        let Some(src) = self.pending else { return };
        if !ctx.input(|i| i.pointer.any_released()) {
            return;
        }
        self.pending = None;

        let Some(target) = port.hovered else { return };
        let Some(conn) = Connection::between(src, target) else {
            return;
        };

        let duplicate = self
            .connections
            .iter()
            .any(|c| c.from == conn.from && c.to == conn.to);
        if !duplicate {
            self.connections.push(conn);
        }
    }

    fn delete_wire_on_right_click(&mut self, ctx: &egui::Context, ports: &[PortGeom]) {
        if !ctx.input(|i| i.pointer.secondary_clicked()) {
            return;
        }
        let Some(mouse) = ctx.input(|i| i.pointer.interact_pos()) else {
            return;
        };

        if self.items.iter().any(|it| it.rect().contains(mouse)) {
            return;
        }

        let hit = self.connections.iter().position(|c| {
            let (Some(a), Some(b)) = (
                ports::center_of(ports, c.from),
                ports::center_of(ports, c.to),
            ) else {
                return false;
            };
            wire::hit(a, b, mouse, 6.0)
        });
        if let Some(i) = hit {
            self.connections.remove(i);
        }
    }
}

impl MyApp {
    fn draw_wires(&self, painter: &egui::Painter, ports: &[PortGeom]) {
        for conn in &self.connections {
            if let (Some(a), Some(b)) = (
                ports::center_of(ports, conn.from),
                ports::center_of(ports, conn.to),
            ) {
                wire::draw(painter, a, b, egui::Color32::from_gray(210));
            }
        }
    }

    fn draw_pending_wire(&self, painter: &egui::Painter, ctx: &egui::Context, ports: &[PortGeom]) {
        let Some(src) = self.pending else { return };
        let Some(a) = ports::center_of(ports, src) else {
            return;
        };
        let Some(mouse) = ctx.input(|i| i.pointer.hover_pos()) else {
            return;
        };
        wire::draw(painter, a, mouse, egui::Color32::from_gray(140));
    }

    fn draw_boxes(
        &self,
        painter: &egui::Painter,
        ctx: &egui::Context,
        responses: &[egui::Response],
    ) {
        for (item, response) in self.items.iter().zip(responses) {
            let rect = item.rect();
            let (fill, stroke) = box_style(item, response);

            painter.rect_filled(rect, 6.0, fill);
            painter.rect_stroke(rect, 6.0, stroke, egui::StrokeKind::Inside);
            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                &item.label,
                egui::FontId::proportional(20.0),
                egui::Color32::BLACK,
            );

            if response.dragged() {
                ctx.set_cursor_icon(egui::CursorIcon::Grabbing);
            } else if response.hovered() {
                ctx.set_cursor_icon(egui::CursorIcon::Grab);
            }
        }
    }

    fn draw_ports(
        &self,
        painter: &egui::Painter,
        ctx: &egui::Context,
        interaction: &PortInteraction,
        ports: &[PortGeom],
    ) {
        for p in ports {
            let is_hover = interaction.hovered == Some(p.id);
            let is_pending = self.pending == Some(p.id);

            let base = ports::base_color(p.id.kind);
            let fill = if is_pending {
                egui::Color32::WHITE
            } else if is_hover {
                base.gamma_multiply(1.4)
            } else {
                base
            };

            painter.circle_filled(p.center, PORT_RADIUS, fill);
            painter.circle_stroke(
                p.center,
                PORT_RADIUS,
                egui::Stroke::new(1.5, egui::Color32::from_gray(40)),
            );

            if is_hover {
                ctx.set_cursor_icon(egui::CursorIcon::Crosshair);
            }
        }
    }

    fn draw_fps(&self, ctx: &egui::Context) {
        let dt = ctx.input(|i| i.stable_dt).max(1e-6);
        let fps = 1.0 / dt;

        egui::Area::new(egui::Id::new("fps_overlay"))
            .anchor(egui::Align2::LEFT_TOP, egui::vec2(0.0, 0.0))
            .interactable(false)
            .show(ctx, |ui| {
                ui.label(
                    egui::RichText::new(format!("{fps:5.1} FPS"))
                        .monospace()
                        .color(egui::Color32::from_gray(180)),
                );
            });
    }
}

fn box_style(item: &LogicItem, response: &egui::Response) -> (egui::Color32, egui::Stroke) {
    if response.dragged() {
        (
            item.color.gamma_multiply(1.25),
            egui::Stroke::new(2.0, egui::Color32::WHITE),
        )
    } else if response.hovered() {
        (
            item.color.gamma_multiply(1.1),
            egui::Stroke::new(1.5, egui::Color32::from_gray(220)),
        )
    } else {
        (
            item.color,
            egui::Stroke::new(1.0, egui::Color32::from_gray(60)),
        )
    }
}
