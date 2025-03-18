use crate::{
    gl_renderer::{Color4, Line, Scene, SceneObject},
    sw_block_definition::{
        BoundingBoxObjectBuilder, Definition, SurfaceObjectBuilder, SwBlockDefinitionMeshKey,
        SwBlockDefinitionMeshes,
    },
};
use enum_map::EnumMap;
use glam::Vec3;
use std::{
    rc::Rc,
    sync::{Arc, Mutex, MutexGuard},
};

const BUOYANCY_SURFACE_MESH_COLOR: Color4 = Color4 {
    r: 0.1,
    g: 0.5,
    b: 0.8,
    a: 0.3,
};
const BUOYANCY_SURFACE_LINE_COLOR: Color4 = Color4 {
    r: 0.05,
    g: 0.25,
    b: 0.4,
    a: 0.2,
};
const BOUNDING_BOX_VOXEL_MESH_COLOR: Color4 = Color4 {
    r: 0.8,
    g: 0.1,
    b: 0.2,
    a: 0.3,
};
const BOUNDING_BOX_VOXEL_LINE_COLOR: Color4 = Color4 {
    r: 0.4,
    g: 0.05,
    b: 0.1,
    a: 0.2,
};
const BOUNDING_BOX_VOXEL_PHYSICS_MESH_COLOR: Color4 = Color4 {
    r: 0.8,
    g: 0.8,
    b: 0.2,
    a: 0.3,
};
const BOUNDING_BOX_VOXEL_PHYSICS_LINE_COLOR: Color4 = Color4 {
    r: 0.4,
    g: 0.4,
    b: 0.1,
    a: 0.2,
};
const BOUNDING_BOX_PHYSICS_MESH_COLOR: Color4 = Color4 {
    r: 0.1,
    g: 0.8,
    b: 0.2,
    a: 0.3,
};
const BOUNDING_BOX_PHYSICS_LINE_COLOR: Color4 = Color4 {
    r: 0.05,
    g: 0.4,
    b: 0.1,
    a: 0.2,
};

#[derive(Clone, PartialEq, Eq)]
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

#[derive(Default)]
pub struct BlockViewScene {
    scene: Arc<Mutex<Scene>>,
    state: BlockViewState,
    is_orthographic: bool,
}

impl BlockViewScene {
    pub fn state_mut<F: FnOnce(&'_ mut BlockViewState)>(&mut self, writer: F) -> bool {
        let mut changed = self.state.clone();
        writer(&mut changed);

        let is_changed = changed != self.state;
        let _ = std::mem::replace(&mut self.state, changed);
        is_changed
    }

    pub fn state_ui(
        &mut self,
        ui: &mut egui::Ui,
        data: Option<Arc<Definition>>,
        meshes: Option<Rc<SwBlockDefinitionMeshes>>,
        force_update: bool,
    ) {
        let meshes_c = meshes.clone();
        let is_changed = self.state_mut(|state| {
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

            if let Some(meshes) = meshes_c {
                for (key, show) in state.show_mesh.iter_mut() {
                    if let Some(mesh) = meshes.get_mesh(&key) {
                        let name = key.ui_name();
                        if let Err(err) = mesh {
                            ui.collapsing(format!("{}: Error", name), |ui| {
                                ui.label(format!("{}", err));
                            });
                        } else {
                            ui.checkbox(show, name);
                        }
                    }
                }
            }
        });
        if force_update || is_changed {
            self.update(data, meshes);
        }
    }

    pub fn scene(&self) -> Arc<Mutex<Scene>> {
        self.scene.clone()
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
        data: Option<Arc<Definition>>,
        meshes: Option<Rc<SwBlockDefinitionMeshes>>,
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
                    .basic_objects(self.state.show_surfaces, self.state.show_surface_edges);
                    if let Some(obj) = mesh_obj {
                        self.add_object(obj);
                    }
                    if let Some(obj) = line_obj {
                        self.add_object(obj.set_z_offset(self.z_offset(-1)));
                    }
                }
            }

            if self.state.show_buoyancy_surfaces {
                if let Some(buoyancy_surfaces) = data.buoyancy_surfaces.last() {
                    for surface in &buoyancy_surfaces.surface {
                        let (mesh_obj, line_obj) = SurfaceObjectBuilder::new(
                            surface.shape,
                            surface.position.last(),
                            surface.orientation,
                            surface.rotation,
                        )
                        .translucent_objects(
                            BUOYANCY_SURFACE_MESH_COLOR,
                            BUOYANCY_SURFACE_LINE_COLOR,
                        );
                        if let Some(obj) = mesh_obj {
                            self.add_object(obj.set_z_offset(self.z_offset(-1)));
                        }
                        if let Some(obj) = line_obj {
                            self.add_object(obj.set_z_offset(self.z_offset(-2)));
                        }
                    }
                }
            }

            if self.state.show_bounding_box_voxel {
                if let Some((voxel_min, voxel_max)) =
                    data.voxel_min.last().zip(data.voxel_max.last())
                {
                    let (mesh_obj, line_obj) =
                        BoundingBoxObjectBuilder::from_voxel(*voxel_min, *voxel_max)
                            .objects(BOUNDING_BOX_VOXEL_MESH_COLOR, BOUNDING_BOX_VOXEL_LINE_COLOR);
                    self.add_object(mesh_obj.set_z_offset(self.z_offset(-4)));
                    self.add_object(line_obj.set_z_offset(self.z_offset(-5)));
                }
            }

            if self.state.show_bounding_box_voxel_physics {
                if let Some((voxel_physics_min, voxel_physics_max)) = data
                    .voxel_physics_min
                    .last()
                    .zip(data.voxel_physics_max.last())
                {
                    let (mesh_obj, line_obj) = BoundingBoxObjectBuilder::from_voxel(
                        *voxel_physics_min,
                        *voxel_physics_max,
                    )
                    .objects(
                        BOUNDING_BOX_VOXEL_PHYSICS_MESH_COLOR,
                        BOUNDING_BOX_VOXEL_PHYSICS_LINE_COLOR,
                    );
                    self.add_object(mesh_obj.set_z_offset(self.z_offset(-3)));
                    self.add_object(line_obj.set_z_offset(self.z_offset(-4)));
                }
            }

            if self.state.show_bounding_box_physics {
                if let Some((bb_physics_min, bb_physics_max)) =
                    data.bb_physics_min.last().zip(data.bb_physics_max.last())
                {
                    let (mesh_obj, line_obj) =
                        BoundingBoxObjectBuilder::new(*bb_physics_min, *bb_physics_max).objects(
                            BOUNDING_BOX_PHYSICS_MESH_COLOR,
                            BOUNDING_BOX_PHYSICS_LINE_COLOR,
                        );
                    self.add_object(mesh_obj.set_z_offset(self.z_offset(-2)));
                    self.add_object(line_obj.set_z_offset(self.z_offset(-3)));
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

    pub fn set_orthographic(
        &mut self,
        is_orthographic: bool,
        data: Option<Arc<Definition>>,
        meshes: Option<Rc<SwBlockDefinitionMeshes>>,
    ) {
        if self.is_orthographic != is_orthographic {
            self.is_orthographic = is_orthographic;
            self.update(data, meshes);
        }
    }

    fn z_offset(&self, count: i32) -> f32 {
        let unit: f32 = if self.is_orthographic {
            0.0000005
        } else {
            0.00001
        };
        unit * count as f32
    }
}
