use super::{utils::replace_extension, BlockViewScene, DefinitionPointer, DefinitionsStore};
use crate::{
    gl_renderer::{Camera, OrbitCamera, RenderFramebuffer, SceneRenderer},
    sw_block_definition::{Definition, SwBlockDefinitionMeshes, Voxel},
};
use glam::{Vec3, Vec4};
use std::{path::PathBuf, sync::Arc, time};

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct AutoCamera {
    pub camera: OrbitCamera,
    pub width: i32,
    pub height: i32,
    pub margin: f32,
    pub is_orthographic: bool,
    pub fov_y: f32,
    pub is_auto: bool,
}

impl Default for AutoCamera {
    fn default() -> Self {
        let width = 512;
        let height = 512;
        let fov_y = 45f32.to_radians();

        Self {
            camera: OrbitCamera::new(
                Vec3::ZERO,
                Vec3::new(1.0, -0.5, 1.0),
                fov_y,
                width as f32 / height as f32,
            ),
            width,
            height,
            margin: 0.0,
            is_orthographic: false,
            fov_y,
            is_auto: false,
        }
    }
}

impl AutoCamera {
    pub fn control(&mut self, ui: &mut egui::Ui, response: egui::Response) {
        self.camera.control(ui, response, true, true, !self.is_auto);
    }

    pub fn aspect_ratio(&self) -> f32 {
        self.width as f32 / self.height as f32
    }

    pub fn update(&mut self, data: &Definition) {
        let width = self.width as f32;
        let height = self.height as f32;
        let aspect_ratio = self.aspect_ratio();

        let camera = &mut self.camera;

        camera.set_aspect_ratio(aspect_ratio);
        if self.is_orthographic {
            camera.set_orthographic();
        } else {
            camera.set_perspective();
            camera.set_fov_y(self.fov_y);
        }

        if self.is_auto {
            let voxels: Vec<&Voxel> = data
                .voxels
                .last()
                .map(|v| v.voxel.iter().collect())
                .unwrap_or_default();
            let (voxel_min, voxel_max) =
                voxels
                    .iter()
                    .fold(((0, 0, 0), (0, 0, 0)), |(min, max), voxel| {
                        let pos = voxel
                            .position
                            .last()
                            .map(|p| p.as_tuple(0))
                            .unwrap_or((0, 0, 0));
                        (
                            (min.0.min(pos.0), min.1.min(pos.1), min.2.min(pos.2)),
                            (max.0.max(pos.0), max.1.max(pos.1), max.2.max(pos.2)),
                        )
                    });
            let corner_min: Vec3 =
                (Vec3::new(voxel_min.0 as f32, voxel_min.1 as f32, voxel_min.2 as f32)
                    - 0.5 * Vec3::ONE)
                    * 0.25;
            let corner_max: Vec3 =
                (Vec3::new(voxel_max.0 as f32, voxel_max.1 as f32, voxel_max.2 as f32)
                    + 0.5 * Vec3::ONE)
                    * 0.25;
            let center = (corner_min + corner_max) * 0.5;

            let min_x = corner_min.x;
            let min_y = corner_min.y;
            let min_z = corner_min.z;
            let max_x = corner_max.x;
            let max_y = corner_max.y;
            let max_z = corner_max.z;
            let corners = [
                Vec3::new(min_x, min_y, min_z),
                Vec3::new(min_x, max_y, min_z),
                Vec3::new(min_x, max_y, max_z),
                Vec3::new(min_x, min_y, max_z),
                Vec3::new(max_x, min_y, min_z),
                Vec3::new(max_x, max_y, min_z),
                Vec3::new(max_x, max_y, max_z),
                Vec3::new(max_x, min_y, max_z),
            ];

            camera.center = Vec3::new(center.x, center.y, -center.z);

            let s = if self.is_orthographic {
                // 平行投影
                let mat_vp = camera.mat_view_proj();
                let (screen_min_x, screen_min_y, screen_max_x, screen_max_y) = corners.iter().fold(
                    (
                        f32::INFINITY,
                        f32::INFINITY,
                        f32::NEG_INFINITY,
                        f32::NEG_INFINITY,
                    ),
                    |(min_x, min_y, max_x, max_y), c| {
                        let s = mat_vp.mul_vec4(Vec4::new(c.x, c.y, -c.z, 1.0));
                        (
                            min_x.min(s.x),
                            min_y.min(s.y),
                            max_x.max(s.x),
                            max_y.max(s.y),
                        )
                    },
                );

                let sx =
                    (-screen_min_x).max(screen_max_x) / (((width) - 2.0 * self.margin) / width);
                let sy =
                    (-screen_min_y).max(screen_max_y) / (((height) - 2.0 * self.margin) / height);
                sx.max(sy)
            } else {
                // 透視投影
                // 中心をバウンディングボックスの中心にしているが、角度により片側に偏って見えてしまうので、できれば直す
                let view = camera.mat_view();
                let tan = (self.fov_y / 2.0).tan();
                let tan_x = (width - 2.0 * self.margin) / width * tan * aspect_ratio;
                let tan_y = (height - 2.0 * self.margin) / height * tan;
                let len = camera.direction.length();
                corners.iter().fold(0.0, |s: f32, corner| {
                    let view_point =
                        view.transform_point3(Vec3::new(corner.x, corner.y, -corner.z));
                    let dx = (view_point.x.abs() / tan_x) - (-view_point.z);
                    let dy = (view_point.y.abs() / tan_y) - (-view_point.z);
                    let sx = (len + dx) / len;
                    let sy = (len + dy) / len;
                    s.max(sx.max(sy))
                })
            };
            camera.direction *= s;
        }
    }
}

