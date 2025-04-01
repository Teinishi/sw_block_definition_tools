use egui::PointerButton;
use glam::{Affine3A, Mat4, Quat, Vec3, Vec3Swizzles};

const ORTHOGRAPHIC_ZOOM_FACTOR: f32 = 0.05;

pub trait Camera: Default {
    fn mat_view(&self) -> Affine3A;
    fn mat_proj(&self) -> Mat4;
    fn mat_view_proj(&self) -> Mat4 {
        self.mat_proj().mul_mat4(&self.mat_view().into())
    }
    fn position(&self) -> Vec3;
    fn z_offset_unit(&self) -> f32;
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
pub enum CameraMode {
    Perspective,
    Orthographic,
}

#[derive(serde::Serialize, serde::Deserialize, Clone, Debug)]
#[serde(default)]
pub struct OrbitCamera {
    pub center: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    mode: CameraMode,
    pub fov_y: f32,
    #[serde(skip)]
    pub aspect_ratio: f32,
    #[serde(skip)]
    pub near_clip: f32,
    #[serde(skip)]
    pub far_clip: f32,
    #[serde(skip)]
    pub rotate_speed: f32,
    #[serde(skip)]
    pub pan_speed: f32,
    #[serde(skip)]
    pub zoom_speed: f32,
    #[serde(skip)]
    pub rotate_pointer_button: PointerButton,
    #[serde(skip)]
    pub pan_pointer_button: PointerButton,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            direction: Vec3::NEG_Z,
            up: Vec3::Y,
            mode: CameraMode::Perspective,
            fov_y: 45f32.to_radians(),
            aspect_ratio: 1.0,
            near_clip: 0.025,
            far_clip: 20100.0,
            rotate_speed: 0.005,
            pan_speed: 0.4,
            zoom_speed: 0.1,
            rotate_pointer_button: PointerButton::Secondary,
            pan_pointer_button: PointerButton::Middle,
        }
    }
}

impl Camera for OrbitCamera {
    fn mat_view(&self) -> Affine3A {
        Affine3A::look_at_rh(
            self.center - self.direction,
            self.center,
            self.up.normalize(),
        )
    }

    fn mat_proj(&self) -> Mat4 {
        match self.mode {
            CameraMode::Perspective => Mat4::perspective_rh_gl(
                self.fov_y,
                self.aspect_ratio,
                self.near_clip,
                self.far_clip,
            ),
            CameraMode::Orthographic => {
                let zoom = self.direction.length() * ORTHOGRAPHIC_ZOOM_FACTOR;
                Mat4::orthographic_rh_gl(
                    -zoom * self.aspect_ratio,
                    zoom * self.aspect_ratio,
                    -zoom,
                    zoom,
                    self.near_clip,
                    self.far_clip,
                )
            }
        }
    }

    fn position(&self) -> Vec3 {
        self.center - self.direction
    }

    fn z_offset_unit(&self) -> f32 {
        match self.mode {
            CameraMode::Perspective => 0.00001,
            CameraMode::Orthographic => 0.0000005,
        }
    }
}

impl OrbitCamera {
    pub fn new(center: Vec3, direction: Vec3, fov_y: f32, aspect_ratio: f32) -> Self {
        let mut camera = Self {
            center,
            direction,
            fov_y,
            aspect_ratio,
            ..Default::default()
        };
        camera.orthogonalize_up();
        camera
    }

    pub fn azimuth_angle(&self) -> f32 {
        let v = if self.direction.xz().length() > 0.00001 {
            self.direction
        } else {
            -self.direction.y.signum() * self.up
        };
        -v.z.atan2(v.x)
    }

    pub fn elevation_angle(&self) -> f32 {
        -(self.direction.y / self.direction.length()).asin()
    }

    pub fn set_direction_angle(&mut self, azimuth_angle: f32, elevation_angle: f32, distance: f32) {
        self.direction = distance * Vec3::X;
        self.up = Vec3::Y;
        self.rotate(
            Quat::from_rotation_y(azimuth_angle).mul_quat(Quat::from_rotation_z(-elevation_angle)),
        );
    }

    pub fn set_perspective(&mut self) {
        if !matches!(self.mode, CameraMode::Perspective) {
            self.direction /= self.orthographic_distance_ratio();
            self.mode = CameraMode::Perspective;
        }
    }

    pub fn set_orthographic(&mut self) {
        if !matches!(self.mode, CameraMode::Orthographic) {
            self.direction *= self.orthographic_distance_ratio();
            self.mode = CameraMode::Orthographic;
        }
    }

    fn orthographic_distance_ratio(&self) -> f32 {
        // 透視投影から平行投影にするときの、direction の変更倍率
        self.fov_y / 2.0 / ORTHOGRAPHIC_ZOOM_FACTOR
    }

    pub fn set_fov_y(&mut self, value: f32) {
        self.fov_y = value;
    }

    pub fn set_aspect_ratio(&mut self, value: f32) {
        self.aspect_ratio = value;
    }

    pub fn orthogonalize_up(&mut self) {
        self.up = (self.up - self.up.project_onto(self.direction)).normalize();
    }

    pub fn rotate(&mut self, quat: Quat) {
        self.direction = quat.mul_vec3(self.direction);
        self.up = quat.mul_vec3(self.up);
    }

    pub fn control(
        &mut self,
        ui: &mut egui::Ui,
        response: egui::Response,
        rotate: bool,
        pan: bool,
        zoom: bool,
    ) {
        if rotate && response.dragged_by(self.rotate_pointer_button) {
            let motion = -self.rotate_speed * response.drag_motion();
            self.rotate(
                Quat::from_rotation_y(motion.x)
                    .mul_quat(Quat::from_axis_angle(self.right_vec(), motion.y)),
            );
        }

        if pan && response.dragged_by(self.pan_pointer_button) {
            let f = match self.mode {
                CameraMode::Perspective => 1.0,
                CameraMode::Orthographic => 0.125,
            };
            let size = response.rect.height().min(response.rect.width());
            let motion =
                self.direction.length() * self.pan_speed * response.drag_motion() / size * f;
            self.center += -motion.x * self.right_vec() + motion.y * self.up;
        }

        if zoom && response.hovered() {
            let wheel = ui.input(|i| {
                i.events.iter().find_map(|e| match e {
                    egui::Event::MouseWheel {
                        unit,
                        delta,
                        modifiers,
                    } => Some((*unit, *delta, *modifiers)),
                    _ => None,
                })
            });
            if let Some(wheel) = wheel {
                let delta = wheel.1.y;
                self.direction *= 1.0 - self.zoom_speed * delta;
            }
        }
    }

    fn right_vec(&self) -> Vec3 {
        self.direction.cross(self.up).normalize()
    }
}
