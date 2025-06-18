use super::{Definition, SwMesh, SwMeshResult};
use crate::gl_renderer::Mesh;
use glam::{Mat4, Vec3, Vec3Swizzles};
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

#[derive(Debug, serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub enum SwWheelAdvancedType {
    AllRound,
    HighSpeed,
    HighGrip,
}

impl SwWheelAdvancedType {
    fn mesh_key(&self) -> SwWheelAdvancedMeshKey {
        match self {
            Self::AllRound => SwWheelAdvancedMeshKey::M,
            Self::HighSpeed => SwWheelAdvancedMeshKey::H,
            Self::HighGrip => SwWheelAdvancedMeshKey::L,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct SwBlockMeshBuilder {
    pub show_meshes: BTreeSet<SwBlockMeshKey>,
    pub mesh_offset: BTreeMap<SwBlockMeshKey, Vec3>,
    pub show_child: bool,
    pub special_meshes: BTreeSet<SwBlockSpecialMesh>,
    pub propeller_blade_count: i32,
    pub wheel_advanced_type: SwWheelAdvancedType,
    pub wheel_advanced_size: f32,
    pub wheel_advanced_double: bool,
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
            wheel_advanced_type: SwWheelAdvancedType::AllRound,
            wheel_advanced_size: 1.0,
            wheel_advanced_double: false,
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
                        Mesh::from_sw_mesh(sw_mesh),
                        Mat4::from_translation(self.mesh_offset[key]),
                    ))
                } else {
                    None
                }
            })
            .collect();

        if let Some(special_mesh) = special_mesh {
            special_mesh.build(self, block_meshes, data, &mut result);
        }

        result
    }
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
    Wheel,
    TrainWheel,
    WheelAdvanced,
}

impl SwBlockSpecialMesh {
    pub fn from_definition_type(definition_type: i32) -> Option<Self> {
        match definition_type {
            2 => Some(Self::Propeller),
            4 => Some(Self::Wheel),
            36 => Some(Self::TrainWheel),
            41 => Some(Self::WheelAdvanced),
            _ => None,
        }
    }

    pub fn skip_mesh(&self, mesh_key: &SwBlockMeshKey) -> bool {
        match self {
            Self::Propeller => matches!(mesh_key, SwBlockMeshKey::Mesh1),
            Self::Wheel => matches!(mesh_key, SwBlockMeshKey::Mesh0),
            Self::TrainWheel => matches!(
                mesh_key,
                SwBlockMeshKey::Mesh0 | SwBlockMeshKey::Mesh1 | SwBlockMeshKey::Mesh2
            ),
            Self::WheelAdvanced => matches!(mesh_key, SwBlockMeshKey::Mesh0),
        }
    }