pub type RenderMessageTuple = (Arc<Definition>, Arc<SwBlockDefinitionMeshes>, String);

pub struct ImageRenderer {
    definitions: Vec<DefinitionPointer>,
    i: usize,
    auto_camera: AutoCamera,
    scene: BlockViewScene,
    renderer: SceneRenderer,
    framebuffer: Box<dyn RenderFramebuffer>,
    save_path: PathBuf,
    append_filename: bool,
    start_time: time::Instant,
    finish_time: Option<time::Instant>,
    logs: Vec<String>,
    progress: SaveImageProgress,
}

impl ImageRenderer {
    pub fn new(
        definitions: Vec<DefinitionPointer>,
        scene: BlockViewScene,
        renderer: SceneRenderer,
        auto_camera: &AutoCamera,
        framebuffer: Box<dyn RenderFramebuffer>,
        save_path: PathBuf,
        append_filename: bool,
    ) -> Self {
        let len = definitions.len();
        Self {
            definitions,
            i: 0,
            auto_camera: auto_camera.clone(),
            scene,
            renderer,
            framebuffer,
            save_path,
            append_filename,
            start_time: time::Instant::now(),
            finish_time: None,
            logs: Vec::new(),
            progress: SaveImageProgress::new(len),
        }
    }

    pub fn update(&mut self, definitions_store: &mut DefinitionsStore) {
        let frame_start_time = std::time::Instant::now();

        loop {
            if self.i >= self.definitions.len() {
                self.progress.current = self.i;

                // 1秒以上かかっていたら、1秒間100%と表示
                let mut finish_immediate = self.start_time.elapsed().as_secs() < 1;
                if let Some(finish_time) = self.finish_time {
                    finish_immediate = finish_immediate || finish_time.elapsed().as_secs() >= 1;
                } else {
                    self.finish_time = Some(time::Instant::now());
                }

                if finish_immediate {
                    self.progress.done = true;
                }
                return;
            }

            let definition = &self.definitions[self.i];
            let data = if let Ok(mut definition) = definition.lock() {
                let filename = definition.filename();
                match definition.load_data() {
                    Some(Ok(data)) => Some((data, filename)),
                    Some(Err(err)) => {
                        self.logs.push(format!(
                            "Failed to save image of {} due to {}",
                            filename, err
                        ));
                        self.i += 1;
                        self.progress.current = self.i;
                        continue;
                    }
                    None => None,
                }
            } else {
                None
            };

            if let Some((data, filename)) = data {
                if self.scene.update(definition, definitions_store) {
                    self.auto_camera.update(&data);

                    self.framebuffer.before_paint();
                    self.renderer.paint(
                        self.framebuffer.gl(),
                        &self.auto_camera.camera,
                        &self.scene.appearance(),
                    );
                    self.framebuffer.after_paint();

                    let image = self.framebuffer.get_image();
                    let _result = if self.append_filename {
                        image.save(self.save_path.join(replace_extension(&filename, "png")))
                    } else {
                        image.save(&self.save_path)
                    };

                    self.i += 1;
                    self.progress.current = self.i;
                }
            }

            /*if let Ok(mut definition) = self.definitions[self.i].lock() {
                let filename = definition.filename();
                if let Some(data_r) = definition.load_data() {
                    match data_r {
                        Ok(data) => {
                            if definition.meshes_loaded() {
                                self.auto_camera.update(&data);
                                self.scene.update(definition, definitions_store);

                                self.framebuffer.before_paint();
                                self.renderer.paint(
                                    self.framebuffer.gl(),
                                    &self.auto_camera.camera,
                                    &self.scene.appearance(),
                                );
                                self.framebuffer.after_paint();

                                let image = self.framebuffer.get_image();
                                let _result = if self.append_filename {
                                    image.save(
                                        self.save_path.join(replace_extension(&filename, "png")),
                                    )
                                } else {
                                    image.save(&self.save_path)
                                };

                                self.i += 1;
                                self.progress.current = self.i;
                            }
                        }
                        Err(err) => {
                            self.logs.push(format!(
                                "Failed to save image of {} due to {}",
                                filename, err
                            ));
                            self.i += 1;
                            self.progress.current = self.i;
                        }
                    }
                }
            }*/

            // 1フレームに200ミリ秒以上かけない、でも1フレームに最低1枚は処理する
            if frame_start_time.elapsed().as_millis() >= 200 {
                break;
            }
        }
    }

    pub fn logs(&self) -> &Vec<String> {
        &self.logs
    }

    pub fn progress(&self) -> &SaveImageProgress {
        &self.progress
    }
}

#[derive(Debug)]
pub enum ProgressMessage {
    Progress(usize),
    Done,
}

#[derive(Debug)]
pub struct SaveImageProgress {
    current: usize,
    total: usize,
    done: bool,
    message: Option<String>,
}

impl SaveImageProgress {
    pub fn new(total: usize) -> Self {
        Self {
            current: 0,
            total,
            done: false,
            message: None,
        }
    }

    pub fn current(&self) -> usize {
        self.current
    }

    pub fn total(&self) -> usize {
        self.total
    }

    pub fn progress(&self) -> f32 {
        self.current as f32 / self.total as f32
    }

    pub fn done(&self) -> bool {
        self.done
    }

    pub fn message(&self) -> &Option<String> {
        &self.message
    }
}
