// src/application/mod.rs

use glium::Surface;
use glium::glutin::event::{Event, WindowEvent};
use glium::glutin::event_loop::{ControlFlow, EventLoop};
use imgui_glium_renderer::Renderer;
use imgui_winit_support::{HiDpiMode, WinitPlatform};
use std::path::Path;
use std::time::Instant;
use imgui::{Condition, Ui, WindowFlags};
use crate::{
    ui::windows::*,
    ui::imgui::ImGui,
    window::Window,
};

pub struct Application {
    name: &'static str,
    imgui: ImGui,
    window: Window,
    renderer: Renderer,
    platform: WinitPlatform,
    event_loop: EventLoop<()>,
}

impl Application {
    pub fn create(name: &'static str) -> Self {
        let title = match Path::new(name).file_name() {
            Some(file_name) => file_name.to_str().unwrap(),
            None => name,
        };
        let event_loop = EventLoop::new();
        let window_size = [680f32, 698f32];
        let window = Window::new(title, window_size, &event_loop);
        let mut imgui = ImGui::new();
        let mut platform = WinitPlatform::init(&mut imgui.context);
        {
            let gl_window = window.display().gl_window();
            platform.attach_window(imgui.context.io_mut(), gl_window.window(), HiDpiMode::Default);
        }
        let renderer = Renderer::init(&mut imgui.context, window.display())
            .expect("Failed to initialize renderer");
        
        Application {
            name,
            imgui,
            window,
            renderer,
            platform,
            event_loop,
        }
    }
    
    #[allow(dead_code)]
    pub fn set_name(&mut self, name: &'static str) {
        self.name = name;
    }
    #[allow(dead_code)]
    pub fn name(&self) -> &str {
        self.name
    }
    
    pub fn run(self) {
        let mut show_about_modal = false;
        let mut show_debug_window = false;
        let mut show_demo_window = false;
        let mut exit_loop = false;
        
        println!("Starting `{}`...", &self.name());
        
        self.main_loop(move |ui, main_window| {
            // We want our ui window to be the same size of our main window
            //  so that we have a surface to draw our ui.
            let [x, y] = main_window.size();
            
            // We give a -1px offset on the y-axis from the origin so the window
            //  has a seamless transition into title bar
            let window_position = [0f32, -1f32];
            let [x_offset, y_offset] = &window_position;
            
            // We then subtract _window_offset_ so _window_size_ always fills the window
            let window_size = [x - x_offset, y - y_offset];
            
            // Set up window flags for our main window
            let window_flags = WindowFlags::NO_COLLAPSE | WindowFlags::NO_RESIZE
                | WindowFlags::NO_MOVE | WindowFlags::NO_TITLE_BAR
                | WindowFlags::MENU_BAR | WindowFlags::NO_COLLAPSE
                | WindowFlags::ALWAYS_AUTO_RESIZE | WindowFlags::NO_BRING_TO_FRONT_ON_FOCUS;
            
            // Create the window with our custom size, position, and flags
            let ui_window = ui.window("##ui_window")
                              .size(window_size, Condition::Always)
                              .position(window_position, Condition::Always)
                              .flags(window_flags);
            ui_window.build(|| {
                // Menu bar
                if let Some(menu_bar) = ui.begin_menu_bar() {
                    if let Some(menu_file) = ui.begin_menu("File") {
                        if ui.menu_item("Exit") {
                            exit_loop = true;
                        }
                        menu_file.end();
                    }
                    if let Some(menu_edit) = ui.begin_menu("Edit") {
                        if ui.menu_item("Cut") {
                            // TODO
                        };
                        if ui.menu_item("Copy") {
                            // TODO
                        };
                        menu_edit.end();
                    }
                    if let Some(menu_options) = ui.begin_menu("Options") {
                        if ui.menu_item("About") {
                            show_about_modal = true;
                        }
                        if ui.menu_item("Show Demo Window") {
                            show_demo_window = true;
                        }
                        ui.separator();
                        let debug_menu_label =
                            if !show_debug_window { "Enable Debug".to_owned() } else { "Disable Debug".to_owned() };
                        if ui.menu_item(debug_menu_label) {
                            show_debug_window = !show_debug_window;
                        }
                        
                        menu_options.end();
                    }
                    self::show_debug_window(ui, &mut show_debug_window);
                    menu_bar.end();
                }
                show_about_modal_window(ui, &mut show_about_modal);
                // End menu bar
                
                // Remaining code goes here!
                if show_demo_window {
                    ui.show_demo_window(&mut show_demo_window);
                }
            });
            
            // Notify caller of loop status
            return !exit_loop;
        });
    }
    
    pub fn main_loop<F: FnMut(&mut Ui, &Window) -> bool + 'static>(self, mut run_ui: F) {
        let Application {
            event_loop,
            window,
            mut imgui,
            mut platform,
            mut renderer,
            ..
        } = self;
        let mut last_frame = Instant::now();
        
        event_loop.run(move |event, _, control_flow| match event {
            Event::NewEvents(_) => {
                // Update any new events
                let now = Instant::now();
                imgui.context.io_mut().update_delta_time(now - last_frame);
                last_frame = now;
            }
            Event::MainEventsCleared => {
                // Processing code goes here
                let gl_window = &window.display().gl_window();
                platform
                    .prepare_frame(imgui.context.io_mut(), gl_window.window())
                    .expect("Failed to prepare frame");
                gl_window.window().request_redraw();
            }
            Event::RedrawRequested(_) => {
                // Begin the new frame to render
                let ui = imgui.context.new_frame();
                // Run imgui Ui and allow UI to request exiting the program
                if !run_ui(ui, &window) {
                    *control_flow = ControlFlow::Exit;
                };
                
                // Render
                let mut surface = window.display().draw();
                surface.clear_color_srgb(1.0, 1.0, 1.0, 1.0);
                let gl_window = window.display().gl_window();
                platform.prepare_render(&ui, gl_window.window());
                let draw_data = imgui.context.render();
                renderer.render(&mut surface, draw_data).expect("Rendering failed");
                surface.finish().expect("Failed to swap buffers");
            }
            Event::WindowEvent {
                // Exit application
                event: WindowEvent::CloseRequested,
                ..
            } => *control_flow = ControlFlow::Exit,
            event => {
                let gl_window = &window.display().gl_window();
                platform.handle_event(imgui.context.io_mut(), gl_window.window(), &event);
            }
        })
    }
}

