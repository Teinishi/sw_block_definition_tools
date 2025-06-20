use std::path::PathBuf;

pub fn ui_filepath_edit(
    ui: &mut egui::Ui,
    pathbuf: &Option<PathBuf>,
    path_str: &mut String,
) -> Option<FilepathEditAction> {
    let mut action = None;

    ui.horizontal(|ui| {
        let path_buf = pathbuf.clone().unwrap_or_default();
        *path_str = path_buf
            .as_os_str()
            .to_str()
            .unwrap_or_default()
            .to_string();
        let text_edit = ui.add_sized([300.0, 18.0], egui::TextEdit::singleline(path_str));
        if text_edit.changed() {
            action = Some(FilepathEditAction::Update(PathBuf::from(path_str.clone())));
        }

        if ui.button("Select").clicked() {
            action = Some(FilepathEditAction::Select);
        }
    });

    action
}

pub enum FilepathEditAction {
    Update(PathBuf),
    Select,
}
