use egui::{Color32, Painter, Pos2, Stroke};

use crate::consts::{HIT_SAMPLES, MIN_CURVE_OFFSET};

/// A cubic bezier with four control points: start, two handles, end.
#[derive(Clone, Copy)]
struct Bezier {
    start: Pos2,
    handle_a: Pos2,
    handle_b: Pos2,
    end: Pos2,
}

impl Bezier {
    /// Build a wire from `a` to `b`. Handles push horizontally so the wire
    /// leaves and enters the ports sideways.
    fn wire(a: Pos2, b: Pos2) -> Self {
        let offset = horizontal_offset(a, b);
        Self {
            start: a,
            handle_a: egui::pos2(a.x + offset, a.y),
            handle_b: egui::pos2(b.x - offset, b.y),
            end: b,
        }
    }

    /// Point on the curve at parameter `t` in `[0, 1]`.
    ///
    /// Standard cubic bezier formula: B(t) = sum C(3, i) * (1-t)^(3-i) * t^i * P_i
    fn point_at(&self, t: f32) -> Pos2 {
        let u = 1.0 - t;
        let (u2, t2) = (u * u, t * t);
        let (u3, t3) = (u2 * u, t2 * t);

        let x = u3 * self.start.x
            + 3.0 * u2 * t * self.handle_a.x
            + 3.0 * u * t2 * self.handle_b.x
            + t3 * self.end.x;

        let y = u3 * self.start.y
            + 3.0 * u2 * t * self.handle_a.y
            + 3.0 * u * t2 * self.handle_b.y
            + t3 * self.end.y;

        egui::pos2(x, y)
    }

    fn control_points(&self) -> [Pos2; 4] {
        [self.start, self.handle_a, self.handle_b, self.end]
    }
}

/// Half the horizontal distance between the endpoints, with a floor so
/// short wires still curve visibly.
fn horizontal_offset(a: Pos2, b: Pos2) -> f32 {
    (b.x - a.x).abs().max(MIN_CURVE_OFFSET) * 0.5
}
pub fn draw(painter: &Painter, a: Pos2, b: Pos2, color: Color32, width: f32) {
    let curve = Bezier::wire(a, b);
    painter.add(egui::epaint::CubicBezierShape::from_points_stroke(
        curve.control_points(),
        false,
        Color32::TRANSPARENT,
        Stroke::new(width, color),
    ));
}

/// Approximate hit-test: step along the curve and check the distance to `p`.
pub fn hit(a: Pos2, b: Pos2, p: Pos2, tolerance: f32) -> bool {
    let curve = Bezier::wire(a, b);
    (0..=HIT_SAMPLES).any(|i| {
        let t = i as f32 / HIT_SAMPLES as f32;
        curve.point_at(t).distance(p) <= tolerance
    })
}
