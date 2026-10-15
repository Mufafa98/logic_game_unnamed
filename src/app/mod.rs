use eframe::{App, Frame};
use egui::*;

use crate::camera::Camera;
use crate::logic_item::{GateKind, ItemKind, LogicItem};
use crate::network::Network;
use crate::ports::{self, Connection, PortGeom, PortId, PortKind};

mod draw;
mod logic;

pub struct MyApp {
    items: Vec<LogicItem>,
    next_id: u64,
    connections: Vec<Connection>,
    pending: Option<PortId>,
    camera: Camera,
    palette_drag: Option<PaletteEntry>,
    selected: Option<u64>,
}

impl Default for MyApp {
    fn default() -> Self {
        let mut app = Self {
            items: Vec::new(),
            next_id: 0,
            connections: Vec::new(),
            pending: None,
            camera: Camera::default(),
            palette_drag: None,
            selected: None,
        };
        let blue = Color32::from_rgb(120, 170, 255);
        let purple = Color32::from_rgb(160, 130, 255);
        let green = Color32::from_rgb(120, 220, 140);
        let nand_type = ItemKind::Gate(GateKind::Nand);

        let a = app.add_box(pos2(60.0, 80.0), "I1", blue, 0, 1, ItemKind::Source);
        let b = app.add_box(pos2(60.0, 240.0), "I2", blue, 0, 1, ItemKind::Source);
        let g = app.add_box(pos2(280.0, 160.0), "NAND", purple, 2, 1, nand_type);
        let l = app.add_box(pos2(500.0, 160.0), "O", green, 1, 0, ItemKind::Led);

        app.connect(a, 0, g, 0);
        app.connect(b, 0, g, 1);
        app.connect(g, 0, l, 0);
        app
    }
}

impl MyApp {
    fn add_box(
        &mut self,
        pos: Pos2,
        label: &str,
        color: Color32,
        no_in: u8,
        no_out: u8,
        kind: ItemKind,
    ) -> u64 {
        let id = self.next_id;
        self.items.push(LogicItem {
            id,
            pos,
            size: vec2(90.0, 60.0),
            label: label.to_owned(),
            color,
            no_in,
            no_out,
            on: false,
            kind,
        });
        self.next_id += 1;
        id
    }

    fn connect(&mut self, src_id: u64, src_out: u8, dst_id: u64, dst_in: u8) {
        self.connections.push(Connection {
            from: PortId {
                item_id: src_id,
                kind: PortKind::Output(src_out),
            },
            to: PortId {
                item_id: dst_id,
                kind: PortKind::Input(dst_in),
            },
        });
    }
}

#[derive(Default)]
pub struct PortInteraction {
    hovered: Option<PortId>,
    drag_started: Option<PortId>,
}

