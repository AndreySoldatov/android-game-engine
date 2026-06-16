use std::f32::consts::PI;

use egui::{Sense, Stroke, Vec2};

#[derive(Clone, Copy)]
pub struct RadialSnap {
    /// The number of steps for thumbstick angle snapping.
    ///
    /// Must be in 2..8 range
    pub steps: usize,
    /// The offset angle in radians for thumbstick angle snapping.
    ///
    /// Must be in 0..=step_size range,
    /// where step_size is (2.0 * PI) / steps.
    pub offset: f32,
}

/// A widget that allows to control a 2D vector value using a thumbstick.
///
/// The thumbstick is a circle with a smaller circle inside it. The user can drag the inner circle to control the value.
///
/// The value is normalized to the range [-1, 1] for both x and y axes.
pub struct Thumbstick<'a> {
    /// The value to be controlled by the thumbstick.
    value: &'a mut Vec2,
    /// The radius of the inner circle.
    inner_radius: f32,
    /// The radius of the outer circle.
    outer_radius: f32,
    /// The dead zone of the thumbstick. If the value is within the dead zone, it will be set to zero.
    ///
    /// This value must be in 0..=1.0 range.
    dead_zone: Option<f32>,
    /// If true, the value will be reset to zero when the user releases the thumbstick.
    reset_on_release: bool,
    /// If this value is some snaps the length of the thumbstick value to a closest step.
    ///
    /// This value must be in 2..=8 range
    length_steps: Option<usize>,
    /// If this value is some snaps the angle of the thumbstick value to a closest step.
    ///
    /// The step count must be in 2..=8 range.
    ///
    /// The offset angle must be in 0..=step_size range.
    radial_snap: Option<RadialSnap>,
}

impl<'a> Thumbstick<'a> {
    pub fn new(value: &'a mut Vec2) -> Self {
        Self {
            value,
            inner_radius: 40.0,
            outer_radius: 80.0,
            dead_zone: None,
            reset_on_release: true,
            length_steps: None,
            radial_snap: None,
        }
    }

    pub fn dead_zone(mut self, dead_zone: Option<f32>) -> Self {
        if let Some(dead_zone) = dead_zone {
            assert!(
                dead_zone >= 0.0 && dead_zone <= 1.0,
                "Deadzone must be in 0..=1.0 range"
            );
        }
        self.dead_zone = dead_zone;
        self
    }

    pub fn inner_radius(mut self, inner_radius: f32) -> Self {
        assert!(
            inner_radius > 0.0 && inner_radius <= self.outer_radius,
            "Inner radius of the thumbstick must be greater than zero and less or equal than outer radius"
        );
        self.inner_radius = inner_radius;
        self
    }

    pub fn outer_radius(mut self, outer_radius: f32) -> Self {
        assert!(
            outer_radius > 0.0 && outer_radius >= self.inner_radius,
            "Outer radius of the thumbstick must be greater than zero and greater or equal than inner radius"
        );
        self.outer_radius = outer_radius;
        self
    }

    pub fn reset_on_release(mut self, reset_on_release: bool) -> Self {
        self.reset_on_release = reset_on_release;
        self
    }

    pub fn length_steps(mut self, length_steps: Option<usize>) -> Self {
        if let Some(length_steps) = length_steps {
            assert!(
                length_steps >= 2 && length_steps <= 8,
                "Length steps of the thumbstick must be greater or equal than 2 and less or equal than 8"
            );
        }
        self.length_steps = length_steps;
        self
    }

    pub fn radial_snap(mut self, radial_snap: Option<RadialSnap>) -> Self {
        if let Some(radial_snap) = radial_snap {
            assert!(
                radial_snap.steps >= 2 && radial_snap.steps <= 8,
                "Radial snap steps of the thumbstick must be greater or equal than 2 and less or equal than 8"
            );
            assert!(
                radial_snap.offset <= ((2.0 * PI) / radial_snap.steps as f32)
                    && radial_snap.offset >= 0.0,
                "Offset must be less or equal step size and greater or equal than zero"
            )
        }
        self.radial_snap = radial_snap;
        self
    }
}

impl<'a> egui::Widget for Thumbstick<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        // Bounding box is the sum of the diameters of the inner and the outer radii
        let size = Vec2::splat(self.outer_radius * 2.0 + self.inner_radius * 2.0);

        let (rect, response) = ui.allocate_exact_size(size, Sense::drag());
        let center = rect.center();

        // Input handling
        if response.is_pointer_button_down_on() {
            let pos = response
                .interact_pointer_pos()
                .expect("At this point this must be Some()");

            let mut v = (pos - center) / self.outer_radius;
            let l = v.length();
            if l > 1.0 {
                v /= l;
            }

            if let Some(dz) = self.dead_zone {
                if v.length() < dz {
                    v = Vec2::ZERO;
                }
            }

            if let Some(length_steps) = self.length_steps {
                let l = v.length();

                if l > 0.0 {
                    let closest_step =
                        (l * (length_steps - 1) as f32).round() / (length_steps - 1) as f32;
                    let scale = closest_step / l;

                    v *= scale;
                } else {
                    v = Vec2::ZERO;
                }
            }

            if let Some(radial_snap) = self.radial_snap {
                let a = v.y.atan2(v.x);
                let step_size = (2.0 * PI) / radial_snap.steps as f32;
                let k = ((a - radial_snap.offset) / step_size).round();
                let newa = radial_snap.offset + k * step_size;

                let l = v.length();

                v.x = newa.cos() * l;
                v.y = newa.sin() * l;
            }

            *self.value = v;
        }

        // Set to zero when released
        if self.reset_on_release && response.drag_stopped() {
            *self.value = Vec2::ZERO;
        }

        // Painting
        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);

            let visuals = ui.style().interact(&response);

            painter.circle(
                center,
                self.outer_radius,
                visuals.bg_fill,
                Stroke::new(1.0, visuals.bg_stroke.color),
            );

            if let Some(length_steps) = self.length_steps {
                for i in 1..(length_steps - 1) {
                    let ratio = i as f32 / (length_steps - 1) as f32;
                    painter.circle_stroke(
                        center,
                        self.outer_radius * ratio,
                        Stroke::new(1.0, visuals.fg_stroke.color.gamma_multiply(0.5)),
                    );
                }
            }

            if let Some(radial_snap) = self.radial_snap {
                for i in 0..radial_snap.steps {
                    let a = (i as f32 / radial_snap.steps as f32) * 2.0 * PI + radial_snap.offset;
                    let x = a.cos() * self.outer_radius;
                    let y = a.sin() * self.outer_radius;
                    painter.line_segment(
                        [center, center + Vec2::new(x, y)],
                        Stroke::new(1.0, visuals.fg_stroke.color.gamma_multiply(0.5)),
                    );
                }
            }

            let knob_center = center + *self.value * self.outer_radius;
            painter.circle_filled(knob_center, self.inner_radius, visuals.fg_stroke.color);
        }

        response
    }
}
