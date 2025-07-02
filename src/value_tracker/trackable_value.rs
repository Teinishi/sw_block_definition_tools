use super::VersionCounter;

// 追跡可能トレイト
pub trait AttachVersion {
    fn attach_version(&mut self, version: &VersionCounter);
}

pub trait CheckUpdate {
    fn check_update(&self, last_version: &mut Option<u32>) -> bool;
}

// 追跡可能な値
#[derive(serde::Serialize, serde::Deserialize, Debug, Default, Clone)]
pub struct TrackableValue<T> {
    value: T,
    #[serde(skip)]
    version: VersionCounter,
}

impl<T> AttachVersion for TrackableValue<T> {
    fn attach_version(&mut self, version: &VersionCounter) {
        self.version = version.clone();
    }
}

impl<T> CheckUpdate for TrackableValue<T> {
    fn check_update(&self, last_version: &mut Option<u32>) -> bool {
        self.version.check_update(last_version)
    }
}

impl<T: PartialEq> TrackableValue<T> {
    pub fn new(value: T, version: VersionCounter) -> Self {
        Self { value, version }
    }

    pub fn inner(&self) -> &T {
        &self.value
    }

    #[allow(dead_code)]
    pub fn into_inner(self) -> T {
        self.value
    }

    #[allow(dead_code)]
    pub fn set(&mut self, new_value: T) {
        if self.value != new_value {
            self.value = new_value;
            self.version.bump();
        }
    }
}
