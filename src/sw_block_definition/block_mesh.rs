use super::{Definition, SwMesh, SwMeshResult};
use crate::gl_renderer::Mesh;
use glam::{Mat4, Vec3};
use std::{
    collections::{BTreeMap, BTreeSet},
    path::Path,
};
use strum::VariantArray;

#[derive(
    Debug,
    serde::Deserialize,
    serde::Serialize,
    strum::VariantArray,
    enum_map::Enum,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum SwBlockMeshKey {
    MeshData,
    Mesh0,
    Mesh1,
    Mesh2,
    MeshEditorOnly,
}

impl SwBlockMeshKey {
    pub fn xml_name(&self) -> &str {
        match self {
            Self::MeshData => "mesh_data_name",
            Self::Mesh0 => "mesh_0_name",
            Self::Mesh1 => "mesh_1_name",
            Self::Mesh2 => "mesh_2_name",
            Self::MeshEditorOnly => "mesh_editor_only_name",
        }
    }

    pub fn ui_name(&self) -> &str {
        match self {
            Self::MeshData => "Mesh data",
            Self::Mesh0 => "Mesh 0",
            Self::Mesh1 => "Mesh 1",
            Self::Mesh2 => "Mesh 2",
            Self::MeshEditorOnly => "Mesh editor only",
        }
    }

    pub fn get_filepath<'a>(&'_ self, data: &'a Definition) -> &'a Option<String> {
        match self {
            Self::MeshData => &data.mesh_data_name,
            Self::Mesh0 => &data.mesh_0_name,
            Self::Mesh1 => &data.mesh_1_name,
            Self::Mesh2 => &data.mesh_2_name,
            Self::MeshEditorOnly => &data.mesh_editor_only_name,
        }
    }
}

#[derive(
    serde::Deserialize,
    serde::Serialize,
    strum::VariantArray,
    Clone,
    Copy,
    PartialEq,
    PartialOrd,
    Eq,
    Ord,
)]
enum SwWheelAdvancedMeshKey {
    Base,
    Plate,
    M,
    L,
    H,
    SusBase,
    SusSpring,
    Wishbone,
}

impl SwWheelAdvancedMeshKey {
    pub fn name(&self) -> &str {
        match self {
            Self::Base => "base",
            Self::Plate => "plate",
            Self::M => "m",
            Self::L => "l",
            Self::H => "h",
            Self::SusBase => "sus_base",
            Self::SusSpring => "sus_spring",
            Self::Wishbone => "wishbone",
        }
    }

