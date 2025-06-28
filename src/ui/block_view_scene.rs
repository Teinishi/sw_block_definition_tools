use super::{
    utils::{
        ui_checkbox_btreeset, ui_color_picker_rgb, ui_color_picker_rgba, ui_dragvalue_vec_z_inv,
    },
    BoundingBoxObjectBuilder,
};
use crate::{
    definition_hub::{BlockDefinition, DefinitionRegistory},
    sw_block_definition::{Definition, DefinitionVec3},
    sw_gl_3d::{
        Color4, Line, Scene, SceneObject, SurfaceObjectBuilder, SwBlockMeshBuilder, SwBlockMeshKey,
        SwBlockMeshes, SwBlockSpecialMesh, SwWheelAdvancedType,
    },
};
use core::f32;
use egui::{DragValue, Grid, Slider};
use glam::{Mat4, Vec3};
use std::{
    collections::BTreeSet,
    fmt::Debug,
    sync::{Arc, Mutex, MutexGuard},
};
use strum::VariantArray;

const BUOYANCY_SURFACE_MESH_COLOR: Color4 = Color4 {
    r: 0.1,
    g: 0.35,
    b: 0.5,
    a: 0.3,
};
const BUOYANCY_SURFACE_LINE_COLOR: Color4 = Color4 {
    r: 0.2,
    g: 0.7,
    b: 1.0,
    a: 1.0,
};
const BOUNDING_BOX_VOXEL_MESH_COLOR: Color4 = Color4 {
    r: 0.5,
    g: 0.1,
    b: 0.1,
    a: 0.3,
};
const BOUNDING_BOX_VOXEL_LINE_COLOR: Color4 = Color4 {
    r: 1.0,
    g: 0.2,
    b: 0.2,
    a: 1.0,
};
const BOUNDING_BOX_VOXEL_PHYSICS_MESH_COLOR: Color4 = Color4 {
    r: 0.4,
    g: 0.3,
    b: 0.0,
    a: 0.3,
};
const BOUNDING_BOX_VOXEL_PHYSICS_LINE_COLOR: Color4 = Color4 {
    r: 0.8,
    g: 0.6,
    b: 0.0,
    a: 1.0,
};
const BOUNDING_BOX_PHYSICS_MESH_COLOR: Color4 = Color4 {
    r: 0.0,
    g: 0.4,
    b: 0.1,
    a: 0.3,
};
const BOUNDING_BOX_PHYSICS_LINE_COLOR: Color4 = Color4 {
    r: 0.0,
    g: 0.8,
    b: 0.2,
    a: 1.0,
};

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
pub struct BlockViewState {
    pub show_xyz_axes: bool,
    pub show_surfaces: bool,
    pub show_surface_edges: bool,
    pub show_buoyancy_surfaces: bool,
    pub show_bounding_box_voxel: bool,
    pub show_bounding_box_voxel_physics: bool,
    pub show_bounding_box_physics: bool,
    pub mesh_builder: SwBlockMeshBuilder,
}

impl Default for BlockViewState {
    fn default() -> Self {
        Self {
            show_xyz_axes: true,
            show_surfaces: true,
            show_surface_edges: true,
            show_buoyancy_surfaces: false,
            show_bounding_box_voxel: false,
            show_bounding_box_voxel_physics: false,
            show_bounding_box_physics: false,
            mesh_builder: SwBlockMeshBuilder::default(),
        }
    }
}

