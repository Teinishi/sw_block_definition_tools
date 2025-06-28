use crate::{
    store::{ModKey, SwBlockDefinition},
    sw_schema_lib::Mod,
    utils::check_xml_root_tag,
};
use std::{
    cell::RefCell,
    collections::BTreeMap,
    fmt,
    fs::read_dir,
    io,
    path::{Path, PathBuf},
    rc::Rc,
    sync::{mpsc, Arc, Mutex, Weak},
    thread,
};

pub type DefinitionsMap = Rc<RefCell<BTreeMap<String, DefinitionPointer>>>;
pub type DefinitionPointer = Arc<Mutex<SwBlockDefinition>>;
pub type WeakDefinitionPointer = Weak<Mutex<SwBlockDefinition>>;
type LoadManifestResult = Result<Mod, SwModManifestError>;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct SwModDefinition {
    path: PathBuf,
    folder_name: String,
    #[serde(skip)]
    manifest: Option<Result<Arc<Mod>, SwModManifestError>>,
    #[serde(skip)]
    definitions: DefinitionsMap,
    #[serde(skip)]
    load_manifest_thread: Option<mpsc::Receiver<LoadManifestResult>>,
}

impl SwModDefinition {
    pub fn new<P: AsRef<Path>>(path: P) -> Option<Self> {
        let pathbuf = path.as_ref().to_path_buf();
        let folder_name = pathbuf.file_name()?.to_os_string().into_string().ok()?;
        Some(Self {
            path: path.as_ref().to_path_buf(),
            folder_name,
            manifest: None,
            definitions: Default::default(),
            load_manifest_thread: None,
        })
    }

    pub fn folder_name(&self) -> &str {
        &self.folder_name
    }

    pub fn is_loading(&self) -> bool {
        self.load_manifest_thread.is_some()
    }

    pub fn get_definition(&self, name: &str) -> Option<DefinitionPointer> {
        self.definitions
            .borrow()
            .get(&format!("{}.xml", name))
            .cloned()
    }

    pub fn unload(&mut self) {
        self.manifest = None;
    }

    pub fn load_manifest(&mut self) -> Option<Result<Arc<Mod>, SwModManifestError>> {
        if let Some(rx) = &self.load_manifest_thread {
            if let Ok(r) = rx.try_recv() {
                match r {
                    Ok(data) => {
                        self.manifest = Some(Ok(Arc::new(data)));
                    }
                    Err(err) => {
                        self.manifest = Some(Err(err));
                    }
                }
                self.load_manifest_thread = None;
            }
        }

        if let Some(manifest) = &self.manifest {
            Some(manifest.clone())
        } else {
            self.spawn_load_manifest();
            None
        }
    }

    pub fn load_definitions(&mut self, mod_key: ModKey) -> io::Result<()> {
        for entry in read_dir(self.path.join("data\\definitions"))? {
            if let Some(entry_path) = entry
                .ok()
                .map(|e| e.path())
                .filter(|e| e.is_file() && e.extension().is_some_and(|x| x == "xml"))
            {
                if let Some(def) = SwBlockDefinition::new(mod_key.clone(), entry_path) {
                    self.definitions
                        .borrow_mut()
                        .insert(def.filename().to_string(), Arc::new(Mutex::new(def)));
                }
            }
        }

        Ok(())
    }

    fn spawn_load_manifest(&mut self) {
        if self.load_manifest_thread.is_some() {
            return;
        }

        let manifest_path = self.path.join("mod.xml");

        let (tx, rx) = mpsc::channel();
        thread::spawn(move || {
            tx.send(load_manifest(manifest_path)).unwrap_or_default();
        });

        self.load_manifest_thread = Some(rx);
    }
}

fn load_manifest<P: AsRef<Path>>(path: P) -> LoadManifestResult {
    let xml = std::fs::read_to_string(path)?;

    if let Err(mes) = check_xml_root_tag(&xml, b"mod") {
        Err(SwModManifestError::Xml(mes))
    } else {
        let data = quick_xml::de::from_str(&xml)?;
        Ok(data)
    }
}

#[derive(Debug, Clone)]
pub enum SwModManifestError {
    Io(String),
    De(String),
    Xml(String),
}

impl From<io::Error> for SwModManifestError {
    fn from(value: io::Error) -> Self {
        Self::Io(value.to_string())
    }
}

impl From<quick_xml::DeError> for SwModManifestError {
    fn from(value: quick_xml::DeError) -> Self {
        Self::De(value.to_string())
    }
}

impl fmt::Display for SwModManifestError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(mes) => write!(f, "IoError: {}", mes),
            Self::De(mes) => write!(f, "DeError: {}", mes),
            Self::Xml(mes) => write!(f, "XmlError: {}", mes),
        }
    }
}

impl std::error::Error for SwModManifestError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        None
    }
}