    pub fn get_filepath(&self, data: &Definition) -> Option<String> {
        data.mesh_0_name
            .as_ref()
            .map(|name| format!("{}_{}.mesh", name, self.name()))
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SwBlockMeshBuilder {
    pub show_meshes: BTreeSet<SwBlockMeshKey>,
    pub mesh_offset: BTreeMap<SwBlockMeshKey, Vec3>,
    pub show_child: bool,
    pub special_meshes: BTreeSet<SwBlockSpecialMesh>,
    pub propeller_blade_count: i32,
}

impl Default for SwBlockMeshBuilder {
    fn default() -> Self {
        Self {
            show_meshes: BTreeSet::from_iter(SwBlockMeshKey::VARIANTS.iter().cloned()),
            mesh_offset: BTreeMap::from_iter(
                SwBlockMeshKey::VARIANTS
                    .iter()
                    .map(|key| (*key, Vec3::ZERO)),
            ),
            show_child: true,
            special_meshes: BTreeSet::from_iter(SwBlockSpecialMesh::VARIANTS.iter().cloned()),
            propeller_blade_count: 4,
        }
    }
}

impl SwBlockMeshBuilder {
    pub fn build(&self, block_meshes: &SwBlockMeshes, data: &Definition) -> Vec<(Mesh, Mat4)> {
        let special_mesh = block_meshes
            .special_mesh
            .filter(|s| self.special_meshes.contains(s));

        let mut result: Vec<(Mesh, Mat4)> = self
            .show_meshes
            .iter()
            .filter_map(|key| {
                // 通常の mesh
                if special_mesh.map(|s| s.skip_mesh(key)).unwrap_or(false) {
                    None
                } else if let Some(Ok(sw_mesh)) = &block_meshes.meshes.get(key) {
                    Some((
                        sw_mesh.as_combined_mesh(),
                        Mat4::from_translation(self.mesh_offset[key]),
                    ))
                } else {
                    None
                }
            })
            .collect();

        let mesh_map: BTreeMap<SwBlockMeshKey, &SwMesh> =
            BTreeMap::from_iter(self.show_meshes.iter().filter_map(|key| {
                block_meshes
                    .meshes
                    .get(key)
                    .and_then(|r| r.as_ref().ok())
                    .map(|sw_mesh| (*key, sw_mesh))
            }));

        match special_mesh {
            Some(SwBlockSpecialMesh::Propeller) => {
                // プロペラ・ローター系のブレード
                if let Some(mesh1) = mesh_map.get(&SwBlockMeshKey::Mesh1) {
                    let transform_mesh1 =
                        Mat4::from_translation(self.mesh_offset[&SwBlockMeshKey::Mesh1]);
                    result.extend(
                        build_propeller(self.propeller_blade_count, mesh1.as_combined_mesh())
                            .into_iter()
                            .map(|(mesh, transform)| (mesh, transform_mesh1.mul_mat4(&transform))),
                    );
                }
            }
            Some(SwBlockSpecialMesh::TrainWheel) => {
                // 鉄道車輪の車軸
                if let Some(mesh0) = mesh_map.get(&SwBlockMeshKey::Mesh0) {
                    let count = data.door_side_dist.unwrap_or(1);
                    let offset_y = data.wheel_suspension_height.unwrap_or_default();
                    let offset_x_vec: Vec<f32> = match count {
                        1 => vec![0.0],
                        2 => vec![1.05, -1.05],
                        3 => vec![2.122, 0.0, -2.122],
                        _ => vec![],
                    };
                    let transform_mesh0 =
                        Mat4::from_translation(self.mesh_offset[&SwBlockMeshKey::Mesh0]);
                    result.extend(offset_x_vec.iter().map(|offset_x| {
                        let offset = Vec3::new(*offset_x, *offset_y, 0.0);
                        (
                            mesh0.as_combined_mesh(),
                            transform_mesh0.mul_mat4(&Mat4::from_translation(offset)),
                        )
                    }));
                }
            }
            Some(SwBlockSpecialMesh::WheelAdvanced) => {}
            _ => {}
        }

        result
    }
}

fn build_propeller(count: i32, mesh1: Mesh) -> Vec<(Mesh, Mat4)> {
    // プロペラ・ローター系のブレード
    (0..count)
        .map(|i| {
            let angle = (i as f32 / count as f32) * 2.0 * std::f32::consts::PI;
            (mesh1.clone(), Mat4::from_rotation_y(angle))
        })
        .collect()
}

#[derive(Default)]
pub struct SwBlockMeshes {
    special_mesh: Option<SwBlockSpecialMesh>,
    meshes: BTreeMap<SwBlockMeshKey, SwMeshResult>,
    wheel_advanced_meshes: BTreeMap<SwWheelAdvancedMeshKey, SwMeshResult>,
    child_name: Option<String>,
}

impl SwBlockMeshes {
    pub fn new<P: AsRef<Path>>(data: &Definition, rom_path: P) -> Self {
        let special_mesh =
            SwBlockSpecialMesh::from_definition_type(data.definition_type.unwrap_or(0));

        let mut meshes = BTreeMap::new();
        for key in SwBlockMeshKey::VARIANTS {
            if let Some(name) = key.get_filepath(data) {
                if !name.is_empty() {
                    meshes.insert(*key, SwMesh::from_file(rom_path.as_ref().join(name)));
                }
            }
        }

        let mut wheel_advanced_meshes = BTreeMap::new();
        if special_mesh == Some(SwBlockSpecialMesh::WheelAdvanced) {
            for key in SwWheelAdvancedMeshKey::VARIANTS {
                if let Some(path) = key.get_filepath(data) {
                    wheel_advanced_meshes
                        .insert(*key, SwMesh::from_file(rom_path.as_ref().join(path)));
                }
            }
        }

        let child_name = data.child_name.clone();

        Self {
            special_mesh,
            meshes,
            wheel_advanced_meshes,
            child_name,
        }
    }

    pub fn builder_type(&self) -> Option<SwBlockSpecialMesh> {
        self.special_mesh
    }

    pub fn has_mesh(&self, key: &SwBlockMeshKey) -> bool {
        self.meshes.contains_key(key)
    }

    pub fn has_child(&self) -> bool {
        self.child_name
            .as_ref()
            .map(|n| !n.is_empty())
            .unwrap_or(false)
    }

    pub fn skip_mesh(&self, mesh_key: &SwBlockMeshKey) -> bool {
        self.special_mesh
            .map(|s| s.skip_mesh(mesh_key))
            .unwrap_or(false)
    }
}

#[derive(
    serde::Serialize,
    serde::Deserialize,
    strum::VariantArray,
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
)]
pub enum SwBlockSpecialMesh {
    Propeller,
    TrainWheel,
    WheelAdvanced,
}

impl SwBlockSpecialMesh {
    pub fn from_definition_type(definition_type: i32) -> Option<Self> {
        match definition_type {
            2 => Some(Self::Propeller),
            36 => Some(Self::TrainWheel),
            41 => Some(Self::WheelAdvanced),
            _ => None,
        }
    }

    pub fn skip_mesh(&self, mesh_key: &SwBlockMeshKey) -> bool {
        match self {
            Self::Propeller => matches!(mesh_key, SwBlockMeshKey::Mesh1),
            Self::TrainWheel => matches!(
                mesh_key,
                SwBlockMeshKey::Mesh0 | SwBlockMeshKey::Mesh1 | SwBlockMeshKey::Mesh2
            ),
            Self::WheelAdvanced => matches!(mesh_key, SwBlockMeshKey::Mesh0),
        }
    }
}
