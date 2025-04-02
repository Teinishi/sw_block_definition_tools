use eframe::egui_glow;
use egui::{Image, Rect, TextureOptions};
use std::sync::Arc;

use crate::gl_renderer::{OrbitCamera, SceneRenderer};

use super::BlockViewAppearance;

pub fn paint_canvas_3d(
    ui: &mut egui::Ui,
    rect: Rect,
    camera: OrbitCamera,
    renderer: Arc<egui::mutex::Mutex<SceneRenderer>>,
    appearance: &BlockViewAppearance,
) {
    let appearance_c = appearance.clone();
    let cb = egui_glow::CallbackFn::new(move |_info, painter| {
        renderer.lock().paint(painter.gl(), &camera, &appearance_c);
    });

    let callback = egui::PaintCallback {
        rect,
        callback: Arc::new(cb),
    };
    ui.painter().add(callback);
}

pub fn paint_checker_pattern(ui: &mut egui::Ui, rect: Rect) {
    egui_extras::install_image_loaders(ui.ctx());
    let background_image_source = match ui.ctx().theme() {
        egui::Theme::Light => egui::include_image!("../../images/checker_light.png"),
        egui::Theme::Dark => egui::include_image!("../../images/checker_dark.png"),
    };
    let background_image = Image::new(background_image_source)
        .texture_options(TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
            wrap_mode: egui::TextureWrapMode::Repeat,
            ..Default::default()
        })
        .uv(Rect::from_x_y_ranges(
            0.0..=(rect.width() / 16.0),
            0.0..=(rect.height() / 16.0),
        ));
    background_image.paint_at(ui, rect);
}
