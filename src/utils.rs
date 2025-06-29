use std::{ffi::OsStr, io, process::Command};

pub fn replace_extension(filename: &OsStr, new_ext: &str) -> String {
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

pub fn count_true<'a, I>(iter: I) -> usize
where
    I: Iterator<Item = &'a bool>,
{
    iter.map(|s| *s as usize).sum()
}

pub fn check_xml_root_tag(xml: &str, root_tag: &[u8]) -> Result<(), String> {
    use quick_xml::events::Event;
    let mut xml_reader = quick_xml::Reader::from_str(xml);
    xml_reader.config_mut().trim_text(true);
    loop {
        if let Ok(event) = xml_reader.read_event() {
            match event {
                Event::Start(ref e) | Event::Empty(ref e) => {
                    if e.name().as_ref() == root_tag {
                        break Ok(());
                    } else {
                        break Err(format!(
                            "Unexpected root element: {:?}",
                            std::str::from_utf8(e.name().as_ref()).unwrap_or_default(),
                        ));
                    }
                }
                Event::Eof => {
                    break Err("Could not find root element".to_string());
                }
                _ => {}
            }
        } else {
            break Err("Could not find root element".to_string());
        }
    }
}

pub fn open_explorer(path: &str) -> io::Result<()> {
    Command::new("explorer.exe")
        .args(["/select,", path])
        .spawn()?;
    Ok(())
}
