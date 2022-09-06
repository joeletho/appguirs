// src/main.rs

extern crate appguirs;

use appguirs::application::Application;

fn main() {
    Application::create("AppGuiRs")
        .run();
}
