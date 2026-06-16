use std::f32::consts::PI;

use egui::{Slider, Vec2};
use vcon_widgets::{
    dpad::{Dpad, DpadState},
    thumbstick::{RadialSnap, Thumbstick},
};

struct ThumbstickState {
    v: Vec2,
    ir: f32,
    or: f32,
    ror: bool,
}

fn main() {
    let mut thumb_state = ThumbstickState {
        v: Vec2::ZERO,
        ir: 20.0,
        or: 80.0,
        ror: true,
    };
    let mut do_ls = false;
    let mut ls = 4;

    let mut do_rs = false;
    let mut rs = 4;
    let mut offset = 0.0;

    let mut dpadv = DpadState::EMPTY;

    eframe::run_ui_native(
        "vcon-widgets collection",
        eframe::NativeOptions::default(),
        move |ui, _frame| {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.label(format!("Thumbstick value: {:.2?}", thumb_state.v));

                ui.add(Slider::new(&mut thumb_state.ir, 1.0..=150.0));
                ui.add(Slider::new(&mut thumb_state.or, 1.0..=150.0));

                ui.checkbox(&mut thumb_state.ror, "Reset on release");

                ui.checkbox(&mut do_ls, "Length snap");
                ui.add(Slider::new(&mut ls, 2..=8));

                ui.checkbox(&mut do_rs, "Radial snap");
                ui.add(Slider::new(&mut rs, 2..=8));
                ui.add(Slider::new(&mut offset, 0.0..=PI));

                ui.add(
                    Thumbstick::new(&mut thumb_state.v)
                        .inner_radius(thumb_state.ir)
                        .outer_radius(thumb_state.or)
                        .reset_on_release(thumb_state.ror)
                        .length_steps(if do_ls { Some(ls) } else { None })
                        .radial_snap(if do_rs {
                            Some(RadialSnap { steps: rs, offset })
                        } else {
                            None
                        }),
                );

                ui.add(Dpad::new(&mut dpadv).intersect(true));
            });
        },
    )
    .unwrap();
}
