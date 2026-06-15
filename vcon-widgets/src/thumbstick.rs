use egui::{Sense, Stroke, Vec2};

pub struct Thumbstick<'a> {
    value: &'a mut Vec2,
    inner_radius: f32,
    outer_radius: f32,
    dead_zone: Option<f32>,
}

impl<'a> Thumbstick<'a> {
    pub fn new(value: &'a mut Vec2) -> Self {
        Self {
            value,
            inner_radius: 40.0,
            outer_radius: 80.0,
            dead_zone: None,
        }
    }

    pub fn dead_zone(mut self, dead_zone: Option<f32>) -> Self {
        self.dead_zone = dead_zone;
        self
    }

    pub fn inner_radius(mut self, inner_radius: f32) -> Self {
        self.inner_radius = inner_radius;
        self
    }

    pub fn outer_radius(mut self, outer_radius: f32) -> Self {
        self.outer_radius = outer_radius;
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
        if response.dragged() && response.is_pointer_button_down_on() {
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

            *self.value = v;
        }

        // Set to zero when released
        if response.drag_stopped() {
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

            let knob_center = center + *self.value * self.outer_radius;
            painter.circle_filled(knob_center, self.inner_radius, visuals.fg_stroke.color);
        }

        response
    }
}
