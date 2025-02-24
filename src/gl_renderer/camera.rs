use egui::PointerButton;
use glam::{Mat4, Quat, Vec3};

pub trait Camera {
    fn mat_view(&self) -> Mat4;
    fn mat_proj(&self) -> Mat4;
    fn mat_view_proj(&self) -> Mat4 {
        self.mat_proj().mul_mat4(&self.mat_view())
    }
    fn position(&self) -> Vec3;
}

#[derive(Debug)]
pub struct OrbitCamera {
    pub center: Vec3,
    pub direction: Vec3,
    pub up: Vec3,
    pub fov_y: f32,
    pub aspect_ratio: f32,
    pub near_clip: f32,
    pub far_clip: f32,
    pub rotate_speed: f32,
    pub pan_speed: f32,
    pub zoom_speed: f32,
    pub rotate_pointer_button: PointerButton,
    pub pan_pointer_button: PointerButton,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            direction: Vec3::NEG_Z,
            up: Vec3::Y,
            fov_y: 60f32.to_radians(),
            aspect_ratio: 1.0,
            near_clip: 0.025,
            far_clip: 20100.0,
            rotate_speed: 0.005,
            pan_speed: 0.001,
            zoom_speed: 0.1,
            rotate_pointer_button: PointerButton::Secondary,
            pan_pointer_button: PointerButton::Middle,
        }
    }
}

impl Camera for OrbitCamera {
    fn mat_view(&self) -> Mat4 {
        Mat4::look_at_rh(
            self.center - self.direction,
            self.center,
            self.up.normalize(),
        )
    }

    fn mat_proj(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov_y, self.aspect_ratio, self.near_clip, self.far_clip)
    }

    fn position(&self) -> Vec3 {
        self.center - self.direction
    }
}

impl OrbitCamera {
    pub fn orthogonalize_up(&mut self) {
        self.up = (self.up - self.up.project_onto(self.direction)).normalize();
    }

    pub fn control(&mut self, ui: &mut egui::Ui, response: egui::Response) {
        if response.dragged_by(self.rotate_pointer_button) {
            let motion = -self.rotate_speed * response.drag_motion();
            let q = Quat::from_rotation_y(motion.x)
                .mul_quat(Quat::from_axis_angle(self.right_vec(), motion.y));
            self.direction = q.mul_vec3(self.direction);
            self.up = q.mul_vec3(self.up);
        }

        if response.dragged_by(self.pan_pointer_button) {
            let motion = self.pan_speed * self.direction.length() * response.drag_motion();
            self.center += -motion.x * self.right_vec() + motion.y * self.up;
        }

        if response.hovered() {
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
