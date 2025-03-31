use crate::{
    gl_renderer::{Color4, Line, Scene, SceneObject},
    sw_block_definition::{
        BoundingBoxObjectBuilder, Definition, SurfaceObjectBuilder, SwBlockDefinitionMeshKey,
        SwBlockDefinitionMeshes,
    },
};
use egui::{DragValue, Grid};
use enum_map::EnumMap;
use glam::Vec3;
use std::{
    fmt::Debug,
    sync::{Arc, Mutex, MutexGuard},
};

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
    pub show_mesh: EnumMap<SwBlockDefinitionMeshKey, bool>,
}

impl Default for BlockViewState {
    fn default() -> Self {
        let mut show_mesh = EnumMap::default();
        for (key, _) in show_mesh {
            show_mesh[key] = true;
        }
        Self {
            show_xyz_axes: true,
            show_surfaces: true,
            show_surface_edges: true,
            show_buoyancy_surfaces: false,
            show_bounding_box_voxel: false,
            show_bounding_box_voxel_physics: false,
            show_bounding_box_physics: false,
            show_mesh,
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Clone, PartialEq)]
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

#[derive(Default)]
pub struct BlockViewStateMeshOptions {
    meshes: EnumMap<SwBlockDefinitionMeshKey, bool>,
}

impl Debug for BlockViewStateMeshOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut is_first = true;
        write!(f, "BlockViewStateMeshOptions {{")?;
        for (key, value) in self.meshes {
            if !is_first {
                write!(f, ", ")?;
            }
            write!(f, "{}: {}", key.xml_name(), value)?;
            is_first = false;
        }
        write!(f, "}}")
    }
}

impl BlockViewStateMeshOptions {
    pub fn from_definition_meshes(definition_meshes: &SwBlockDefinitionMeshes) -> Self {
        let mut meshes: EnumMap<SwBlockDefinitionMeshKey, bool> = Default::default();
        for (key, value) in meshes.iter_mut() {
            *value = definition_meshes.get_mesh(&key).is_some();
        }

        Self { meshes }
    }

    pub fn or(&mut self, other: &Self) {
        for (key, value) in self.meshes.iter_mut() {
            *value = *value || other.meshes[key];
        }
    }
}

#[derive(serde::Serialize, serde::Deserialize, Default)]
pub struct BlockViewScene {
    #[serde(skip)]
    scene: Arc<Mutex<Scene>>,
    state: BlockViewState,
    colors: BlockViewAppearance,
}

impl BlockViewScene {
    pub fn clone_state(other: &Self) -> Self {
        Self {
            scene: Default::default(),
            state: other.state.clone(),
            colors: other.colors.clone(),
        }
    }

    pub fn state_mut<F: FnOnce(&'_ mut BlockViewState)>(&mut self, writer: F) -> bool {
        let before_change = self.state.clone();
        writer(&mut self.state);
        before_change != self.state
    }

    pub fn color_mut<F: FnOnce(&'_ mut BlockViewAppearance)>(&mut self, writer: F) -> bool {
        let before_change = self.colors.clone();
        writer(&mut self.colors);
        before_change != self.colors
    }

    pub fn state_ui(
        &mut self,
        ui: &mut egui::Ui,
        mesh_options: &BlockViewStateMeshOptions,
    ) -> bool {
        self.state_mut(|state| {
            ui.checkbox(&mut state.show_xyz_axes, "XYZ axes");
            ui.checkbox(&mut state.show_surfaces, "Surfaces");
            ui.checkbox(&mut state.show_surface_edges, "Surface edge lines");
            ui.checkbox(&mut state.show_buoyancy_surfaces, "Buoyancy surfaces");
            ui.checkbox(&mut state.show_bounding_box_voxel, "Bounding box (voxel)");
            ui.checkbox(
                &mut state.show_bounding_box_voxel_physics,
                "Bounding box (voxel physics)",
            );
            ui.checkbox(
                &mut state.show_bounding_box_physics,
                "Bounding box (physics)",
            );

            for (key, show_option) in mesh_options.meshes {
                if show_option {
                    ui.checkbox(&mut state.show_mesh[key.clone()], key.ui_name());
                }
            }
        })
    }

    pub fn color_ui(
        &mut self,
        ui: &mut egui::Ui,
        id: egui::Id,
        _mesh_options: &BlockViewStateMeshOptions,
    ) -> bool {
        fn ui_color_picker_rgb(ui: &mut egui::Ui, color: &mut Color4) {
            let mut arr: [f32; 3] = color.as_array()[..3].try_into().unwrap();
            ui.color_edit_button_rgb(&mut arr);
            color.r = arr[0];
            color.g = arr[1];
            color.b = arr[2];
            color.a = 1.0;
        }

        fn ui_color_picker_rgba(ui: &mut egui::Ui, color: &mut Color4) {
            let mut arr = color.as_array();
            ui.color_edit_button_rgba_unmultiplied(&mut arr);
            color.r = arr[0];
            color.g = arr[1];
            color.b = arr[2];
            color.a = arr[3];
        }

        let state = self.state.clone();

        self.color_mut(|colors| {
            Grid::new(id).spacing([10.0, 8.0]).show(ui, |ui| {
                ui.label("Surface color");
                ui_color_picker_rgb(ui, &mut colors.surface);
                ui.end_row();

                ui.checkbox(&mut colors.override_color, "Override color");
                ui.end_row();

                if colors.override_color {
                    ui.label("Override color 1");
                    ui_color_picker_rgb(ui, &mut colors.override_1);
                    ui.end_row();

                    ui.label("Override color 2");
                    ui_color_picker_rgb(ui, &mut colors.override_2);
                    ui.end_row();

                    ui.label("Override color 3");
                    ui_color_picker_rgb(ui, &mut colors.override_3);
                    ui.end_row();
                }

                ui.label("Additive color");
                ui_color_picker_rgb(ui, &mut colors.additive);
                ui.end_row();

                for (show, text, colors) in [
                    (
                        state.show_buoyancy_surfaces,
                        "Buoyancy surfaces",
                        &mut colors.buoyancy_surface,
                    ),
                    (
                        state.show_bounding_box_voxel,
                        "Bounding box (voxel)",
                        &mut colors.bounding_box_voxel,
                    ),
                    (
                        state.show_bounding_box_voxel_physics,
                        "Bounding box (voxel physics)",
                        &mut colors.bounding_box_voxel_physics,
                    ),
                    (
                        state.show_bounding_box_physics,
                        "Bounding box (physics)",
                        &mut colors.bounding_box_physics,
                    ),
                ] {
                    if !show {
                        continue;
                    }
                    ui.label(text);
                    ui_color_picker_rgba(ui, &mut colors.0);
                    ui_color_picker_rgba(ui, &mut colors.1);
                    ui.add(DragValue::new(&mut colors.2).range(0.0..=10.0).speed(0.1));
                    ui.end_row();
                }
            });
        })
    }

