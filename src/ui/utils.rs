use egui::{Rect, UiBuilder};

pub fn replace_extension(filename: &str, new_ext: &str) -> String {
    let mut path = std::path::Path::new(filename).to_owned();
    path.set_extension(new_ext);
    path.to_string_lossy().into_owned()
}

pub fn fit_size_aspect(size: egui::Vec2, aspect_ratio: f32) -> egui::Vec2 {
    let width = size.x;
    let height = size.y;
    if width / height > aspect_ratio {
        egui::vec2(height * aspect_ratio, height)
    } else {
        egui::vec2(width, width / aspect_ratio)
    }
}

pub fn ui_center(ui: &mut egui::Ui, size: egui::Vec2, add_contents: impl FnOnce(&mut egui::Ui)) {
    let rect = Rect::from_center_size(ui.available_rect_before_wrap().center(), size);
    ui.allocate_new_ui(UiBuilder::new().max_rect(rect), add_contents);
}

pub fn count_true<'a, I>(iter: I) -> usize
where
    I: Iterator<Item = &'a bool>,
{
    iter.map(|s| *s as usize).sum()
}
