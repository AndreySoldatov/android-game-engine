use eframe::egui;

pub struct Editor {
    code: String,
}

impl Editor {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Editor {
            code: "print \"Hello, world!\"".into(),
        }
    }
}

impl eframe::App for Editor {
    fn ui(&mut self, _ui: &mut eframe::egui::Ui, _frame: &mut eframe::Frame) {}
}
