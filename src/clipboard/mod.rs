// src/clipboard/mod.rs

use clipboard::{ClipboardContext, ClipboardProvider};
use imgui::ClipboardBackend;

pub fn init() -> Option<ClipboardSupport> {
    ClipboardContext::new().ok().map(ClipboardSupport)
}

pub struct ClipboardSupport(pub ClipboardContext);

impl ClipboardBackend for ClipboardSupport {
    fn get(&mut self) -> Option<String> {
        self.0.get_contents().ok()
    }
    fn set(&mut self, text: &str) {
        // ignore errors?
        let _ = self.0.set_contents(text.to_owned());
    }
}