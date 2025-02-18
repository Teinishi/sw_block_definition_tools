use super::definition_schema::Definition;
use quick_xml;
use std::{
    fmt,
    fs::File,
    io,
    path::{Path, PathBuf},
    rc::Rc,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SwBlockDefinition {
    path: PathBuf,
    filename: String,
    #[serde(skip)]
    data: Option<Result<Rc<Definition>, SwBlockDefinitionDataError>>,
}

impl SwBlockDefinition {
    pub fn new<P: AsRef<Path>>(path: P) -> Option<Self> {
        let pathbuf = path.as_ref().to_path_buf();
        let filename = pathbuf.file_name()?.to_os_string().into_string().ok()?;
        Some(Self {
            path: pathbuf,
            filename,
            data: None,
        })
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    fn open_file(&self) -> Result<Rc<Definition>, SwBlockDefinitionDataError> {
        let file = File::open(self.path.clone())?;
        let data: Definition = quick_xml::de::from_reader(io::BufReader::new(file))?;
        Ok(Rc::new(data))
    }

    pub fn data(&mut self) -> Result<Rc<Definition>, SwBlockDefinitionDataError> {
        if let Some(data) = &self.data {
            return data.clone();
        }
        let data = self.open_file();
        self.data = Some(data.clone());
        data
    }
}

impl PartialEq for SwBlockDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

#[derive(Debug, Clone)]
pub enum SwBlockDefinitionDataError {
    IoError(String),
    DeError(String),
}

impl From<io::Error> for SwBlockDefinitionDataError {
    fn from(value: io::Error) -> Self {
        Self::IoError(value.to_string())
    }
}

impl From<quick_xml::DeError> for SwBlockDefinitionDataError {
    fn from(value: quick_xml::DeError) -> Self {
        Self::DeError(value.to_string())
    }
}

impl fmt::Display for SwBlockDefinitionDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(mes) => write!(f, "IoError: {}", mes),
            Self::DeError(mes) => write!(f, "DeError: {}", mes),
        }
    }
}

impl std::error::Error for SwBlockDefinitionDataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
