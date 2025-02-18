use super::definition_schema::Definition;
use quick_xml;
use std::{
    fmt, io,
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
        let xml = std::fs::read_to_string(self.path.clone())?;
        let mut xml_reader = quick_xml::Reader::from_str(&xml);
        xml_reader.config_mut().trim_text(true);

        loop {
            if let Ok(event) = xml_reader.read_event() {
                match event {
                    quick_xml::events::Event::Start(ref e) => {
                        if e.name().as_ref() == b"definition" {
                            let data: Definition = quick_xml::de::from_str(&xml)?;
                            return Ok(Rc::new(data));
                        } else {
                            return Err(SwBlockDefinitionDataError::XmlError(format!(
                                "Unexpected root element: {:?}",
                                std::str::from_utf8(e.name().as_ref()).unwrap_or_default(),
                            )));
                        }
                    }
                    _ => {}
                }
            } else {
                break;
            }
        }
        Err(SwBlockDefinitionDataError::XmlError(
            "Could not find root element".to_string(),
        ))
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
    XmlError(String),
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
            Self::XmlError(mes) => write!(f, "XmlError: {}", mes),
        }
    }
}

impl std::error::Error for SwBlockDefinitionDataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