impl BlockViewState {
    pub fn show_child_body(&self) -> bool {
        self.mesh_builder.show_child
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, mesh_options: &BlockViewStateMeshOptions) -> bool {
        let before_change = self.clone();

        ui.checkbox(&mut self.show_xyz_axes, "XYZ axes");
        ui.checkbox(&mut self.show_surfaces, "Surfaces");
        ui.checkbox(&mut self.show_surface_edges, "Surface edge lines");
        ui.checkbox(&mut self.show_buoyancy_surfaces, "Buoyancy surfaces");
        ui.checkbox(&mut self.show_bounding_box_voxel, "Bounding box (voxel)");
        ui.checkbox(
            &mut self.show_bounding_box_voxel_physics,
            "Bounding box (voxel physics)",
        );
        ui.checkbox(
            &mut self.show_bounding_box_physics,
            "Bounding box (physics)",
        );

        if !mesh_options.meshes.is_empty() {
            ui.separator();
        }
        for key in &mesh_options.meshes {
            let mut checked = self.mesh_builder.show_meshes.contains(key);
            ui.checkbox(&mut checked, key.ui_name());
            if checked {
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui_dragvalue_vec_z_inv(
                        ui,
                        self.mesh_builder.mesh_offset.get_mut(key).unwrap(),
                        0.01,
                    );
                });
                self.mesh_builder.show_meshes.insert(*key);
            } else {
                self.mesh_builder.show_meshes.remove(key);
            }
        }

        let propeller = mesh_options.propeller();
        let wheel_old = mesh_options.wheel_old();
        let train_wheel = mesh_options.train_wheel();
        let wheel_advanced = mesh_options.wheel_advanced();
        let child = mesh_options.child();

        if propeller || train_wheel || wheel_advanced || child {
            ui.separator();
        }
        if propeller {
            let checked = ui_checkbox_btreeset(
                ui,
                &mut self.mesh_builder.special_meshes,
                SwBlockSpecialMesh::Propeller,
                "Propeller mode",
            );
            if checked {
                ui.horizontal(|ui| {
                    ui.add_space(20.0);
                    ui.add(
                        DragValue::new(&mut self.mesh_builder.propeller_blade_count)
                            .range(2..=8)
                            .speed(0.1),
                    );
                    ui.label("Blades");
                });
            }
        }
        if wheel_old {
            ui_checkbox_btreeset(
                ui,
                &mut self.mesh_builder.special_meshes,
                SwBlockSpecialMesh::Wheel,
                "Wheel (old) mode",
            );
        }
        if train_wheel {
            ui_checkbox_btreeset(
                ui,
                &mut self.mesh_builder.special_meshes,
                SwBlockSpecialMesh::TrainWheel,
                "Train wheel mode",
            );
        }
        if wheel_advanced {
            let checked = ui_checkbox_btreeset(
                ui,
                &mut self.mesh_builder.special_meshes,
                SwBlockSpecialMesh::WheelAdvanced,
                "Wheel mode",
            );
            if checked {
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.add_space(20.0);
                    ui.selectable_value(
                        &mut self.mesh_builder.wheel_advanced_type,
                        SwWheelAdvancedType::AllRound,
                        "All round",
                    );
                    ui.selectable_value(
                        &mut self.mesh_builder.wheel_advanced_type,
                        SwWheelAdvancedType::HighSpeed,
                        "High speed",
                    );
                    ui.selectable_value(
                        &mut self.mesh_builder.wheel_advanced_type,
                        SwWheelAdvancedType::HighGrip,
                        "High grip",
                    );
                });
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.add_space(20.0);
                    ui.label("Tyre radius");
                    ui.add(
                        Slider::new(&mut self.mesh_builder.wheel_advanced_size, 0.0..=2.0)
                            .step_by(0.5),
                    );
                });
                ui.horizontal(|ui: &mut egui::Ui| {
                    ui.add_space(20.0);
                    ui.checkbox(&mut self.mesh_builder.wheel_advanced_double, "Double wheel");
                });
            }
        }
        if child {
            ui.checkbox(&mut self.mesh_builder.show_child, "Child body");
        }

        Self::ne(self, &before_change)
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq)]
pub struct BlockViewAppearance {
    pub surface: Color4,
    pub override_color: bool,
    pub override_1: Color4,
    pub override_2: Color4,
    pub override_3: Color4,
    pub additive: Color4,
    pub buoyancy_surface: (Color4, Color4, f32),
    pub bounding_box_voxel: (Color4, Color4, f32),
    pub bounding_box_voxel_physics: (Color4, Color4, f32),
    pub bounding_box_physics: (Color4, Color4, f32),
}

impl Default for BlockViewAppearance {
    fn default() -> Self {
        Self {
            surface: Color4::WHITE,
            override_color: true,
            override_1: Color4::WHITE,
            override_2: Color4::WHITE,
            override_3: Color4::WHITE,
            additive: Color4::WHITE,
            buoyancy_surface: (
                BUOYANCY_SURFACE_MESH_COLOR,
                BUOYANCY_SURFACE_LINE_COLOR,
                4.0,
            ),
            bounding_box_voxel: (
                BOUNDING_BOX_VOXEL_MESH_COLOR,
                BOUNDING_BOX_VOXEL_LINE_COLOR,
                4.0,
            ),
            bounding_box_voxel_physics: (
                BOUNDING_BOX_VOXEL_PHYSICS_MESH_COLOR,
                BOUNDING_BOX_VOXEL_PHYSICS_LINE_COLOR,
                4.0,
            ),
            bounding_box_physics: (
                BOUNDING_BOX_PHYSICS_MESH_COLOR,
                BOUNDING_BOX_PHYSICS_LINE_COLOR,
                4.0,
            ),
        }
    }
}

