use super::{BlockViewAppearance, BlockViewScene, BlockViewState};
use crate::{
    definition_hub::{BlockDefinition, DefinitionRegistory},
    sw_block_definition::{Definition, Voxel, VoxelLocationChild},
    sw_gl_3d::{Camera, OrbitCamera, RenderFramebuffer, SceneRenderer, SwBlockMeshes},
    utils::replace_extension,
};
use glam::Vec3;
use std::{collections::HashSet, ffi::OsString, path::PathBuf, sync::Arc, time};

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct VoxelPosition {
    x: i32,
    y: i32,
    z: i32,
}

impl From<&Voxel> for VoxelPosition {
    fn from(value: &Voxel) -> Self {
        let (x, y, z) = value
            .position
            .last()
            .map(|p| p.as_tuple(0))
            .unwrap_or((0, 0, 0));
        Self { x, y, z }
    }
}

impl From<&VoxelLocationChild> for VoxelPosition {
    fn from(value: &VoxelLocationChild) -> Self {
        Self {
            x: value.x.unwrap_or(0),
            y: value.y.unwrap_or(0),
            z: value.z.unwrap_or(0),
        }
    }
}

impl VoxelPosition {
    fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
            z: self.z.min(other.z),
        }
    }

    fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
            z: self.z.max(other.z),
        }
    }

    fn add(&self, other: &Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }

    fn corner_min(&self) -> VoxelCorner {
        VoxelCorner::new(self.x, self.y, self.z)
    }

    fn corner_max(&self) -> VoxelCorner {
        VoxelCorner::new(self.x + 1, self.y + 1, self.z + 1)
    }

    fn corners(&self) -> [VoxelCorner; 8] {
        let x = self.x;
        let y = self.y;
        let z = self.z;
        [
            VoxelCorner::new(x, y, z),
            VoxelCorner::new(x, y, z + 1),
            VoxelCorner::new(x, y + 1, z),
            VoxelCorner::new(x, y + 1, z + 1),
            VoxelCorner::new(x + 1, y, z),
            VoxelCorner::new(x + 1, y, z + 1),
            VoxelCorner::new(x + 1, y + 1, z),
            VoxelCorner::new(x + 1, y + 1, z + 1),
        ]
    }

    fn get_voxels(
        definition: &BlockDefinition,
        registory: &mut DefinitionRegistory,
        include_child: bool,
    ) -> Vec<Self> {
        if let Some(Ok(data)) = definition.load_data() {
            let mut voxels = if let Some(voxels) = data.voxels.last() {
                voxels.voxel.iter().map(Self::from).collect()
            } else {
                Vec::new()
            };

            if !include_child {
                return voxels;
            }
            if let Some(child) = data
                .child_name
                .as_ref()
                .and_then(|name| registory.resolve(definition.mod_key(), name))
            {
                let child_position = data
                    .voxel_location_child
                    .last()
                    .map(Self::from)
                    .unwrap_or_default();
                voxels.extend(
                    Self::get_voxels(&child.clone(), registory, false)
                        .iter()
                        .map(|v| v.add(&child_position)),
                );
            }

            voxels
        } else {
            vec![]
        }
    }

    fn get_bounds(voxels: &[Self]) -> (Self, Self) {
        voxels
            .iter()
            .fold((Self::default(), Self::default()), |(min, max), voxel| {
                (min.min(voxel), max.max(voxel))
            })
    }
}

#[derive(Debug, Default, Clone, Hash, PartialEq, Eq)]
struct VoxelCorner {
    x: i32,
    y: i32,
    z: i32,
}

impl VoxelCorner {
    fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn to_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    fn world_pos_lh(&self) -> Vec3 {
        let vec = 0.25 * (self.to_vec3() - 0.5 * Vec3::ONE);
        Vec3::new(vec.x, vec.y, -vec.z)
    }

