use crate::{
    gl_renderer::{Color4, Line, Scene, SceneObject},
    sw_block_definition::{
        Definition, SurfaceObjectBuilder, SwBlockDefinitionMeshKey, SwBlockDefinitionMeshes,
    },
};
use enum_map::EnumMap;
use glam::Vec3;
use std::{
    rc::Rc,
    sync::{Arc, Mutex},
};

#[derive(Clone, PartialEq, Eq)]
pub struct BlockViewState {
    pub show_xyz_axes: bool,
    pub show_surfaces: bool,
    pub show_surface_edges: bool,
    pub show_buoyancy_surfaces: bool,
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
            show_mesh,
        }
    }
}

#[derive(Default)]
pub struct BlockViewScene {
    scene: Arc<Mutex<Scene>>,
    state: BlockViewState,
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

    pub fn update(
        &mut self,
        data: Option<Arc<Definition>>,
        meshes: Option<Rc<SwBlockDefinitionMeshes>>,
    ) {
        self.scene.lock().unwrap().clear();

        if self.state.show_xyz_axes {
            for (direction, color) in [
                (Vec3::X, Color4::RED),
                (Vec3::Y, Color4::GREEN),
                (Vec3::Z, Color4::BLUE),
            ] {
                self.scene
                    .lock()
                    .unwrap()
                    .add_object(SceneObject::from_line(
                        Line::single_color_lh(
                            vec![Vec3::ZERO, 100.0 * direction],
                            color,
                            2.0,
                            false,
                        ),
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
                        self.scene.lock().unwrap().add_object(obj);
                    }
                    if let Some(obj) = line_obj {
                        self.scene
                            .lock()
                            .unwrap()
                            .add_object(obj.set_z_offset(-0.00001));
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
                        .translucent_objects();
                        if let Some(obj) = mesh_obj {
                            self.scene
                                .lock()
                                .unwrap()
                                .add_object(obj.set_z_offset(-0.00001));
                        }
                        if let Some(obj) = line_obj {
                            self.scene
                                .lock()
                                .unwrap()
                                .add_object(obj.set_z_offset(-0.00002));
                        }
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
                        self.scene
                            .lock()
                            .unwrap()
                            .add_object(SceneObject::from_mesh(m, None));
                    }
                }
            }
        }
    }
}