impl BlockViewAppearance {
    fn ui(&mut self, ui: &mut egui::Ui, id: egui::Id, state: &BlockViewState) -> bool {
        let before_change = self.clone();

        Grid::new(id).spacing([10.0, 8.0]).show(ui, |ui| {
            ui.label("Surface color");
            ui_color_picker_rgb(ui, &mut self.surface);
            ui.end_row();

            ui.checkbox(&mut self.override_color, "Override color");
            ui.end_row();

            if self.override_color {
                ui.label("Override color 1");
                ui_color_picker_rgb(ui, &mut self.override_1);
                ui.end_row();

                ui.label("Override color 2");
                ui_color_picker_rgb(ui, &mut self.override_2);
                ui.end_row();

                ui.label("Override color 3");
                ui_color_picker_rgb(ui, &mut self.override_3);
                ui.end_row();
            }

            ui.label("Additive color");
            ui_color_picker_rgb(ui, &mut self.additive);
            ui.end_row();

            for (show, text, appearance) in [
                (
                    state.show_buoyancy_surfaces,
                    "Buoyancy surfaces",
                    &mut self.buoyancy_surface,
                ),
                (
                    state.show_bounding_box_voxel,
                    "Bounding box (voxel)",
                    &mut self.bounding_box_voxel,
                ),
                (
                    state.show_bounding_box_voxel_physics,
                    "Bounding box (voxel physics)",
                    &mut self.bounding_box_voxel_physics,
                ),
                (
                    state.show_bounding_box_physics,
                    "Bounding box (physics)",
                    &mut self.bounding_box_physics,
                ),
            ] {
                if !show {
                    continue;
                }
                ui.label(text);
                ui_color_picker_rgba(ui, &mut appearance.0);
                ui_color_picker_rgba(ui, &mut appearance.1);
                ui.add(
                    DragValue::new(&mut appearance.2)
                        .range(0.0..=10.0)
                        .speed(0.1),
                );
                ui.end_row();
            }
        });

        Self::ne(self, &before_change)
    }
}

#[derive(Default)]
pub struct BlockViewStateMeshOptions {
    meshes: BTreeSet<SwBlockMeshKey>,
    special_meshes: BTreeSet<SwBlockSpecialMesh>,
    child: bool,
}

impl BlockViewStateMeshOptions {
    pub fn from_definition_meshes(
        block_meshes: &SwBlockMeshes,
        data: &Option<Arc<Definition>>,
    ) -> Self {
        let meshes = BTreeSet::from_iter(
            SwBlockMeshKey::VARIANTS
                .iter()
                .filter(|key| block_meshes.has_mesh(key))
                .cloned(),
        );

        let special_meshes = BTreeSet::from_iter(
            data.as_ref()
                .and_then(|d| d.definition_type)
                .and_then(SwBlockSpecialMesh::from_definition_type)
                .iter()
                .cloned(),
        );

        Self {
            meshes,
            special_meshes,
            child: block_meshes.has_child(),
        }
    }

    pub fn or(&mut self, other: &Self) {
        self.meshes.extend(other.meshes.iter().cloned());
        self.special_meshes
            .extend(other.special_meshes.iter().cloned());
        self.child = self.child || other.child;
    }

    fn propeller(&self) -> bool {
        self.special_meshes.contains(&SwBlockSpecialMesh::Propeller)
    }

    fn wheel_old(&self) -> bool {
        self.special_meshes.contains(&SwBlockSpecialMesh::Wheel)
    }

    fn train_wheel(&self) -> bool {
        self.special_meshes
            .contains(&SwBlockSpecialMesh::TrainWheel)
    }

    fn wheel_advanced(&self) -> bool {
        self.special_meshes
            .contains(&SwBlockSpecialMesh::WheelAdvanced)
    }

