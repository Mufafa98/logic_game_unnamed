use crate::app::{MyApp, PortInteraction};

use egui::*;

use crate::camera::Camera;
use crate::ports::{self, Connection, PortGeom};
use crate::wire;
impl MyApp {
    pub fn handle_box_deletion(&mut self, responses: &[Response], palette: Rect, ctx: &Context) {
        let Some(mouse) = ctx.input(|i| i.pointer.latest_pos()) else {
            return;
        };
        if !palette.contains(mouse) {
            return;
        }

        let id = self
            .graph
            .items
            .iter()
            .zip(responses)
            .find(|(_, r)| r.drag_stopped())
            .map(|(it, _)| it.id);

        let Some(id) = id else { return };

        self.graph.items.retain(|it| it.id != id);
        self.graph
            .connections
            .retain(|c| c.from.item_id != id && c.to.item_id != id);
    }

    pub fn toggle_boxes_on_click(&mut self, port: &PortInteraction, responses: &[Response]) {
        if port.hovered.is_some() {
            return;
        }
        for (item, response) in self.graph.items.iter_mut().zip(responses) {
            if !response.clicked() {
                continue;
            }
            self.selected = Some(item.id);
            if item.is_source() {
                item.toggle();
            }
        }
    }

    pub fn interact_boxes(&mut self, ui: &mut Ui, camera: Camera) -> Vec<Response> {
        let mut responses = Vec::with_capacity(self.graph.items.len());
        for item in &mut self.graph.items {
            let screen_rect = camera.world_rect_to_screen(item.rect());
            let response = ui.interact(screen_rect, ui.id().with(item.id), Sense::click_and_drag());

            if response.dragged() {
                item.pos += response.drag_delta() / camera.zoom;
            }
            responses.push(response);
        }
        responses
    }

    pub fn update_pending_wire(&mut self, ctx: &Context, port: &PortInteraction) {
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
            .graph
            .connections
            .iter()
            .any(|c| c.from == conn.from && c.to == conn.to);
        if !duplicate {
            self.graph.connections.push(conn);
        }
    }

    pub fn delete_wire_on_right_click(
        &mut self,
        ctx: &Context,
        ports: &[PortGeom],
        camera: Camera,
    ) {
        if !ctx.input(|i| i.pointer.secondary_clicked()) {
            return;
        }
        let Some(mouse) = ctx.input(|i| i.pointer.interact_pos()) else {
            return;
        };

        let mouse_world = camera.screen_to_world(mouse);
        if self
            .graph
            .items
            .iter()
            .any(|it| it.rect().contains(mouse_world))
        {
            return;
        }

        let hit = self.graph.connections.iter().position(|c| {
            let (Some(a), Some(b)) = (
                ports::center_of(ports, c.from),
                ports::center_of(ports, c.to),
            ) else {
                return false;
            };
            wire::hit(
                camera.world_to_screen(a),
                camera.world_to_screen(b),
                mouse,
                6.0,
            )
        });
        if let Some(i) = hit {
            self.graph.connections.remove(i);
        }
    }
}
