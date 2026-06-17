pub mod demo_app;

#[cfg(target_os = "android")]
#[unsafe(no_mangle)]
fn android_main(android_app: winit::platform::android::activity::AndroidApp) {
    android_logger::init_once(
        android_logger::Config::default()
            .with_max_level(log::LevelFilter::Info)
            .with_tag("vcon-widget-android-demo"),
    );

    let options = eframe::NativeOptions {
        renderer: eframe::Renderer::Wgpu,
        android_app: Some(android_app),
        ..Default::default()
    };

    if let Err(err) = eframe::run_native(
        "Vcon widgets android demo",
        options,
        Box::new(|cc| Ok(Box::new(demo_app::DemoState::new(cc)))),
    ) {
        log::error!("eframe failed: {err:?}")
    }
}
