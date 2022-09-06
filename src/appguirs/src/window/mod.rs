// src/window/mod.rs

use glium::{Display, glutin};
use glium::glutin::event_loop::EventLoop;
use glium::glutin::window::WindowBuilder;

pub struct Window {
    title: &'static str,
    display: Display,
}

impl Window {
    pub fn new<E>(
        title: &'static str,
        size: [f32; 2],
        event_loop: &EventLoop<E>,
    ) -> Window {
        let builder = WindowBuilder::new()
            .with_title(title)
            .with_inner_size(glutin::dpi::LogicalSize::new(size[0], size[1]));
        let context = glutin::ContextBuilder::new().with_vsync(true);
        let display =
            Display::new(builder, context, &event_loop).expect("Failed to initialize display");
        
        Window {
            title,
            display,
        }
    }
    #[allow(dead_code)]
    pub fn display(&self) -> &Display {
        &self.display
    }
    #[allow(dead_code)]
    pub fn title(&self) -> &'static str {
        &self.title
    }
    #[allow(dead_code)]
    pub fn height(&self) -> f32 {
        self.display.gl_window().window().inner_size().height as f32
    }
    #[allow(dead_code)]
    pub fn width(&self) -> f32 {
        self.display.gl_window().window().inner_size().width as f32
    }
    #[allow(dead_code)]
    pub fn size(&self) -> [f32; 2] {
        [self.width() as f32, self.height() as f32]
    }
    #[allow(dead_code)]
    pub fn update(&self) {}
}

impl Drop for Window {
    fn drop(&mut self) {
        println!("`{}` window has closed.", self.title());
    }
}