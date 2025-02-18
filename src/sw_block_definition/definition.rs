use std::path::{Path, PathBuf};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SwBlockDefinition {
    path: PathBuf,
    filename: String,
}

impl SwBlockDefinition {
    pub fn new<P: AsRef<Path>>(path: P) -> Option<Self> {
        let pathbuf = path.as_ref().to_path_buf();
        let filename = pathbuf.file_name()?.to_os_string().into_string().ok()?;
        Some(Self {
            path: pathbuf,
            filename,
        })
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }
}

impl PartialEq for SwBlockDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}
