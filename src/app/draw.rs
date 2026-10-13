use crate::app::{MyApp, PortInteraction};
use crate::logic_item::{ItemKind, LogicItem};
use egui::*;

use crate::camera::Camera;
use crate::consts::{LOGIC_ON_COLOR, PORT_RADIUS, WIRE_OFF_COLOR, WIRE_ON_COLOR};
use crate::network::Network;
use crate::ports::{self, PortGeom, PortKind};
use crate::wire;

impl MyApp {
    pub fn draw_wires(
        &self,
        painter: &Painter,
        ports: &[PortGeom],
        network: &Network,
        camera: Camera,
    ) {
        let width = (2.5 * camera.zoom).max(1.0);
        for conn in &self.connections {
            let (Some(a), Some(b)) = (
                ports::center_of(ports, conn.from),
                ports::center_of(ports, conn.to),
            ) else {
                continue;
            };

            let powered = match conn.from.kind {
                PortKind::Output(idx) => network.output(conn.from.item_id, idx),
                PortKind::Input(_) => false,
            };
            let color = if powered {
                WIRE_ON_COLOR
            } else {
                WIRE_OFF_COLOR
            };
            wire::draw(
                painter,
                camera.world_to_screen(a),
                camera.world_to_screen(b),
                color,
                width,
            );
        }
    }

    pub fn draw_pending_wire(
        &self,
        painter: &Painter,
        ctx: &Context,
        ports: &[PortGeom],
        camera: Camera,
    ) {
        let Some(src) = self.pending else { return };
        let Some(a) = ports::center_of(ports, src) else {
            return;
        };
        let Some(mouse) = ctx.input(|i| i.pointer.hover_pos()) else {
            return;
        };
        let width = (2.5 * camera.zoom).max(1.0);
        wire::draw(
            painter,
            camera.world_to_screen(a),
            mouse,
            Color32::from_gray(140),
            width,
        );
    }

    pub fn draw_boxes(
        &self,
        painter: &Painter,
        ctx: &Context,
        responses: &[Response],
        network: &Network,
        camera: Camera,
    ) {
        for (item, response) in self.items.iter().zip(responses) {
            let rect = camera.world_rect_to_screen(item.rect());
            let (fill, stroke) = box_style(item, response, network, camera.zoom);

            painter.rect_filled(rect, 6.0 * camera.zoom, fill);
            painter.rect_stroke(rect, 6.0 * camera.zoom, stroke, StrokeKind::Inside);
            painter.text(
                rect.center(),
                Align2::CENTER_CENTER,
                &item.label,
                FontId::proportional(20.0 * camera.zoom),
                Color32::BLACK,
            );

            if response.dragged() {
                ctx.set_cursor_icon(CursorIcon::Grabbing);
            } else if response.hovered() {
                ctx.set_cursor_icon(CursorIcon::Grab);
            }
        }
    }

    pub fn draw_ports(
        &self,
        painter: &Painter,
        ctx: &Context,
        interaction: &PortInteraction,
        ports: &[PortGeom],
        camera: Camera,
    ) {
        let radius = PORT_RADIUS * camera.zoom;
        let stroke = Stroke::new((1.5 * camera.zoom).max(0.5), Color32::from_gray(40));
        for p in ports {
            let center = camera.world_to_screen(p.center);
            let is_hover = interaction.hovered == Some(p.id);
            let is_pending = self.pending == Some(p.id);

            let base = ports::base_color(p.id.kind);
            let fill = if is_pending {
                Color32::WHITE
            } else if is_hover {
                base.gamma_multiply(1.4)
            } else {
                base
            };

            painter.circle_filled(center, radius, fill);
            painter.circle_stroke(center, radius, stroke);

            if is_hover {
                ctx.set_cursor_icon(CursorIcon::Crosshair);
            }
        }
    }

    pub fn draw_fps(&self, ctx: &Context) {
        let dt = ctx.input(|i| i.stable_dt).max(1e-6);
        let fps = 1.0 / dt;

        Area::new(Id::new("fps_overlay"))
            .anchor(Align2::LEFT_TOP, vec2(0.0, 0.0))
            .interactable(false)
            .show(ctx, |ui| {
                ui.label(
                    RichText::new(format!("{fps:5.1} FPS"))
                        .monospace()
                        .color(Color32::from_gray(180)),
                );
            });
    }

    pub fn draw_palette_ghost(&self, painter: &Painter, ctx: &Context, camera: Camera) {
        let Some(entry) = self.palette_drag else {
            return;
        };
        let Some(mouse) = ctx.input(|i| i.pointer.hover_pos()) else {
            return;
        };

        let box_size = vec2(90.0, 60.0) * camera.zoom;
        let rect = Rect::from_center_size(mouse, box_size);

        let [r, g, b, _] = entry.color.to_array();
        let fill = Color32::from_rgba_unmultiplied(r, g, b, 160);

        painter.rect_filled(rect, 6.0 * camera.zoom, fill);
        painter.rect_stroke(
            rect,
            6.0 * camera.zoom,
            Stroke::new(1.5, Color32::WHITE),
            StrokeKind::Inside,
        );
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            entry.label,
            FontId::proportional(18.0 * camera.zoom),
            Color32::from_gray(40),
        );
    }

    pub fn draw_trash_box(&self, box_responses: Vec<Response>, ctx: &Context, palette_rect: Rect) {
        let dragging_box = box_responses.iter().any(|r| r.dragged());
        let over_palette = ctx
            .input(|i| i.pointer.latest_pos())
            .is_some_and(|p| palette_rect.contains(p));
        if dragging_box && over_palette {
            Area::new(Id::new("trash_hint"))
                .order(Order::Tooltip)
                .interactable(false)
                .show(&ctx, |ui| {
                    let painter = ui.painter();
                    painter.rect_filled(
                        palette_rect,
                        0.0,
                        Color32::from_rgba_unmultiplied(200, 60, 60, 70),
                    );
                    painter.rect_stroke(
                        palette_rect,
                        0.0,
                        Stroke::new(3.0, Color32::from_rgb(230, 90, 90)),
                        StrokeKind::Inside,
                    );
                    painter.text(
                        palette_rect.center(),
                        Align2::CENTER_CENTER,
                        "Drop to delete",
                        FontId::proportional(18.0),
                        Color32::WHITE,
                    );
                });
        }
    }
}

fn box_style(
    item: &LogicItem,
    response: &Response,
    network: &Network,
    zoom: f32,
) -> (Color32, Stroke) {
    let lit = match item.kind {
        ItemKind::Source => item.on,
        ItemKind::Led => network.input(item.id, 0),
        ItemKind::Gate(_) => false,
    };
    let base = if lit { LOGIC_ON_COLOR } else { item.color };

    if response.dragged() {
        (
            base.gamma_multiply(1.25),
            Stroke::new(2.0 * zoom, Color32::WHITE),
        )
    } else if response.hovered() {
        (
            base.gamma_multiply(1.1),
            Stroke::new(1.5 * zoom, Color32::from_gray(220)),
        )
    } else {
        (base, Stroke::new(1.0 * zoom, Color32::from_gray(60)))
    }
}
