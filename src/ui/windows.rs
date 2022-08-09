// src/ui/windows.rs

use std::borrow::BorrowMut;
use imgui::*;
use imgui::sys::*;
use crate::utils;

#[allow(dead_code)]
unsafe fn set_next_window_pos(pos: (f32, f32)) {
    igSetNextWindowPos(ImVec2::from(pos), ImGuiCond_Always as ImGuiCond, ImVec2::from([0.0f32, 0.0f32]));
}

#[allow(dead_code)]
unsafe fn set_next_window_pos_from(pos: (f32, f32), condition: ImGuiCond_, pivot: ImVec2) {
    igSetNextWindowPos(ImVec2::from(pos), condition as ImGuiCond, pivot);
}

#[allow(dead_code)]
unsafe fn set_next_window_size(size: [f32; 2]) {
    igSetNextWindowSize(ImVec2::from(size), ImGuiCond_Always as ImGuiCond);
}

#[allow(dead_code)]
pub fn center_next_window(io: &Io) {
    unsafe {
        let title_bar_height = igGetFontSize() + (*igGetStyle()).FramePadding.y * 2.0;
        let pos = (io.display_size[0] * 0.5f32, io.display_size[1] * 0.5f32 - title_bar_height);
        set_next_window_pos(pos);
    }
}

#[allow(dead_code)]
pub fn position_next_window(io: &Io, window_size: [f32; 2], x_pos: f32, y_pos: f32, condition: ImGuiCond_, pivot: ImVec2) {
    let x_max = io.display_size[0];
    let y_max = io.display_size[1];
    
    // "Clamping" the window to fit inside the bounds of the display size
    let mut pos = (x_max * x_pos, y_max * y_pos);
    if pos.0 < 0.0 {
        pos.0 = 0.0;
    } else if pos.0 > x_max - window_size[0] {
        pos.0 = x_max - window_size[0];
    }
    
    unsafe {
        let title_bar_height = igGetFontSize() + (*igGetStyle()).FramePadding.y * 2.0;
        if pos.1 < title_bar_height {
            pos.1 = title_bar_height;
        } else if pos.1 > y_max - window_size[1] {
            pos.1 = y_max - window_size[1];
        }
        set_next_window_pos_from(pos, condition, pivot);
    }
}

#[allow(dead_code)]
pub fn show_about_modal_window(ui: &Ui, show: &mut bool) {
    if *show {
        {
            let io = ui.io();
            let window_size = [320f32, 90f32];
            position_next_window(io, window_size, 0.5, 0.25, ImGuiCond_Always, ImVec2::from([0.5, 0.5]));
        }
        ui.open_popup("About");
        let about_modal = ui.modal_popup_config("About");
        let flags = WindowFlags::NO_COLLAPSE | WindowFlags::NO_MOVE |
            WindowFlags::NO_RESIZE | WindowFlags::NO_SAVED_SETTINGS;
        if let Some(_modal) =
        about_modal.flags(flags).begin_popup() {
            let message =
                "This is a window package written in Rust with imgui-rs.\n".to_owned() +
                    "You can use this for your own project - Just add code!";
            ui.text(message);
            if ui.button("OK") {
                *show = false;
                ui.close_current_popup();
            }
        }
    }
}

#[allow(dead_code)]
pub fn show_debug_window(ui: &Ui, show: &mut bool) {
    static mut WINDOW_FOCUSED: bool = false;
    static mut SHOW_DEBUG_WINDOW: bool = false;
    unsafe {
        SHOW_DEBUG_WINDOW = *show.borrow_mut();
        
        if SHOW_DEBUG_WINDOW {
            let title_bg_active_color: [f32; 4] = utils::color_from_bytes(36f32, 36f32, 36f32, 255f32);
            let style_title_bg_active_color = ui.push_style_color(StyleColor::TitleBgActive, title_bg_active_color);
            let title_bg_inactive_color: [f32; 4] = utils::color_from_bytes(53f32, 53f32, 53f32, 255f32);
            let style_title_bg_inactive_color = ui.push_style_color(StyleColor::TitleBg, title_bg_inactive_color);
            let bg_color: [f32; 4];
            if WINDOW_FOCUSED {
                bg_color = utils::color_from_bytes(20f32, 20f32, 20f32, 255f32);
            } else {
                bg_color = utils::color_from_bytes(20f32, 20f32, 20f32, 200f32);
            }
            let style_bg_color = ui.push_style_color(StyleColor::WindowBg, bg_color);
            
            let window_size = [200f32, 100f32];
            let io = ui.io();
            position_next_window(io, window_size, 1.0, 0.0, ImGuiCond_Appearing, ImVec2::from([0.0, 0.0]));
            
            let label = "Debug Window".to_owned();
            if let Some(debug_window) = ui.window(label)
                                          .size(window_size, Condition::Appearing)
                                          .opened(show)
                                          .begin()
            {
                WINDOW_FOCUSED = ui.is_window_focused();
                static mut SHOW_CURSOR_POS: bool = false;
                let label_cursor_pos = "Cursor Position".to_owned();
                let text_color = ui.style_color(StyleColor::Text);
                let disabled_text_color = ui.style_color(StyleColor::TextDisabled);
                let mut style_disabled_text_color = ui.push_style_color(StyleColor::Text, text_color);
                if !WINDOW_FOCUSED {
                    style_disabled_text_color.pop();
                    style_disabled_text_color = ui.push_style_color(StyleColor::Text, disabled_text_color);
                }
                
                let _tree_node_cursor_pos =
                    ui.tree_node_config(&label_cursor_pos)
                      .opened(SHOW_CURSOR_POS, Condition::Appearing)
                      .build(|| {
                          let [x, y] = if ui.io().mouse_pos < [0f32, 0f32] { [0f32, 0f32] } else { ui.io().mouse_pos };
                          let x_coord = format!("x:    {}", x);
                          let y_coord = format!("y:    {}", y);
                          ui.text(&x_coord);
                          ui.text(&y_coord);
                      });
                style_disabled_text_color.pop();
                debug_window.end();
            }
            style_title_bg_active_color.pop();
            style_title_bg_inactive_color.pop();
            style_bg_color.pop();
        }
    }
}