    pub fn scene(&self) -> Arc<Mutex<Scene>> {
        self.scene.clone()
    }

    pub fn colors(&self) -> BlockViewAppearance {
        self.colors.clone()
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
        data: &Option<Arc<Definition>>,
        meshes: &Option<Arc<SwBlockDefinitionMeshes>>,
    ) {
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

        if let Some(data) = data {
            if let Some(surfaces) = data.surfaces.last() {
                for surface in &surfaces.surface {
                    let (mesh_obj, line_obj) = SurfaceObjectBuilder::new(
                        surface.shape,
                        surface.position.last(),
                        surface.orientation,
                        surface.rotation,
                    )
                    .basic_objects(
                        self.state.show_surfaces,
                        self.state.show_surface_edges,
                        self.colors.surface,
                    );
                    if let Some(obj) = mesh_obj {
                        self.add_object(obj);
                    }
                    if let Some(obj) = line_obj {
                        self.add_object(obj.set_z_offset(-1.0));
                    }
                }
            }

            if self.state.show_buoyancy_surfaces {
                if let Some(buoyancy_surfaces) = data.buoyancy_surfaces.last() {
                    let (mesh_color, line_color, line_width) = self.colors.buoyancy_surface;
                    for surface in &buoyancy_surfaces.surface {
                        let (mesh_obj, line_obj) = SurfaceObjectBuilder::new(
                            surface.shape,
                            surface.position.last(),
                            surface.orientation,
                            surface.rotation,
                        )
                        .translucent_objects(mesh_color, line_color, line_width);
                        if let Some(obj) = mesh_obj {
                            self.add_object(obj.set_z_offset(-1.0));
                        }
                        if let Some(obj) = line_obj {
                            self.add_object(obj.set_z_offset(-2.0));
                        }
                    }
                }
            }

            if self.state.show_bounding_box_voxel {
                if let Some((voxel_min, voxel_max)) =
                    data.voxel_min.last().zip(data.voxel_max.last())
                {
                    let (mesh_color, line_color, line_width) = self.colors.bounding_box_voxel;
                    let (mesh_obj, line_obj) =
                        BoundingBoxObjectBuilder::from_voxel(*voxel_min, *voxel_max)
                            .objects(mesh_color, line_color, line_width);
                    self.add_object(mesh_obj.set_z_offset(-4.0));
                    if let Some(line_obj) = line_obj {
                        self.add_object(line_obj.set_z_offset(-5.0));
                    }
                }
            }

            if self.state.show_bounding_box_voxel_physics {
                if let Some((voxel_physics_min, voxel_physics_max)) = data
                    .voxel_physics_min
                    .last()
                    .zip(data.voxel_physics_max.last())
                {
                    let (mesh_color, line_color, line_width) =
                        self.colors.bounding_box_voxel_physics;
                    let (mesh_obj, line_obj) = BoundingBoxObjectBuilder::from_voxel(
                        *voxel_physics_min,
                        *voxel_physics_max,
                    )
                    .objects(mesh_color, line_color, line_width);
                    self.add_object(mesh_obj.set_z_offset(-3.0));
                    if let Some(line_obj) = line_obj {
                        self.add_object(line_obj.set_z_offset(-4.0));
                    }
                }
            }

            if self.state.show_bounding_box_physics {
                if let Some((bb_physics_min, bb_physics_max)) =
                    data.bb_physics_min.last().zip(data.bb_physics_max.last())
                {
                    let (mesh_color, line_color, line_width) = self.colors.bounding_box_physics;
                    let (mesh_obj, line_obj) =
                        BoundingBoxObjectBuilder::new(*bb_physics_min, *bb_physics_max)
                            .objects(mesh_color, line_color, line_width);
                    self.add_object(mesh_obj.set_z_offset(-2.0));
                    if let Some(line_obj) = line_obj {
                        self.add_object(line_obj.set_z_offset(-3.0));
                    }
                }
            }
        }

        if let Some(meshes) = meshes {
            for (key, show) in self.state.show_mesh {
                if !show {
                    continue;
                }
                if let Some(Ok(mesh)) = meshes.get_mesh(&key) {
                    for m in mesh.as_meshes() {
                        self.add_object(SceneObject::from_mesh(m, None));
                    }
                }
            }
        }
    }

    pub fn clear(&mut self) {
        self.scene.lock().unwrap().clear();
    }
}
