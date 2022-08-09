// src/ui/style.rs

use imgui::{FontConfig, FontSource, StyleColor};
use crate::{
    ui::imgui::ImGui,
    utils,
};

#[allow(dead_code)]
pub fn set_default_colors(imgui: &mut ImGui) {
    let mut theme_colors = imgui.context.style().colors.clone();
    {
        let title_bg_active_color = utils::color_from_bytes(53f32, 53f32, 53f32, 255f32);
        let title_bg_inactive_color = utils::color_from_bytes(73f32, 73f32, 73f32, 255f32);
        let bg_color = utils::color_from_bytes(80f32, 80f32, 80f32, 255f32);
        theme_colors[StyleColor::TitleBgActive as usize] = title_bg_active_color;
        theme_colors[StyleColor::TitleBg as usize] = title_bg_inactive_color;
        theme_colors[StyleColor::WindowBg as usize] = bg_color;
    }
    imgui.context.style_mut().colors = theme_colors;
}

#[allow(dead_code)]
pub fn set_default_font(imgui: &mut ImGui) {
    let font_size = 13.0;
    imgui.context.fonts().add_font(&[FontSource::TtfData {
        data: include_bytes!("../resources/REFSAN.TTF"),
        size_pixels: font_size,
        config: Some(FontConfig {
            rasterizer_multiply: 1.5,
            oversample_h: 4,
            oversample_v: 4,
            ..FontConfig::default()
        }),
    }]);
}

#[allow(dead_code)]
pub fn set_default_theme(imgui: &mut ImGui) {
    set_default_colors(imgui);
    set_default_font(imgui);
}

#[allow(dead_code)]
pub fn set_dark_theme(imgui: &mut ImGui) {
    imgui.context.style_mut().use_dark_colors();
}

#[allow(dead_code)]
pub fn set_light_theme(imgui: &mut ImGui) {
    imgui.context.style_mut().use_light_colors();
}

#[allow(dead_code)]
pub fn set_classic_theme(imgui: &mut ImGui) {
    imgui.context.style_mut().use_classic_colors();
}