    fn get_outer_corners(voxels: HashSet<VoxelPosition>) -> HashSet<Self> {
        // ボクセルの頂点のうち外側のものを抽出
        // 実際は余計なものも含んでいそうだが、問題にならないのでスルーしている
        let mut corners = HashSet::new();
        let mut removed = HashSet::new();

        for voxel in &voxels {
            for corner in voxel.corners() {
                if !removed.contains(&corner) {
                    if corners.remove(&corner) {
                        removed.insert(corner);
                    } else {
                        corners.insert(corner);
                    }
                }
            }
        }

        corners
    }
}

#[derive(serde::Serialize, serde::Deserialize, Debug, Clone)]
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

    pub fn update(
        &mut self,
        definition: &BlockDefinition,
        registory: &mut DefinitionRegistory,
        state: &BlockViewState,
    ) {
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
            let voxels = VoxelPosition::get_voxels(definition, registory, state.show_child_body());
            let (voxel_min, voxel_max) = VoxelPosition::get_bounds(&voxels);
            let corner_min = voxel_min.corner_min().world_pos_lh();
            let corner_max = voxel_max.corner_max().world_pos_lh();
            let center = (corner_min + corner_max) * 0.5;

            let corners =
                VoxelCorner::get_outer_corners(HashSet::from_iter(voxels.iter().cloned()));

            // 中心をバウンディングボックスの中心にしているが、角度により片側に偏って見えてしまうので、できれば直す
            camera.center = center;

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
                        let s = mat_vp.mul_vec4(c.world_pos_lh().extend(1.0));
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
                let view = camera.mat_view();
                let tan = (self.fov_y / 2.0).tan();
                let tan_x = (width - 2.0 * self.margin) / width * tan * aspect_ratio;
                let tan_y = (height - 2.0 * self.margin) / height * tan;
                let len = camera.direction.length();
                corners.iter().fold(0.0, |s: f32, corner| {
                    let view_point = view.transform_point3(corner.world_pos_lh());
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

pub type RenderMessageTuple = (Arc<Definition>, Arc<SwBlockMeshes>, String);

pub struct ImageRenderer {
    definitions: Vec<BlockDefinition>,
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
        definitions: Vec<BlockDefinition>,
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

    pub fn update(&mut self, registory: &mut DefinitionRegistory, state: &BlockViewState) {
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
            let filename = definition.filename();
            if let Some(Err(err)) = definition.load_data() {
                self.logs.push(format!(
                    "Failed to save image of {} due to {}",
                    filename, err
                ));
                self.i += 1;
                self.progress.current = self.i;
                continue;
            }

            let filename = definition.filename();
            if self.scene.update(definition, registory) {
                self.auto_camera.update(definition, registory, state);

                self.framebuffer.before_paint();
                self.renderer.paint(
                    self.framebuffer.gl(),
                    &self.auto_camera.camera,
                    self.scene.appearance(),
                );
                self.framebuffer.after_paint();

                let image = self.framebuffer.get_image();
                let _result = if self.append_filename {
                    image.save(self.save_path.join(replace_extension(
                        &OsString::from(filename.to_string()),
                        "png",
                    )))
                } else {
                    image.save(&self.save_path)
                };

                self.i += 1;
                self.progress.current = self.i;
            }

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

#[derive(serde::Serialize, serde::Deserialize, Clone)]
pub struct SaveImageConfig {
    image: AutoCamera,
    state: BlockViewState,
    appearance: BlockViewAppearance,
}

impl SaveImageConfig {
    pub fn new(auto_camera: AutoCamera, scene: &BlockViewScene) -> Self {
        Self {
            image: auto_camera,
            state: scene.state().clone(),
            appearance: scene.appearance().clone(),
        }
    }

    pub fn apply(self, auto_camera: &mut AutoCamera, scene: &mut BlockViewScene) {
        *auto_camera = self.image;
        scene.set_state(self.state);
        scene.set_appearance(self.appearance);
    }
}
