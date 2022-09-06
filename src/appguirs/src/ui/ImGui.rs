// src/ui/ImGui.rs

use crate::ui::style;

pub struct ImGui {
    pub context: imgui::Context,
}

impl ImGui {
    fn init(&mut self) {
        if let Some(backend) = crate::clipboard::init() {
            self.context.set_clipboard_backend(backend);
        } else {
            eprintln!("Failed to initialize clipboard");
        }
        style::set_default_theme(self);
    }
    
    pub fn new() -> Self {
        let context = imgui::Context::create();
        let mut imgui = ImGui {
            context,
        };
        imgui.init();
        return imgui;
    }
}
