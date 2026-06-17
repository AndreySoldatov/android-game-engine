#[cfg(not(target_os = "android"))]
use android_demo::demo_app::DemoState;

#[cfg(not(target_os = "android"))]
fn main() {
    eframe::run_native(
        "Vcon widgets demo",
        eframe::NativeOptions::default(),
        Box::new(|cc| Ok(Box::new(DemoState::new(cc)))),
    )
    .unwrap();
}

#[cfg(target_os = "android")]
fn main() {}