impl PortInteraction {
    fn gather(ui: &mut Ui, ports: &[PortGeom], camera: Camera) -> Self {
        let mut out = Self::default();
        for p in ports {
            let screen_rect = camera.world_rect_to_screen(p.rect);
            let r = ui.interact(
                screen_rect,
                ui.id().with(("port", p.id.item_id, p.id.kind)),
                Sense::click_and_drag(),
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
#[derive(Clone, Copy)]
struct PaletteEntry {
    kind: ItemKind,
    label: &'static str,
    color: Color32,
    no_in: u8,
    no_out: u8,
}

impl PaletteEntry {
    fn all() -> [Self; 3] {
        [
            Self {
                kind: ItemKind::Source,
                label: "Source",
                color: Color32::from_rgb(120, 170, 255),
                no_in: 0,
                no_out: 1,
            },
            Self {
                kind: ItemKind::Led,
                label: "LED",
                color: Color32::from_rgb(120, 220, 140),
                no_in: 1,
                no_out: 0,
            },
            Self {
                kind: ItemKind::Gate(GateKind::Nand),
                label: "NAND",
                color: Color32::from_rgb(160, 130, 255),
                no_in: 2,
                no_out: 1,
            },
        ]
    }

    fn draw(&self, ui: &mut Ui) -> Response {
        let (rect, response) =
            ui.allocate_exact_size(vec2(ui.available_width(), 40.0), Sense::drag());

        let dragging = response.dragged();
        let hovered = response.hovered();

        let fill = if dragging {
            self.color.gamma_multiply(1.25)
        } else if hovered {
            self.color.gamma_multiply(1.15)
        } else {
            self.color
        };

        let painter = ui.painter();
        painter.rect_filled(rect, 5.0, fill);
        painter.rect_stroke(
            rect,
            5.0,
            Stroke::new(1.0, Color32::from_gray(60)),
            StrokeKind::Inside,
        );
        painter.text(
            rect.center(),
            Align2::CENTER_CENTER,
            self.label,
            FontId::proportional(14.0),
            Color32::BLACK,
        );

        if dragging {
            ui.ctx().set_cursor_icon(CursorIcon::Grabbing);
        } else if hovered {
            ui.ctx().set_cursor_icon(CursorIcon::Grab);
        }

        response
    }
}
impl App for MyApp {
    fn ui(&mut self, ui: &mut Ui, _frame: &mut Frame) {
        let ctx = ui.ctx().clone();
        ctx.request_repaint();

        let full = ui.max_rect();
        let palette_w = 180.0;

        let palette_rect = Rect::from_min_size(
            pos2(full.max.x - palette_w, full.min.y),
            vec2(palette_w, full.height()),
        );

        let canvas = Rect::from_min_max(full.min, pos2(full.max.x - palette_w, full.max.y));

        ui.allocate_rect(full, Sense::hover());

        let mut started_drag: Option<PaletteEntry> = None;
        Area::new(Id::new("palette"))
            .fixed_pos(palette_rect.min)
            .order(Order::Foreground)
            .show(&ctx, |ui| {
                ui.set_max_width(palette_w - 12.0);

                for entry in PaletteEntry::all() {
                    if entry.draw(ui).drag_started() {
                        started_drag = Some(entry);
                    }
                    ui.add_space(6.0);
                }

                ui.add_space(8.0);
                ui.separator();
                ui.add_space(6.0);

                ui.label(RichText::new("Selected").strong());
                ui.add_space(4.0);

                match self
                    .selected
                    .and_then(|id| self.items.iter_mut().find(|it| it.id == id))
                {
                    Some(item) => {
                        ui.label("Label");
                        let resp = ui.text_edit_singleline(&mut item.label);

                        if resp.lost_focus() {
                            resp.request_focus();
                        }
                    }
                    None => {
                        ui.label(RichText::new("(click a block to edit)").weak());
                    }
                }
            });
        if let Some(entry) = started_drag {
            self.palette_drag = Some(entry);
        }

        let painter = ui.painter_at(canvas);
        let camera = self.camera;

        let scroll = ctx.input(|i| i.smooth_scroll_delta.y);
        if scroll != 0.0 && self.palette_drag.is_none() {
            if let Some(cursor) = ctx.input(|i| i.pointer.hover_pos()) {
                self.camera.zoom_at(cursor, (scroll * 0.0015).exp());
            }
        }

        let bg = ui.interact(canvas, Id::new("canvas_bg"), Sense::drag());
        if bg.dragged() && self.palette_drag.is_none() {
            self.camera.pan += bg.drag_delta();
            ctx.set_cursor_icon(CursorIcon::Grabbing);
        }

        let ports = ports::collect(&self.items);
        let box_responses = self.interact_boxes(ui, camera);
        let port_interaction = PortInteraction::gather(ui, &ports, camera);

        self.toggle_boxes_on_click(&port_interaction, &box_responses);
        self.update_pending_wire(&ctx, &port_interaction);
        self.delete_wire_on_right_click(&ctx, &ports, camera);
        self.handle_box_deletion(&box_responses, palette_rect, &ctx);

        let network = Network::evaluate(&self.items, &self.connections);

        self.draw_wires(&painter, &ports, &network, camera);
        self.draw_pending_wire(&painter, &ctx, &ports, camera);
        self.draw_boxes(&painter, &ctx, &box_responses, &network, camera);
        self.draw_ports(&painter, &ctx, &port_interaction, &ports, camera);

        if self.palette_drag.is_some() && ctx.input(|i| i.pointer.any_released()) {
            let entry = self.palette_drag.take().unwrap();
            if let Some(mouse) = ctx.input(|i| i.pointer.interact_pos()) {
                if canvas.contains(mouse) {
                    let size = vec2(90.0, 60.0);
                    let world = camera.screen_to_world(mouse);
                    let pos = world - size * 0.5;
                    self.add_box(
                        pos,
                        entry.label,
                        entry.color,
                        entry.no_in,
                        entry.no_out,
                        entry.kind,
                    );
                }
            }
        }
        self.draw_palette_ghost(&painter, &ctx, camera);
        self.draw_trash_box(box_responses, &ctx, palette_rect);

        self.draw_fps(&ctx);
    }
}
