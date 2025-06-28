use crate::{
    store::{ModFileLoader, ModKey},
    sw_block_definition::Definition,
    sw_gl_3d::SwBlockMeshes,
    utils::check_xml_root_tag,
};
use std::{
    fmt, io,
    path::{Path, PathBuf},
    sync::{mpsc, Arc},
    thread,
};

type LoadDataResult = Result<Definition, SwBlockDefinitionDataError>;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SwBlockDefinition {
    mod_key: ModKey,
    path: PathBuf,
    filename: String,
    #[serde(skip)]
    data: Option<Result<Arc<Definition>, SwBlockDefinitionDataError>>,
    #[serde(skip)]
    meshes: Option<Arc<SwBlockMeshes>>,
    #[serde(skip)]
    load_data_thread: Option<mpsc::Receiver<LoadDataResult>>,
    #[serde(skip)]
    load_mesh_thread: Option<mpsc::Receiver<SwBlockMeshes>>,
}

impl PartialEq for SwBlockDefinition {
    fn eq(&self, other: &Self) -> bool {
        self.path == other.path
    }
}

impl SwBlockDefinition {
    pub fn new<P: AsRef<Path>>(mod_key: ModKey, path: P) -> Option<Self> {
        let pathbuf = path.as_ref().to_path_buf();
        let filename = pathbuf.file_name()?.to_os_string().into_string().ok()?;
        Some(Self {
            mod_key,
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

    pub fn path(&self) -> &PathBuf {
        &self.path
    }

    pub fn is_loading_data(&self) -> bool {
        self.load_data_thread.is_some()
    }

    pub fn is_lodading_meshes(&self) -> bool {
        self.load_mesh_thread.is_some()
    }

    pub fn meshes_loaded(&self) -> bool {
        self.meshes.is_some()
    }

    pub fn unload(&mut self) {
        self.data = None;
        self.meshes = None;
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

    pub fn load_data_block(
        &mut self,
    ) -> Result<Result<Arc<Definition>, SwBlockDefinitionDataError>, mpsc::RecvError> {
        if let Some(data) = &self.data {
            Ok(data.clone())
        } else {
            if self.load_data_thread.is_none() {
                self.spawn_load_data();
            }
            let rx = self.load_data_thread.as_ref().unwrap();
            let data = rx.recv()?.map(Arc::new);
            self.data = Some(data.clone());
            self.load_data_thread = None;
            Ok(data)
        }
    }

    pub fn data(&self) -> Option<Result<Arc<Definition>, SwBlockDefinitionDataError>> {
        self.data.clone()
    }

    pub fn load_meshes(&mut self) -> Option<Arc<SwBlockMeshes>> {
        if let Some(rx) = &self.load_mesh_thread {
            if let Ok(meshes) = rx.try_recv() {
                self.meshes = Some(Arc::new(meshes));
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

    pub fn load_meshes_block(&mut self) -> Result<Arc<SwBlockMeshes>, mpsc::RecvError> {
        if let Some(meshes) = &self.meshes {
            Ok(meshes.clone())
        } else {
            if self.load_mesh_thread.is_none() {
                self.spawn_load_meshes();
            }
            let rx = self.load_mesh_thread.as_ref().unwrap();
            let meshes = Arc::new(rx.recv()?);
            self.meshes = Some(meshes.clone());
            self.load_mesh_thread = None;
            Ok(meshes)
        }
    }

    pub fn load_data_meshes(&mut self) -> (Option<Arc<Definition>>, Option<Arc<SwBlockMeshes>>) {
        (self.load_data().and_then(|d| d.ok()), self.load_meshes())
    }

    pub fn search(&mut self, search_text: &str) -> Option<bool> {
        let search_text = search_text.to_lowercase();
        if self.filename.to_lowercase().contains(&search_text) {
            return Some(true);
        }
        if let Ok(data) = self.load_data()? {
            for field in [&data.name, &data.tags].into_iter().flatten() {
                if field.to_lowercase().contains(&search_text) {
                    return Some(true);
                }
            }
        }
        Some(false)
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

            let (tx, rx) = mpsc::channel();
            thread::spawn(move || {
                tx.send(SwBlockMeshes::new(&data, ModFileLoader::from_mod_key(self.mod_key)))
                    .unwrap();
            });

            self.load_mesh_thread = Some(rx);
        }
    }
}

fn load_data<P: AsRef<Path>>(path: P) -> LoadDataResult {
    let xml = std::fs::read_to_string(path)?;

    if let Err(mes) = check_xml_root_tag(&xml, b"definition") {
        Err(SwBlockDefinitionDataError::Xml(mes))
    } else {
        let data = quick_xml::de::from_str(&xml)?;
        Ok(data)
    }
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
