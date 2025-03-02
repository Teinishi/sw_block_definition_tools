use super::{
    sw_mesh::{SwMesh, SwMeshFromFileError},
    Definition,
};
use std::{
    collections::BTreeMap,
    fmt, io,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{mpsc, Arc},
    thread,
};

type LoadDataResult = Result<Definition, SwBlockDefinitionDataError>;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SwBlockDefinition {
    rom_path: PathBuf,
    path: PathBuf,
    filename: String,
    #[serde(skip)]
    data: Option<Result<Arc<Definition>, SwBlockDefinitionDataError>>,
    #[serde(skip)]
    meshes: Option<Rc<SwBlockDefinitionMeshes>>,
    #[serde(skip)]
    load_data_thread: Option<mpsc::Receiver<LoadDataResult>>,
    #[serde(skip)]
    load_mesh_thread: Option<mpsc::Receiver<SwBlockDefinitionMeshes>>,
}

impl PartialEq for SwBlockDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
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
            meshes: None,
            load_data_thread: None,
            load_mesh_thread: None,
        })
    }

    pub fn filename(&self) -> String {
        self.filename.clone()
    }

    pub fn filepath(&self) -> PathBuf {
        self.rom_path
            .join("data/definitions/")
            .join(self.filename.clone())
    }

    pub fn load_data(&mut self) -> Option<Result<Arc<Definition>, SwBlockDefinitionDataError>> {
        if let Some(rx) = &self.load_data_thread {
            if let Ok(r) = rx.try_recv() {
                match r {
                    Ok(data) => {
                        self.data = Some(Ok(Arc::new(data)));
                    }
                    Err(err) => {
                        self.data = Some(Err(err));
                    }
                }
                self.meshes = None;
                self.load_data_thread = None;
            }
        }

        if let Some(data) = &self.data {
            Some(data.clone())
        } else {
            self.spawn_load_data();
            None
        }
    }

    pub fn data(&self) -> Option<Result<Arc<Definition>, SwBlockDefinitionDataError>> {
        self.data.clone()
    }

    pub fn load_meshes(&mut self) -> Option<Rc<SwBlockDefinitionMeshes>> {
        if let Some(rx) = &self.load_mesh_thread {
            if let Ok(meshes) = rx.try_recv() {
                self.meshes = Some(Rc::new(meshes));
                self.load_mesh_thread = None;
            }
        }

        if let Some(meshes) = &self.meshes {
            Some(meshes.clone())
        } else {
            self.spawn_load_meshes();
            None
        }
    }

    fn spawn_load_data(&mut self) {
        if self.load_data_thread.is_some() {
            return;
        }

        let path = self.path.clone();

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            tx.send(load_data(path)).unwrap_or_default();
        });

        self.load_data_thread = Some(rx);
    }

    fn spawn_load_meshes(&mut self) {
        if self.load_mesh_thread.is_some() {
            return;
        }

        if let Some(Ok(data)) = &self.data {
            let data = data.clone();
            let rom_path = self.rom_path.clone();

            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                tx.send(load_meshes(&data, rom_path)).unwrap();
            });

            self.load_mesh_thread = Some(rx);
        }
    }
}

fn load_data<P: AsRef<Path>>(path: P) -> LoadDataResult {
    let xml = std::fs::read_to_string(path)?;

    // ルート要素が  <definition> であるかチェック
    let check_definition: Result<(), String> = {
        let mut xml_reader = quick_xml::Reader::from_str(&xml);
        xml_reader.config_mut().trim_text(true);
        loop {
            if let Ok(event) = xml_reader.read_event() {
                if let quick_xml::events::Event::Start(ref e) = event {
                    if e.name().as_ref() == b"definition" {
                        break Ok(());
                    } else {
                        break Err(format!(
                            "Unexpected root element: {:?}",
                            std::str::from_utf8(e.name().as_ref()).unwrap_or_default(),
                        ));
                    }
                }
            } else {
                break Err("Could not find root element".to_string());
            }
        }
    };

    if let Err(mes) = check_definition {
        Err(SwBlockDefinitionDataError::Xml(mes))
    } else {
        let data = quick_xml::de::from_str(&xml)?;
        //let meshes = SwBlockDefinitionMeshes::new(&data, rom_path);
        Ok(data)
    }
}

fn load_meshes<P: AsRef<Path>>(data: &Definition, rom_path: P) -> SwBlockDefinitionMeshes {
    thread::sleep(std::time::Duration::from_secs(1));
    SwBlockDefinitionMeshes::new(data, rom_path)
}

#[derive(Debug, Clone)]
pub enum SwBlockDefinitionDataError {
    Io(String),
    De(String),
    Xml(String),
}

impl From<io::Error> for SwBlockDefinitionDataError {
    fn from(value: io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<quick_xml::DeError> for SwBlockDefinitionDataError {
    fn from(value: quick_xml::DeError) -> Self {
        Self::De(value.to_string())
    }
}

impl fmt::Display for SwBlockDefinitionDataError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(mes) => write!(f, "IoError: {}", mes),
            Self::De(mes) => write!(f, "DeError: {}", mes),
            Self::Xml(mes) => write!(f, "XmlError: {}", mes),
        }
    }
}

impl std::error::Error for SwBlockDefinitionDataError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}

#[derive(
    serde::Deserialize, serde::Serialize, enum_map::Enum, Clone, PartialEq, PartialOrd, Eq, Ord,
)]
pub enum SwBlockDefinitionMeshKey {
    MeshData,
    Mesh0,
    Mesh1,
    Mesh2,
    MeshEditorOnly,
}

impl SwBlockDefinitionMeshKey {
    pub fn xml_name(&self) -> &str {
        match self {
            Self::MeshData => "mesh_data_name",
            Self::Mesh0 => "mesh_0_name",
            Self::Mesh1 => "mesh_1_name",
            Self::Mesh2 => "mesh_2_name",
            Self::MeshEditorOnly => "mesh_editor_only_name",
        }
    }
}

#[derive(Default)]
pub struct SwBlockDefinitionMeshes {
    meshes: BTreeMap<SwBlockDefinitionMeshKey, Result<SwMesh, SwMeshFromFileError>>,
}

impl SwBlockDefinitionMeshes {
    pub fn new<P: AsRef<Path>>(data: &Definition, rom_path: P) -> Self {
        let mut meshes = BTreeMap::new();

        for (key, name) in [
            (SwBlockDefinitionMeshKey::MeshData, &data.mesh_data_name),
            (SwBlockDefinitionMeshKey::Mesh0, &data.mesh_0_name),
            (SwBlockDefinitionMeshKey::Mesh1, &data.mesh_1_name),
            (SwBlockDefinitionMeshKey::Mesh2, &data.mesh_2_name),
            (
                SwBlockDefinitionMeshKey::MeshEditorOnly,
                &data.mesh_editor_only_name,
            ),
        ] {
            if let Some(name) = name {
                if !name.is_empty() {
                    meshes.insert(key, SwMesh::from_file(rom_path.as_ref().join(name)));
                }
            }
        }

        Self { meshes }
    }

    pub fn get_mesh(
        &self,
        key: &SwBlockDefinitionMeshKey,
    ) -> Option<&Result<SwMesh, SwMeshFromFileError>> {
        self.meshes.get(key)
    }
}
