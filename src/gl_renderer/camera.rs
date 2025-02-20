use egui::PointerButton;
use glam::{Mat4, Quat, Vec3};

pub trait Camera {
    fn mat_view(&self) -> Mat4;
    fn mat_proj(&self) -> Mat4;
    fn mat_view_proj(&self) -> Mat4 {
        self.mat_proj().mul_mat4(&&self.mat_view())
    }
}

#[derive(Debug)]
pub struct OrbitCamera {
    center: Vec3,
    direction: Vec3,
    up: Vec3,
    fov_y: f32,
    aspect_ratio: f32,
    near_clip: f32,
    far_clip: f32,
    rotate_speed: f32,
    pan_speed: f32,
    zoom_speed: f32,
    rotate_pointer_button: PointerButton,
    pan_pointer_button: PointerButton,
}

impl Default for OrbitCamera {
    fn default() -> Self {
        Self {
            center: Vec3::ZERO,
            direction: Vec3::NEG_Z,
            up: Vec3::Y,
            fov_y: 60f32.to_radians(),
            aspect_ratio: 1.0,
            near_clip: 0.01,
            far_clip: 100.0,
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
        Mat4::look_at_rh(self.center - self.direction, self.center, self.up)
    }

    fn mat_proj(&self) -> Mat4 {
        Mat4::perspective_rh(self.fov_y, self.aspect_ratio, self.near_clip, self.far_clip)
    }
}

impl OrbitCamera {
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
