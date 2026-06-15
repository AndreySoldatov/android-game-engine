use eframe::egui;

pub struct Editor {
    code: String,
}

impl Editor {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        Editor {
            code: "print \"Hello, world!\"".into(),
        }
    }
}

impl eframe::App for Editor {
    fn ui(&mut self, ui: &mut eframe::egui::Ui, frame: &mut eframe::Frame) {}
}
