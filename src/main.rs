use eframe::egui;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Logic Game Unnamed",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::default()))),
    )
}

struct LogicItem {
    id: u64,
    pos: egui::Pos2,
    size: egui::Vec2,
    label: String,
    color: egui::Color32,
}

struct MyApp {
    items: Vec<LogicItem>,
    next_id: u64,
}

impl MyApp {
    fn add_box(&mut self, pos: egui::Pos2, label: &str, color: egui::Color32) {
        self.items.push(LogicItem {
            id: self.next_id,
            pos,
            size: egui::vec2(90.0, 60.0),
            label: label.to_owned(),
            color,
        });
        self.next_id += 1;
    }
}

impl Default for MyApp {
    fn default() -> Self {
        let mut app = Self {
            items: Vec::new(),
            next_id: 0,
        };
        app.add_box(
            egui::pos2(60.0, 60.0),
            "A",
            egui::Color32::from_rgb(120, 170, 255),
        );
        app.add_box(
            egui::pos2(240.0, 140.0),
            "B",
            egui::Color32::from_rgb(255, 170, 120),
        );
        app.add_box(
            egui::pos2(120.0, 280.0),
            "C",
            egui::Color32::from_rgb(170, 255, 150),
        );
        app
    }
}

impl eframe::App for MyApp {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        let painter = ui.painter();
        let canvas = ui.max_rect();
        let ctx = ui.ctx();
        ctx.request_repaint();

        for item in self.items.iter_mut() {
            let rect = egui::Rect::from_min_size(item.pos, item.size);

            let response = ui.interact(rect, ui.id().with(item.id), egui::Sense::click_and_drag());

            if response.dragged() {
                item.pos += response.drag_delta();
            }

            let rect = egui::Rect::from_min_size(item.pos, item.size);
            if !canvas.contains_rect(rect) {
                item.pos.x = item.pos.x.clamp(canvas.min.x, canvas.max.x - item.size.x);
                item.pos.y = item.pos.y.clamp(canvas.min.y, canvas.max.y - item.size.y);
            }
            let rect = egui::Rect::from_min_size(item.pos, item.size);

            let (fill, stroke) = if response.dragged() {
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
            };

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
