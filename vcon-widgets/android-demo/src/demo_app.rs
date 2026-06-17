use std::f32::consts::PI;

use eframe::egui;
use eframe::egui::{Slider, Vec2};
use vcon_widgets::{
    dpad::{Dpad, DpadState},
    thumbstick::{AngleSnap, Thumbstick},
};

struct ThumbstickProperties {
    value: Vec2,
    inner_radius: f32,
    outer_radius: f32,
    reset_on_release: bool,
    do_length_steps: bool,
    length_snap: usize,
    do_angle_snap: bool,
    angle_snap: AngleSnap,
}

struct DpadProperties {
    value: DpadState,
    size: f32,
    intersect: bool,
}

pub struct DemoState {
    thumb: ThumbstickProperties,
    dpad: DpadProperties,
}

impl DemoState {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            thumb: ThumbstickProperties {
                value: Vec2::ZERO,
                inner_radius: 20.0,
                outer_radius: 60.0,
                reset_on_release: true,
                do_length_steps: false,
                length_snap: 4,
                do_angle_snap: false,
                angle_snap: AngleSnap {
                    steps: 4,
                    offset: 0.0,
                },
            },
            dpad: DpadProperties {
                value: DpadState::EMPTY,
                size: 120.0,
                intersect: false,
            },
        }
    }
}

impl eframe::App for DemoState {
    fn ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                ui.vertical(|ui| {
                    ui.heading("Thumbstick");
                    ui.label(format!("{:.2?}", self.thumb.value));

                    ui.label("Inner radius");
                    ui.add(Slider::new(&mut self.thumb.inner_radius, 1.0..=150.0));
                    ui.label("Outer radius");
                    ui.add(Slider::new(&mut self.thumb.outer_radius, 1.0..=150.0));

                    ui.checkbox(&mut self.thumb.reset_on_release, "Reset on release");

                    ui.checkbox(&mut self.thumb.do_length_steps, "Length snap");
                    ui.label("Length snap segments");
                    ui.add(Slider::new(&mut self.thumb.length_snap, 2..=8));

                    ui.checkbox(&mut self.thumb.do_angle_snap, "Angle snap");
                    ui.label("Angle snap segments");
                    ui.add(Slider::new(&mut self.thumb.angle_snap.steps, 2..=8));
                    ui.label("Angle snap offset");
                    ui.add(Slider::new(&mut self.thumb.angle_snap.offset, 0.0..=PI));

                    ui.add(
                        Thumbstick::new(&mut self.thumb.value)
                            .inner_radius(self.thumb.inner_radius)
                            .outer_radius(self.thumb.outer_radius)
                            .reset_on_release(self.thumb.reset_on_release)
                            .length_snap(if self.thumb.do_length_steps {
                                Some(self.thumb.length_snap)
                            } else {
                                None
                            })
                            .angle_snap(if self.thumb.do_angle_snap {
                                Some(self.thumb.angle_snap)
                            } else {
                                None
                            }),
                    );
                });
                ui.separator();
                ui.vertical(|ui| {
                    ui.heading("Dpad");
                    ui.label(format!("{:#?}", self.dpad.value));

                    ui.checkbox(&mut self.dpad.intersect, "Intersect");
                    ui.label("Size");
                    ui.add(egui::Slider::new(&mut self.dpad.size, 80.0..=200.0));

                    ui.add(
                        Dpad::new(&mut self.dpad.value)
                            .intersect(self.dpad.intersect)
                            .size(self.dpad.size),
                    );
                })
            });
        });
    }
}
