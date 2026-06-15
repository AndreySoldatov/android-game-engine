use egui::{Rect, Slider, Vec2};
use vcon_widgets::thumbstick::Thumbstick;

fn main() {
    let mut v = Vec2::ZERO;
    let mut pos = Vec2::splat(50.0);

    eframe::run_ui_native(
        "vcon-widgets collection",
        eframe::NativeOptions::default(),
        move |ui, _frame| {
            egui::CentralPanel::default().show_inside(ui, |ui| {
                ui.label(format!("{:.2?}", v));

                ui.add(Slider::new(&mut pos.x, 50.0..=150.0));
                ui.add(Slider::new(&mut pos.y, 50.0..=150.0));

                ui.place(
                    Rect::from_center_size((pos * 2.0).to_pos2(), Vec2::splat(40.0 + 80.0)),
                    Thumbstick::new(&mut v),
                );
            });
        },
    )
    .unwrap();
}
