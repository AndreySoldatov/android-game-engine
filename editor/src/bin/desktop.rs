use editor::Editor;

fn main() -> eframe::Result {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "Age Editor",
        native_options,
        Box::new(|cc| Ok(Box::new(Editor::new(cc)))),
    )
}
