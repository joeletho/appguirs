// src/main.rs

use crate::application::Application;

mod application;
mod window;
mod utils;
mod ui;
mod clipboard;

fn main() {
    Application::create("AppGuiRs")
        .run();
}
