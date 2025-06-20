use crate::sw_gl_3d::Color4;
use egui::{Button, DragValue, Rect, UiBuilder};
use glam::Vec3;
use std::collections::BTreeSet;

pub fn ui_center(ui: &mut egui::Ui, size: egui::Vec2, add_contents: impl FnOnce(&mut egui::Ui)) {
    let rect = Rect::from_center_size(ui.available_rect_before_wrap().center(), size);
    ui.allocate_new_ui(UiBuilder::new().max_rect(rect), add_contents);
}

pub fn ui_checkbox_btreeset<K: Ord + Eq>(
    ui: &mut egui::Ui,
    set: &mut BTreeSet<K>,
    value: K,
    text: impl Into<egui::WidgetText>,
) -> bool {
    let mut checked = set.contains(&value);
    ui.checkbox(&mut checked, text);
    if checked {
        set.insert(value);
    } else {
        set.remove(&value);
    }
    checked
}

pub fn ui_dragvalue_vec_z_inv(ui: &mut egui::Ui, vec: &mut Vec3, speed: f32) {
    let mut z = if vec.z == 0.0 { 0.0 } else { -vec.z };
    ui.horizontal(|ui| {
        ui.add(DragValue::new(&mut vec.x).speed(speed));
        ui.add(DragValue::new(&mut vec.y).speed(speed));
        if ui.add(DragValue::new(&mut z).speed(speed)).changed() {
            vec.z = -z;
        }
        if ui
            .add_sized(egui::vec2(20.0, 20.0), Button::new("\u{27F2}"))
            .clicked()
        {
            *vec = Vec3::ZERO;
        }
    });
}

pub fn ui_color_picker_rgb(ui: &mut egui::Ui, color: &mut Color4) {
    let mut arr: [f32; 3] = color.as_array()[..3].try_into().unwrap();
    ui.color_edit_button_rgb(&mut arr);
    color.r = arr[0];
    color.g = arr[1];
    color.b = arr[2];
    color.a = 1.0;
}

pub fn ui_color_picker_rgba(ui: &mut egui::Ui, color: &mut Color4) {
    let mut arr = color.as_array();
    ui.color_edit_button_rgba_unmultiplied(&mut arr);
    color.r = arr[0];
    color.g = arr[1];
    color.b = arr[2];
    color.a = arr[3];
}
