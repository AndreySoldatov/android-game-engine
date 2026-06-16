use std::f32::consts::PI;

use egui::{Pos2, Sense, Vec2, epaint::PathShape};

#[derive(Clone, Copy)]
pub struct DpadState {
    pub top: bool,
    pub bottom: bool,
    pub left: bool,
    pub right: bool,
}

impl DpadState {
    pub const EMPTY: Self = Self {
        top: false,
        bottom: false,
        left: false,
        right: false,
    };

    pub fn to_vec2(&self) -> Vec2 {
        fn btf(b: bool) -> f32 {
            if b { 1.0 } else { 0.0 }
        }
        Vec2::new(
            -btf(self.left) + btf(self.right),
            -btf(self.top) + btf(self.bottom),
        )
    }
}

pub struct Dpad<'a> {
    value: &'a mut DpadState,
    size: f32,
    intersect: bool,
}

impl<'a> Dpad<'a> {
    pub fn new(value: &'a mut DpadState) -> Self {
        Self {
            value,
            size: 120.0,
            intersect: false,
        }
    }

    fn intersection(&self, a: f32) -> DpadState {
        let mut state = DpadState::EMPTY;
        let pi_half_step = if self.intersect {
            PI * 0.375
        } else {
            PI * 0.25
        };
        if a > -pi_half_step && a <= pi_half_step {
            // Right quarter
            state.right = true;
        }
        if a > (PI * 0.5) - pi_half_step && a <= (PI * 0.5) + pi_half_step {
            // Bottom quarter
            state.bottom = true;
        }
        if a < (PI * -0.5) + pi_half_step && a >= (PI * -0.5) - pi_half_step {
            // Top quarter
            state.top = true;
        }
        if a <= -PI + pi_half_step || a >= PI - pi_half_step {
            // Left quarter
            state.left = true;
        }
        state
    }

    pub fn size(mut self, size: f32) -> Self {
        self.size = size;
        self
    }

    pub fn intersect(mut self, intersect: bool) -> Self {
        self.intersect = intersect;
        self
    }
}

const ONE_SIXTH: f32 = 0.16667;
const ONE_THIRD: f32 = 0.33333;

const TOP_DPAD: [Pos2; 5] = [
    Pos2::new(-1.0, -0.75),
    Pos2::new(1.0, -0.75),
    Pos2::new(1.0, 0.75),
    Pos2::new(0.0, 1.75),
    Pos2::new(-1.0, 0.75),
];

const BOTTOM_DPAD: [Pos2; 5] = [
    Pos2::new(-1.0, 0.75),
    Pos2::new(1.0, 0.75),
    Pos2::new(1.0, -0.75),
    Pos2::new(0.0, -1.75),
    Pos2::new(-1.0, -0.75),
];

const LEFT_DPAD: [Pos2; 5] = [
    Pos2::new(-0.75, -1.0),
    Pos2::new(-0.75, 1.0),
    Pos2::new(0.75, 1.0),
    Pos2::new(1.75, 0.0),
    Pos2::new(0.75, -1.0),
];

const RIGHT_DPAD: [Pos2; 5] = [
    Pos2::new(0.75, -1.0),
    Pos2::new(0.75, 1.0),
    Pos2::new(-0.75, 1.0),
    Pos2::new(-1.75, 0.0),
    Pos2::new(-0.75, -1.0),
];

fn scale_and_move_path(mut path: Vec<Pos2>, scale: f32, offset: Vec2) -> Vec<Pos2> {
    for p in &mut path {
        *p = *p * scale + offset;
    }

    path
}

impl<'a> egui::Widget for Dpad<'a> {
    fn ui(self, ui: &mut egui::Ui) -> egui::Response {
        let (rect, response) = ui.allocate_exact_size(Vec2::splat(self.size), Sense::drag());
        let center = rect.center();

        if response.is_pointer_button_down_on() && response.contains_pointer() {
            let pos = response
                .interact_pointer_pos()
                .expect("At this point this must be Some()");
            let delta = pos - center;
            let l = delta.length();

            if l <= self.size * 0.5 && l >= self.size * 0.1 {
                let a = delta.y.atan2(delta.x);
                *self.value = self.intersection(a);
            } else {
                *self.value = DpadState::EMPTY;
            }
        }
        if !response.is_pointer_button_down_on() || !response.contains_pointer() {
            *self.value = DpadState::EMPTY;
        }

        // Painting
        if ui.is_rect_visible(rect) {
            let painter = ui.painter_at(rect);
            let va = ui.visuals().widgets.active;
            let vi = ui.visuals().widgets.inactive;

            painter.add(PathShape::convex_polygon(
                scale_and_move_path(
                    TOP_DPAD.to_vec(),
                    self.size * ONE_SIXTH,
                    (center + Vec2::new(0.0, -self.size * ONE_THIRD)).to_vec2(),
                ),
                if self.value.top {
                    va.bg_fill
                } else {
                    vi.bg_fill
                },
                if self.value.top {
                    va.bg_stroke
                } else {
                    vi.bg_stroke
                },
            ));
            painter.add(PathShape::convex_polygon(
                scale_and_move_path(
                    BOTTOM_DPAD.to_vec(),
                    self.size * ONE_SIXTH,
                    (center + Vec2::new(0.0, self.size * ONE_THIRD)).to_vec2(),
                ),
                if self.value.bottom {
                    va.bg_fill
                } else {
                    vi.bg_fill
                },
                if self.value.bottom {
                    va.bg_stroke
                } else {
                    vi.bg_stroke
                },
            ));
            painter.add(PathShape::convex_polygon(
                scale_and_move_path(
                    LEFT_DPAD.to_vec(),
                    self.size * ONE_SIXTH,
                    (center + Vec2::new(-self.size * ONE_THIRD, 0.0)).to_vec2(),
                ),
                if self.value.left {
                    va.bg_fill
                } else {
                    vi.bg_fill
                },
                if self.value.left {
                    va.bg_stroke
                } else {
                    vi.bg_stroke
                },
            ));
            painter.add(PathShape::convex_polygon(
                scale_and_move_path(
                    RIGHT_DPAD.to_vec(),
                    self.size * ONE_SIXTH,
                    (center + Vec2::new(self.size * ONE_THIRD, 0.0)).to_vec2(),
                ),
                if self.value.right {
                    va.bg_fill
                } else {
                    vi.bg_fill
                },
                if self.value.right {
                    va.bg_stroke
                } else {
                    vi.bg_stroke
                },
            ));
        }

        response
    }
}