    fn build(
        &self,
        builder: &SwBlockMeshBuilder,
        block_meshes: &SwBlockMeshes,
        data: &Definition,
        result: &mut Vec<(Mesh, Mat4)>,
    ) {
        let show_mesh0 = builder.show_meshes.contains(&SwBlockMeshKey::Mesh0);
        let show_mesh1 = builder.show_meshes.contains(&SwBlockMeshKey::Mesh1);

        let transform_mesh0 = Mat4::from_translation(builder.mesh_offset[&SwBlockMeshKey::Mesh0]);
        let transform_mesh1 = Mat4::from_translation(builder.mesh_offset[&SwBlockMeshKey::Mesh1]);

        match self {
            Self::Propeller if show_mesh1 => {
                // プロペラ・ローター系のブレード
                if let Some(Ok(mesh1)) = block_meshes.meshes.get(&SwBlockMeshKey::Mesh1) {
                    let count = builder.propeller_blade_count;
                    result.extend((0..count).map(|i| {
                        let angle = (i as f32 / count as f32) * 2.0 * std::f32::consts::PI;
                        (
                            Mesh::from_sw_mesh(mesh1),
                            transform_mesh1.mul_mat4(&Mat4::from_rotation_y(angle)),
                        )
                    }));
                }
            }
            Self::Wheel if show_mesh0 => {
                // 旧タイヤ
                if let Some(Ok(mesh0)) = block_meshes.meshes.get(&SwBlockMeshKey::Mesh0) {
                    let position = data
                        .constraint_pos_parent
                        .last()
                        .map(|v| std::convert::Into::<Vec3>::into(*v))
                        .unwrap_or_default();

                    result.push((
                        Mesh::from_sw_mesh(mesh0),
                        transform_mesh0.mul_mat4(&Mat4::from_translation(position)),
                    ));
                }
            }
            Self::TrainWheel if show_mesh0 => {
                // 鉄道車輪の車軸
                if let Some(Ok(mesh0)) = block_meshes.meshes.get(&SwBlockMeshKey::Mesh0) {
                    let count = data.door_side_dist.unwrap_or(1);
                    let offset_y = data.wheel_suspension_height.unwrap_or_default();
                    let offset_x_vec: Vec<f32> = match count {
                        1 => vec![0.0],
                        2 => vec![1.05, -1.05],
                        3 => vec![2.122, 0.0, -2.122],
                        _ => vec![],
                    };
                    result.extend(offset_x_vec.iter().map(|offset_x| {
                        let offset = Vec3::new(*offset_x, *offset_y, 0.0);
                        (
                            Mesh::from_sw_mesh(mesh0),
                            transform_mesh0.mul_mat4(&Mat4::from_translation(offset)),
                        )
                    }));
                }
            }
            Self::WheelAdvanced if show_mesh0 => {
                // タイヤ
                let no_suspension = (data.flags.unwrap_or(0) >> 23) & 1 != 0;
                let wheel_type = data.wheel_type.unwrap_or(0);
                let radius = notnan_unwrap(data.wheel_radius, 0.0);
                let width = notnan_unwrap(data.wheel_width, 0.0);
                let suspension_offset = notnan_unwrap(data.wheel_suspension_offset, 0.25);
                let suspension_height = notnan_unwrap(data.wheel_suspension_height, 1.0);
                let wishbone_offset = notnan_unwrap(data.wheel_wishbone_offset, 0.0);
                let wishbone_length = notnan_unwrap(data.wheel_wishbone_length, 1.25);
                let wishbone_margin = notnan_unwrap(data.wheel_wishbone_margin, 0.085);

                if wheel_type != 0 {
                    if let Some(Ok(mesh0)) = block_meshes.meshes.get(&SwBlockMeshKey::Mesh0) {
                        // 履帯系
                        let position = data
                            .constraint_pos_parent
                            .last()
                            .map(|v| std::convert::Into::<Vec3>::into(*v))
                            .unwrap_or_default();
                        result.push((
                            Mesh::from_sw_mesh(mesh0),
                            transform_mesh0.mul_mat4(&Mat4::from_translation(position)),
                        ));
                    }
                } else {
                    let wheel_mesh_key = builder.wheel_advanced_type.mesh_key();
                    let wheel_scale = (radius != 0.0)
                        .then(|| 1.0 + 0.125 * (builder.wheel_advanced_size - 1.0) / radius);

                    let wishbone_offset_vec = Vec3::new(0.0, -0.125, -0.125 - wishbone_offset);
                    let suspension_pivot_1 = wishbone_offset_vec
                        + Vec3::new(0.0, suspension_offset, suspension_height - wishbone_margin);
                    let suspension_pivot_2 = wishbone_offset_vec
                        + Vec3::new(0.0, wishbone_length - wishbone_margin, wishbone_margin);
                    let suspension_pivot_3 = wishbone_offset_vec
                        + Vec3::new(
                            0.0,
                            wishbone_length - 2.0 * wishbone_margin,
                            wishbone_margin,
                        );

                    let suspension_angle =
                        glam::Vec2::Y.angle_to((suspension_pivot_3 - suspension_pivot_1).zy());
                    let suspension_rotation = Mat4::from_rotation_x(-suspension_angle);

                    if let Some(Ok(mesh_m)) =
                        block_meshes.wheel_advanced_meshes.get(&wheel_mesh_key)
                    {
                        // タイヤ本体
                        let position = data
                            .constraint_pos_parent
                            .last()
                            .map(|v| std::convert::Into::<Vec3>::into(*v))
                            .unwrap_or_default()
                            - wishbone_offset * Vec3::Z;

                        if let Some(scale) = wheel_scale {
                            result.push((
                            Mesh::from_sw_mesh(mesh_m),
                                transform_mesh0
                                    .mul_mat4(&Mat4::from_translation(position))
                                    .mul_mat4(&Mat4::from_scale(scale * Vec3::ONE)),
                            ));
                            if builder.wheel_advanced_double {
                                result.push((
                            Mesh::from_sw_mesh(mesh_m),
                                    transform_mesh0
                                        .mul_mat4(&Mat4::from_translation(
                                            position + width * scale * Vec3::Y,
                                        ))
                                        .mul_mat4(&Mat4::from_scale(scale * Vec3::ONE)),
                                ));
                            }
                        }
                    }

                    if !no_suspension {
                        if let Some(Ok(mesh_plate)) = block_meshes
                            .wheel_advanced_meshes
                            .get(&SwWheelAdvancedMeshKey::Plate)
                        {
                            // タイヤ裏のプレート
                            result.push((Mesh::from_sw_mesh(mesh_plate), transform_mesh0));
                        }

                        if let Some(Ok(mesh_sus_base)) = block_meshes
                            .wheel_advanced_meshes
                            .get(&SwWheelAdvancedMeshKey::SusBase)
                        {
                            // サスペンションの基部側
                            let transform = Mat4::from_translation(suspension_pivot_1)
                                .mul_mat4(&suspension_rotation);
                            result.push((
                                Mesh::from_sw_mesh(mesh_sus_base),
                                transform_mesh0.mul_mat4(&transform),
                            ));
                        }

                        if let Some(Ok(mesh_sus_spring)) = block_meshes
                            .wheel_advanced_meshes
                            .get(&SwWheelAdvancedMeshKey::SusSpring)
                        {
                            // サスペンションのタイヤ側
                            let transform = Mat4::from_translation(suspension_pivot_2)
                                .mul_mat4(&suspension_rotation);
                            result.push((
                                Mesh::from_sw_mesh(mesh_sus_spring),
                                transform_mesh0.mul_mat4(&transform),
                            ));
                        }

                        if let Some(Ok(mesh_wishbone)) = block_meshes
                            .wheel_advanced_meshes
                            .get(&SwWheelAdvancedMeshKey::Wishbone)
                        {
                            // 基部とプレートを繋ぐパーツ
                            let transform = Mat4::from_translation(
                                wishbone_offset_vec
                                    + Vec3::new(0.0, wishbone_margin, wishbone_margin),
                            );
                            result.push((
                                Mesh::from_sw_mesh(mesh_wishbone),
                                transform_mesh0.mul_mat4(&transform),
                            ));
                        }
                    }
                }
            }
            _ => {}
        }
    }
}

fn notnan_unwrap<T: ordered_float::FloatCore>(
    value: Option<ordered_float::NotNan<T>>,
    default: T,
) -> T {
    value.map(|v| *v).unwrap_or(default)
}
