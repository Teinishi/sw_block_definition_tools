use super::{
    definition_schema::Definition,
    sw_mesh::{SwMesh, SwMeshFromFileError},
};
use quick_xml;
use std::{
    fmt, io,
    path::{Path, PathBuf},
    rc::Rc,
};

#[derive(serde::Deserialize, serde::Serialize, Debug)]
pub struct SwBlockDefinition {
    rom_path: PathBuf,
    path: PathBuf,
    filename: String,
    #[serde(skip)]
    data: Option<Result<Rc<Definition>, SwBlockDefinitionDataError>>,
    #[serde(skip)]
    meshes: Option<Rc<SwBlockDefinitionMeshes>>,
}

impl SwBlockDefinition {
    pub fn new<P: AsRef<Path>, Q: AsRef<Path>>(rom_path: P, path: Q) -> Option<Self> {
        let pathbuf = path.as_ref().to_path_buf();
        let filename = pathbuf.file_name()?.to_os_string().into_string().ok()?;
        Some(Self {
            rom_path: rom_path.as_ref().to_path_buf(),
            path: pathbuf,
            filename,
            data: None,
            meshes: Default::default(),
        })
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    fn open_file(&mut self) -> Result<Rc<Definition>, SwBlockDefinitionDataError> {
        let xml = std::fs::read_to_string(self.path.clone())?;

        // ルート要素が  <definition> であるかチェック
        let is_definition: Result<(), String> = {
            let mut xml_reader = quick_xml::Reader::from_str(&xml);
            xml_reader.config_mut().trim_text(true);
            loop {
                if let Ok(event) = xml_reader.read_event() {
                    match event {
                        quick_xml::events::Event::Start(ref e) => {
                            if e.name().as_ref() == b"definition" {
                                break Ok(());
                            } else {
                                break Err(format!(
                                    "Unexpected root element: {:?}",
                                    std::str::from_utf8(e.name().as_ref()).unwrap_or_default(),
                                ));
                            }
                        }
                        _ => {}
                    }
                } else {
                    break Err("Could not find root element".to_string());
                }
            }
        };

        if let Err(mes) = is_definition {
            Err(SwBlockDefinitionDataError::XmlError(mes))
        } else {
            let data: Definition = quick_xml::de::from_str(&xml)?;
            self.meshes = Some(Rc::new(SwBlockDefinitionMeshes::new(
                &data,
                self.rom_path.clone(),
            )));
            Ok(Rc::new(data))
        }
    }

    pub fn data(&mut self) -> Result<Rc<Definition>, SwBlockDefinitionDataError> {
        if let Some(data) = &self.data {
            data.clone()
        } else {
            let data = self.open_file();
            self.data = Some(data.clone());
            data
        }
    }

    pub fn meshes(&mut self) -> Rc<SwBlockDefinitionMeshes> {
        let _ = self.data();
        self.meshes.clone().unwrap()
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

#[derive(Default, Debug)]
pub struct SwBlockDefinitionMeshes {
    mesh_data: Option<Result<SwMesh, SwMeshFromFileError>>,
    mesh_0: Option<Result<SwMesh, SwMeshFromFileError>>,
    mesh_1: Option<Result<SwMesh, SwMeshFromFileError>>,
    mesh_2: Option<Result<SwMesh, SwMeshFromFileError>>,
    mesh_editor_only: Option<Result<SwMesh, SwMeshFromFileError>>,
}

impl SwBlockDefinitionMeshes {
    pub fn new<P: AsRef<Path>>(data: &Definition, rom_path: P) -> Self {
        let mesh_from_file = |name: &Option<String>| {
            if let Some(name) = name {
                if name.len() > 0 {
                    return Some(SwMesh::from_file(rom_path.as_ref().join(name)));
                }
            }
            None
        };
        Self {
            mesh_data: mesh_from_file(&data.mesh_data_name),
            mesh_0: mesh_from_file(&data.mesh_0_name),
            mesh_1: mesh_from_file(&data.mesh_1_name),
            mesh_2: mesh_from_file(&data.mesh_2_name),
            mesh_editor_only: mesh_from_file(&data.mesh_editor_only_name),
        }
    }

    pub fn mesh_data(&self) -> &Option<Result<SwMesh, SwMeshFromFileError>> {
        &self.mesh_data
    }

    pub fn mesh_0(&self) -> &Option<Result<SwMesh, SwMeshFromFileError>> {
        &self.mesh_0
    }

    pub fn mesh_1(&self) -> &Option<Result<SwMesh, SwMeshFromFileError>> {
        &self.mesh_1
    }

    pub fn mesh_2(&self) -> &Option<Result<SwMesh, SwMeshFromFileError>> {
        &self.mesh_2
    }

    pub fn mesh_editor_only(&self) -> &Option<Result<SwMesh, SwMeshFromFileError>> {
        &self.mesh_editor_only
    }
}
