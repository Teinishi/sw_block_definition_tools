use super::State;
use crate::gl_renderer::{Color4, Line, OrbitCamera, Scene, SceneObject, SceneRenderer};
use crate::sw_block_definition::create_surface_object;
use eframe::egui_glow;
use egui::{mutex::Mutex, vec2};
use glam::Vec3;
use std::sync::Arc;

pub struct Definition3dPanel {
    scene: Arc<Mutex<Scene>>,
    camera: Arc<Mutex<OrbitCamera>>,
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    //framebuffer: Option<MultisampleFramebuffer>,
}

impl Definition3dPanel {
    pub fn new<'a>(cc: &'a eframe::CreationContext<'a>) -> Option<Self> {
        let gl = cc.gl.as_ref()?;
        let scene = Arc::new(Mutex::new(Scene::default()));
        let mut camera = OrbitCamera {
            direction: Vec3::new(1.0, -0.5, -1.0),
            ..Default::default()
        };
        camera.orthogonalize_up();
        let camera = Arc::new(Mutex::new(camera));
        let renderer = SceneRenderer::new(gl, scene.clone());

        Some(Self {
            scene: scene.clone(),
            camera,
            renderer: Some(Arc::new(egui::mutex::Mutex::new(renderer))),
            //framebuffer: MultisampleFramebuffer::new(gl.clone(), 512, 512, 16),
        })
    }

    pub fn destroy(&self, gl: Option<&eframe::glow::Context>) {
        if let Some(renderer) = &self.renderer {
            renderer.lock().destroy(gl);
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State) {
        egui::Frame::canvas(ui.style())
            .fill(egui::Color32::TRANSPARENT)
            .show(ui, |ui| {
                self.custom_painting(ui);
            });

        let mut c = state.show_xyz_axis();
        ui.checkbox(&mut c, "XYZ Axis");
        state.set_show_xyz_axis(c);

        let mut c = state.show_surfaces();
        ui.checkbox(&mut c, "Surfaces");
        state.set_show_surfaces(c);

        let mut c = state.show_surface_edge();
        ui.checkbox(&mut c, "Surface Edge Lines");
        state.set_show_surface_edge(c);

        if let Some(definition) = state.selected_definition() {
            let meshes = definition.meshes();

            let mut change = None;
            for (key, show) in state.show_mesh() {
                if let Some(mesh) = meshes.get_mesh(&key) {
                    let name = key.xml_name();
                    if let Err(err) = mesh {
                        ui.collapsing(format!("{}: Error", name), |ui| {
                            ui.label(format!("{}", err));
                        });
                    } else {
                        let mut c = *show;
                        ui.checkbox(&mut c, name);
                        if c != *show {
                            change = Some((key, c));
                        }
                    }
                }
            }
            if let Some((key, value)) = change {
                state.set_show_mesh(key, value);
            }
        }

        if state.is_changed() {
            self.update_scene(state);
        }
    }

    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let size = ui.available_width();
        let (rect, response) = ui.allocate_exact_size(vec2(size, size), egui::Sense::drag());

        self.camera.lock().control(ui, response);
        let camera = self.camera.clone();

        if let Some(renderer) = self.renderer.clone() {
            let cb = egui_glow::CallbackFn::new(move |_info, painter| {
                renderer.lock().paint(painter.gl(), camera.clone());
            });

            let callback = egui::PaintCallback {
                rect,
                callback: Arc::new(cb),
            };
            ui.painter().add(callback);
        }
    }

    fn update_scene(&mut self, state: &mut State) {
        self.scene.lock().clear();

        if state.show_xyz_axis() {
            for (direction, color) in [
                (Vec3::X, Color4::RED),
                (Vec3::Y, Color4::GREEN),
                (Vec3::Z, Color4::BLUE),
            ] {
                self.scene.lock().add_object(SceneObject::from_line(
                    Line::single_color_lh(vec![Vec3::ZERO, 100.0 * direction], color, 2.0, false),
                    None,
                ));
            }
        }

        if let Some(data) = state.selected_definition().and_then(|def| def.data().ok()) {
            if let Some(surfaces) = data.surfaces.last() {
                for surface in &surfaces.surface {
                    let (mesh_obj, line_obj) = create_surface_object(
                        surface,
                        state.show_surfaces(),
                        state.show_surface_edge(),
                    );
                    if let Some(obj) = mesh_obj {
                        self.scene.lock().add_object(obj);
                    }
                    if let Some(obj) = line_obj {
                        self.scene.lock().add_object(obj);
                    }
                }
            }
        }

        if let Some(definition) = state.selected_definition() {
            let meshes = definition.meshes();

            for (key, show) in state.show_mesh() {
                if !*show {
                    continue;
                }
                if let Some(Ok(mesh)) = meshes.get_mesh(&key) {
                    for m in mesh.as_meshes() {
                        self.scene
                            .lock()
                            .add_object(SceneObject::from_mesh(m, None));
                    }
                }
            }
        }
    }
}
