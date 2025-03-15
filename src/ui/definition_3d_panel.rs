use super::State;
use crate::gl_renderer::{Color4, Line, OrbitCamera, Scene, SceneObject, SceneRenderer};
use crate::sw_block_definition::SurfaceObjectBuilder;
use eframe::egui_glow;
use egui::{vec2, Id, Image, Modal, Rect, TextureOptions};
use glam::Vec3;
use std::sync::{Arc, Mutex};

#[derive(serde::Serialize, serde::Deserialize)]
pub struct Definition3dPanel {
    open_save_image_modal: bool,
    #[serde(skip)]
    scene: Arc<Mutex<Scene>>,
    camera: Arc<Mutex<OrbitCamera>>,
    #[serde(skip)]
    renderer: Option<Arc<egui::mutex::Mutex<SceneRenderer>>>,
    #[serde(skip)]
    mesh_loaded: bool,
    #[cfg(not(target_arch = "wasm32"))]
    #[serde(skip)]
    framebuffer: Option<crate::gl_renderer::MultisampleFramebuffer>,
}

impl Definition3dPanel {
    pub fn new<'a>(
        cc: &'a eframe::CreationContext<'a>,
        camera: Option<OrbitCamera>,
    ) -> Option<Self> {
        let scene = Arc::new(Mutex::new(Scene::default()));
        let mut camera = camera.unwrap_or_else(|| OrbitCamera {
            direction: Vec3::new(1.0, -0.5, -1.0),
            ..Default::default()
        });
        camera.orthogonalize_up();
        let camera = Arc::new(Mutex::new(camera));

        let mut instance = Self {
            open_save_image_modal: false,
            scene: scene.clone(),
            camera,
            renderer: None,
            mesh_loaded: false,
            #[cfg(not(target_arch = "wasm32"))]
            framebuffer: None,
        };
        Self::creation_context(&mut instance, cc);
        Some(instance)
    }

    pub fn creation_context<'a>(&mut self, cc: &'a eframe::CreationContext<'a>) {
        if let Some(gl) = &cc.gl {
            let renderer = SceneRenderer::new(gl, self.scene.clone());
            self.renderer = Some(Arc::new(egui::mutex::Mutex::new(renderer)));
            #[cfg(not(target_arch = "wasm32"))]
            {
                self.framebuffer = Some(crate::gl_renderer::MultisampleFramebuffer::new(
                    gl.clone(),
                    512,
                    512,
                    16,
                ));
            }
        }
    }

    pub fn destroy(&self, gl: Option<&eframe::glow::Context>) {
        if let Some(renderer) = &self.renderer {
            renderer.lock().destroy(gl);
        }
    }

    #[allow(unused_variables)]
    pub fn ui(&mut self, ui: &mut egui::Ui, state: &mut State, frame: &eframe::Frame) {
        egui::Frame::canvas(ui.style())
            .fill(egui::Color32::TRANSPARENT)
            .inner_margin(0.0)
            .show(ui, |ui| {
                let s = ui.available_width();
                let (rect, response) = ui.allocate_exact_size(vec2(s, s), egui::Sense::drag());
                self.paint_canvas(ui, rect, response);
            });

        let mut c = state.show_xyz_axis();
        ui.checkbox(&mut c, "XYZ axes");
        state.set_show_xyz_axis(c);

        let mut c = state.show_surfaces();
        ui.checkbox(&mut c, "Surfaces");
        state.set_show_surfaces(c);

        let mut c = state.show_surface_edge();
        ui.checkbox(&mut c, "Surface edge lines");
        state.set_show_surface_edge(c);

        let mut c = state.show_buoyancy_surfaces();
        ui.checkbox(&mut c, "Buoyancy surfaces");
        state.set_show_buoyancy_surfaces(c);

        let mesh_loaded_now;

        if let Some(meshes) = state.selected_meshes() {
            mesh_loaded_now = true;

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
        } else {
            mesh_loaded_now = false;
        }

        #[cfg(not(target_arch = "wasm32"))]
        {
            ui.separator();

            if ui.button("Save image").clicked() {
                self.open_save_image_modal = true;
            }
        }

        if state.is_changed_3d() || (mesh_loaded_now != self.mesh_loaded) {
            self.update_scene(state);
        }
        self.mesh_loaded = mesh_loaded_now;

        if self.open_save_image_modal {
            let modal = Modal::new(Id::new("save_image_modal")).show(ui.ctx(), |ui| {
                egui::Frame::canvas(ui.style())
                    .fill(egui::Color32::TRANSPARENT)
                    .inner_margin(0.0)
                    .show(ui, |ui| {
                        let s = ui.available_width();
                        let (rect, response) =
                            ui.allocate_exact_size(vec2(s, s), egui::Sense::drag());
                        paint_checker_pattern(ui, rect);
                        self.paint_canvas(ui, rect, response);
                    });

                ui.separator();

                if ui.button("Save image").clicked() {
                    self.save_image(Some(frame));
                }
            });

            if modal.should_close() {
                self.open_save_image_modal = false;
            }
        }
    }

    fn paint_canvas(&mut self, ui: &mut egui::Ui, rect: Rect, response: egui::Response) {
        self.camera.lock().unwrap().control(ui, response);
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
        self.scene.lock().unwrap().clear();

        if state.show_xyz_axis() {
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

        if let Some(data) = state
            .selected_definition()
            .and_then(|def| def.load_data().and_then(|d| d.ok()))
        {
            if let Some(surfaces) = data.surfaces.last() {
                for surface in &surfaces.surface {
                    let (mesh_obj, line_obj) = SurfaceObjectBuilder::new(
                        surface.shape,
                        surface.position.last(),
                        surface.orientation,
                        surface.rotation,
                    )
                    .basic_objects(state.show_surfaces(), state.show_surface_edge());
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

            if state.show_buoyancy_surfaces() {
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

        if let Some(meshes) = state.selected_meshes() {
            for (key, show) in state.show_mesh() {
                if !*show {
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

    fn save_image<W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle>(
        &self,
        parent: Option<&W>,
    ) {
        if let Some((renderer, framebuffer)) = self.renderer.as_ref().zip(self.framebuffer.as_ref())
        {
            framebuffer.bind();
            renderer
                .lock()
                .paint(&framebuffer.gl(), self.camera.clone());
            framebuffer.resolve();
            let image = framebuffer.get_image();

            if let Some(path) = save_image_dialog(parent) {
                image.save(path).expect("Failed to save image");
            }
        }
    }
}

fn paint_checker_pattern(ui: &mut egui::Ui, rect: Rect) {
    egui_extras::install_image_loaders(ui.ctx());
    let background_image_source = match ui.ctx().theme() {
        egui::Theme::Light => egui::include_image!("../../images/checker_light.png"),
        egui::Theme::Dark => egui::include_image!("../../images/checker_dark.png"),
    };
    let background_image = Image::new(background_image_source)
        .texture_options(TextureOptions {
            magnification: egui::TextureFilter::Nearest,
            minification: egui::TextureFilter::Nearest,
            wrap_mode: egui::TextureWrapMode::Repeat,
            ..Default::default()
        })
        .uv(Rect::from_x_y_ranges(
            0.0..=(rect.width() / 16.0),
            0.0..=(rect.height() / 16.0),
        ));
    background_image.paint_at(ui, rect);
}

#[cfg(not(target_arch = "wasm32"))]
fn save_image_dialog<
    W: raw_window_handle::HasWindowHandle + raw_window_handle::HasDisplayHandle,
>(
    parent: Option<&W>,
) -> Option<std::path::PathBuf> {
    use rfd::FileDialog;

    let mut dialog = FileDialog::new().add_filter("PNG image", &["png"]);
    if let Some(p) = parent {
        dialog = dialog.set_parent(p)
    }
    dialog.save_file()
}
