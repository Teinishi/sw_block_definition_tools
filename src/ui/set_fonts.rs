pub fn set_fonts(cc: &eframe::CreationContext<'_>) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert(
        "noto_sans_jp_regular".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../fonts/NotoSansJP-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        "roboto_regular".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../fonts/Roboto-Regular.ttf"
        ))),
    );
    fonts.font_data.insert(
        "roboto_mono_regular".to_owned(),
        std::sync::Arc::new(egui::FontData::from_static(include_bytes!(
            "../../fonts/RobotoMono-Regular.ttf"
        ))),
    );
    let font_families_proportional = fonts
        .families
        .get_mut(&egui::FontFamily::Proportional)
        .unwrap();
    font_families_proportional.insert(0, "roboto_regular".to_owned());
    font_families_proportional.insert(1, "noto_sans_jp_regular".to_owned());
    let font_families_monospace = fonts
        .families
        .get_mut(&egui::FontFamily::Monospace)
        .unwrap();
    font_families_monospace.insert(0, "roboto_mono_regular".to_owned());
    cc.egui_ctx.set_fonts(fonts);
}