    fn child(&self) -> bool {
        self.child
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct BlockViewScene {
    #[serde(skip)]
    scene: Arc<Mutex<Scene>>,
    state: BlockViewState,
    appearance: BlockViewAppearance,
}

impl BlockViewScene {
    pub fn reset(&mut self) {
        self.clear();
        self.state = Default::default();
        self.appearance = Default::default();
    }

    pub fn clear(&mut self) {
        self.scene.lock().unwrap().clear();
    }

    pub fn clone_state(other: &Self) -> Self {
        Self {
            scene: Default::default(),
            state: other.state.clone(),
            appearance: other.appearance.clone(),
        }
    }

    pub fn color_mut<F: FnOnce(&'_ mut BlockViewAppearance)>(&mut self, writer: F) -> bool {
        let before_change = self.appearance.clone();
        writer(&mut self.appearance);
        before_change != self.appearance
    }

    pub fn state_ui(
        &mut self,
        ui: &mut egui::Ui,
        mesh_options: &BlockViewStateMeshOptions,
    ) -> bool {
        self.state.ui(ui, mesh_options)
    }

    pub fn appearance_ui(&mut self, ui: &mut egui::Ui, id: egui::Id) -> bool {
        self.appearance.ui(ui, id, &self.state)
    }

    pub fn scene(&self) -> Arc<Mutex<Scene>> {
        self.scene.clone()
    }

    pub fn state(&self) -> &BlockViewState {
        &self.state
    }

    pub fn set_state(&mut self, state: BlockViewState) {
        self.state = state;
    }

    pub fn appearance(&self) -> &BlockViewAppearance {
        &self.appearance
    }

    pub fn set_appearance(&mut self, appearance: BlockViewAppearance) {
        self.appearance = appearance;
    }

    fn use_scene<F: FnOnce(MutexGuard<'_, Scene>)>(&mut self, writer: F) {
        if let Ok(scene) = self.scene.lock() {
            writer(scene);
        }
    }

    fn add_object(&mut self, object: SceneObject) {
        self.use_scene(|mut scene| {
            scene.add_object(object);
        });
    }

    pub fn update(
        &mut self,
        definition: &BlockDefinition,
        registory: &mut DefinitionRegistory,
    ) -> bool {
        self.use_scene(|mut scene| {
            scene.clear();
        });

        if self.state.show_xyz_axes {
            for (direction, color) in [
                (Vec3::X, Color4::RED),
                (Vec3::Y, Color4::GREEN),
                (Vec3::Z, Color4::BLUE),
            ] {
                self.add_object(SceneObject::from_line(
                    Line::single_stroke_lh(vec![Vec3::ZERO, 100.0 * direction], color, 2.0, false),
                    None,
                ));
            }
        }

        let transform = Mat4::IDENTITY;
        let (data, meshes) = definition.load_data_meshes();
        let mut done = data.is_some() && meshes.is_some();
        self.add_block_objects(&data, &meshes, &transform);

        // 子パーツを追加
        if self.state.show_child_body() {
            if let Some(child) = data.and_then(|d| {
                d.child_name
                    .clone()
                    .and_then(|name| registory.resolve(definition.mod_key(), &name))
            }) {
                let (child_data, child_meshes) = child.load_data_meshes();
                done = done && child_data.is_some() && child_meshes.is_some();

                let mut translation = Vec3::ZERO;
                if let Some(ref child_data) = &child_data {
                    if let Some(v) = child_data.voxel_location_child.last() {
                        translation = std::convert::Into::<Vec3>::into(*v) * 0.25;
                    }
                }

                self.add_block_objects(
                    &child_data,
                    &child_meshes,
                    &Mat4::from_translation(translation).mul_mat4(&transform),
                );
            }
        }

        done
    }

    fn add_block_objects(
        &mut self,
        data: &Option<Arc<Definition>>,
        meshes: &Option<Arc<SwBlockMeshes>>,
        transform: &Mat4,
    ) {
        // meshを追加
        if let Some((data, block_meshes)) = data.as_ref().zip(meshes.as_ref()) {
            for (mesh, mesh_transform) in self.state.mesh_builder.build(block_meshes, data) {
                self.add_object(SceneObject::from_mesh(
                    mesh,
                    Some(transform.mul_mat4(&mesh_transform)),
                ));
            }
        }

        // surface を追加
        if let Some(data) = data {
            if let Some(surfaces) = data.surfaces.last() {
                for surface in &surfaces.surface {
                    let position = surface.position.last();

                    let mut obj_builder = SurfaceObjectBuilder::new(
                        surface.shape,
                        position,
                        surface.orientation,
                        surface.rotation,
                    );

                    if let Some(logic_nodes) = data.logic_nodes.last().map(|n| &n.logic_node) {
                        let vec_default = DefinitionVec3::<i32>::default();

                        let position = position.unwrap_or(&vec_default).as_array(0);
                        let orientation = surface.orientation.unwrap_or(0);

                        let node = logic_nodes.iter().find(|node| {
                            node.position.last().unwrap_or(&vec_default).as_array(0) == position
                                && node.orientation.unwrap_or(0) == orientation
                        });
                        match node.and_then(|n| n.node_type) {
                            Some(2) => obj_builder.power_node(),
                            Some(3) => obj_builder.fluid_node(),
                            _ => {}
                        }
                    }

                    let (mesh_obj, line_obj) = obj_builder.basic_objects(
                        self.state.show_surfaces,
                        self.state.show_surface_edges,
                        self.appearance.surface,
                    );
                    if let Some(obj) = mesh_obj {
                        self.add_object(obj.apply_transform_left(transform));
                    }
                    if let Some(obj) = line_obj {
                        self.add_object(obj.apply_transform_left(transform).set_z_offset(-1.0));
                    }
                }
            }

            // buoyancy surface を追加
            if self.state.show_buoyancy_surfaces {
                if let Some(buoyancy_surfaces) = data.buoyancy_surfaces.last() {
                    let (mesh_color, line_color, line_width) = self.appearance.buoyancy_surface;
                    for surface in &buoyancy_surfaces.surface {
                        let (mesh_obj, line_obj) = SurfaceObjectBuilder::new(
                            surface.shape,
                            surface.position.last(),
                            surface.orientation,
                            surface.rotation,
                        )
                        .translucent_objects(mesh_color, line_color, line_width);
                        if let Some(obj) = mesh_obj {
                            self.add_object(obj.apply_transform_left(transform).set_z_offset(-1.0));
                        }
                        if let Some(obj) = line_obj {
                            self.add_object(obj.apply_transform_left(transform).set_z_offset(-2.0));
                        }
                    }
                }
            }

            // voxel bouxing box を追加
            if self.state.show_bounding_box_voxel {
                if let Some((voxel_min, voxel_max)) =
                    data.voxel_min.last().zip(data.voxel_max.last())
                {
                    let (mesh_color, line_color, line_width) = self.appearance.bounding_box_voxel;
                    let (mesh_obj, line_obj) =
                        BoundingBoxObjectBuilder::from_voxel(*voxel_min, *voxel_max)
                            .objects(mesh_color, line_color, line_width);
                    self.add_object(mesh_obj.apply_transform_left(transform).set_z_offset(-4.0));
                    if let Some(line_obj) = line_obj {
                        self.add_object(
                            line_obj.apply_transform_left(transform).set_z_offset(-5.0),
                        );
                    }
                }
            }

            // voxel physics bouxing box を追加
            if self.state.show_bounding_box_voxel_physics {
                if let Some((voxel_physics_min, voxel_physics_max)) = data
                    .voxel_physics_min
                    .last()
                    .zip(data.voxel_physics_max.last())
                {
                    let (mesh_color, line_color, line_width) =
                        self.appearance.bounding_box_voxel_physics;
                    let (mesh_obj, line_obj) = BoundingBoxObjectBuilder::from_voxel(
                        *voxel_physics_min,
                        *voxel_physics_max,
                    )
                    .objects(mesh_color, line_color, line_width);
                    self.add_object(mesh_obj.apply_transform_left(transform).set_z_offset(-3.0));
                    if let Some(line_obj) = line_obj {
                        self.add_object(
                            line_obj.apply_transform_left(transform).set_z_offset(-4.0),
                        );
                    }
                }
            }

            // physics bouxing box を追加
            if self.state.show_bounding_box_physics {
                if let Some((bb_physics_min, bb_physics_max)) =
                    data.bb_physics_min.last().zip(data.bb_physics_max.last())
                {
                    let (mesh_color, line_color, line_width) = self.appearance.bounding_box_physics;
                    let (mesh_obj, line_obj) =
                        BoundingBoxObjectBuilder::new(*bb_physics_min, *bb_physics_max)
                            .objects(mesh_color, line_color, line_width);
                    self.add_object(mesh_obj.apply_transform_left(transform).set_z_offset(-2.0));
                    if let Some(line_obj) = line_obj {
                        self.add_object(
                            line_obj.apply_transform_left(transform).set_z_offset(-3.0),
                        );
                    }
                }
            }
        }
    }
}
