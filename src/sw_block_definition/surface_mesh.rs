use super::definition_schema;
use crate::gl_renderer::{Color4, Mesh, SceneObject};
use glam::{Mat4, Quat, Vec3};
use std::f32::consts::PI;

const SURFACE_COLOR_GREY: Color4 = Color4 {
    r: 0.304987,
    g: 0.304987,
    b: 0.304987,
    a: 1.0,
};

const SURFACE_COLOR_BLACK: Color4 = Color4 {
    r: 0.051269,
    g: 0.051269,
    b: 0.051269,
    a: 1.0,
};

/*
const PIPE_COLOR_FLUID: Color4 = Color4 {
    r: 0.0,
    g: 0.215861,
    b: 0.215861,
    a: 1.0,
};

const PIPE_COLOR_POWER: Color4 = Color4 {
    r: 1.0,
    g: 0.366253,
    b: 0.0,
    a: 1.0,
};
*/

pub fn create_surface_object(surface: &definition_schema::Surface) -> Option<SceneObject> {
    let rotation = Quat::from_rotation_x(-PI / 2.0 * surface.rotation.unwrap_or(0) as f32);

    let orientation = match surface.orientation {
        Some(1) => Quat::from_rotation_z(PI),
        Some(2) => Quat::from_rotation_z(PI / 2.0),
        Some(3) => Quat::from_rotation_z(-PI / 2.0),
        Some(4) => Quat::from_rotation_x(-PI / 2.0).mul_quat(Quat::from_rotation_z(PI / 2.0)),
        Some(5) => Quat::from_rotation_x(PI / 2.0).mul_quat(Quat::from_rotation_z(PI / 2.0)),
        _ => Quat::IDENTITY,
    };

    let translation = match surface.position.last() {
        Some(position) => {
            0.25 * Vec3::new(position.x as f32, position.y as f32, -position.z as f32)
        }
        None => Vec3::ZERO,
    };

    let mesh = surface_mesh_single_color(surface.shape?, Color4::WHITE)
        .or_else(|| surface_mesh_multiple_color(surface.shape?, Color4::WHITE));

    Some(SceneObject::from_mesh(
        mesh?,
        Some(Mat4::from_rotation_translation(
            orientation.mul_quat(rotation),
            translation,
        )),
    ))
}

fn surface_mesh_single_color(shape: i32, color: Color4) -> Option<Mesh> {
    let (vertices, triangles) = match shape {
        1 => (
            vec![
                Vec3::new(0.125, 0.125, 0.125),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        2 => (
            vec![
                Vec3::new(0.125, 0.125, 0.125),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
            ],
            vec![[0, 1, 2]],
        ),
        6 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.125),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(-0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        7 => (
            vec![
                Vec3::new(-0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(-0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2]],
        ),
        8 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.125),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2]],
        ),
        9 => (
            vec![
                Vec3::new(0.125, 0.125, 0.125),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.0),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        10 => (
            vec![
                Vec3::new(0.125, 0.125, 0.0),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
            ],
            vec![[0, 1, 2]],
        ),
        11 => (
            vec![
                Vec3::new(0.125, 0.125, 0.0),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        12 => (
            vec![
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.0),
            ],
            vec![[0, 1, 2]],
        ),
        13 => (
            vec![
                Vec3::new(0.125, 0.125, 0.125),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.0625),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        14 => (
            vec![
                Vec3::new(0.125, 0.125, 0.0625),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.0),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        15 => (
            vec![
                Vec3::new(0.125, 0.125, 0.0),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.0625),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        16 => (
            vec![
                Vec3::new(0.125, 0.125, -0.0625),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
            ],
            vec![[0, 1, 2]],
        ),
        17 => (
            vec![
                Vec3::new(0.125, 0.125, 0.0625),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        18 => (
            vec![
                Vec3::new(0.125, 0.125, 0.0),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.0625),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        19 => (
            vec![
                Vec3::new(0.125, 0.125, -0.0625),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.0),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        20 => (
            vec![
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.0625),
            ],
            vec![[0, 1, 2]],
        ),
        21 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.125),
                Vec3::new(0.125, 0.125, 0.0),
                Vec3::new(0.125, -0.125, 0.0),
                Vec3::new(-0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        22 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.0),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(-0.125, -0.125, 0.0),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        23 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.125),
                Vec3::new(0.125, 0.125, 0.0625),
                Vec3::new(0.125, -0.125, 0.0625),
                Vec3::new(-0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        24 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.0625),
                Vec3::new(0.125, 0.125, 0.0),
                Vec3::new(0.125, -0.125, 0.0),
                Vec3::new(-0.125, -0.125, 0.0625),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        25 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.0),
                Vec3::new(0.125, 0.125, -0.0625),
                Vec3::new(0.125, -0.125, -0.0625),
                Vec3::new(-0.125, -0.125, 0.0),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        26 => (
            vec![
                Vec3::new(-0.125, 0.125, -0.0625),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(-0.125, -0.125, -0.0625),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        27 => (
            vec![
                Vec3::new(-0.125, 0.125, -0.125),
                Vec3::new(0.125, 0.0, -0.125),
                Vec3::new(0.125, -0.125, 0.0),
                Vec3::new(-0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        28 => (
            vec![
                Vec3::new(-0.125, 0.0, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(-0.125, -0.125, 0.0),
            ],
            vec![[0, 1, 2]],
        ),
        29 => (
            vec![
                Vec3::new(-0.125, 0.125, -0.125),
                Vec3::new(0.125, 0.0625, -0.125),
                Vec3::new(0.125, -0.125, 0.0625),
                Vec3::new(-0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        30 => (
            vec![
                Vec3::new(-0.125, 0.0625, -0.125),
                Vec3::new(0.125, 0.0, -0.125),
                Vec3::new(0.125, -0.125, 0.0),
                Vec3::new(-0.125, -0.125, 0.0625),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        31 => (
            vec![
                Vec3::new(-0.125, 0.0, -0.125),
                Vec3::new(0.125, -0.0625, -0.125),
                Vec3::new(0.125, -0.125, -0.0625),
                Vec3::new(-0.125, -0.125, 0.0),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        32 => (
            vec![
                Vec3::new(-0.125, -0.0625, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(-0.125, -0.125, -0.0625),
            ],
            vec![[0, 1, 2]],
        ),
        33 => (
            vec![
                Vec3::new(0.0, 0.125, 0.125),
                Vec3::new(-0.125, 0.125, -0.125),
                Vec3::new(0.0, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        34 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.125),
                Vec3::new(-0.125, -0.125, -0.125),
                Vec3::new(0.0, -0.125, 0.125),
            ],
            vec![[0, 1, 2]],
        ),
        35 => (
            vec![
                Vec3::new(0.0, 0.125, 0.125),
                Vec3::new(-0.0625, 0.125, -0.125),
                Vec3::new(0.0625, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        36 => (
            vec![
                Vec3::new(-0.0625, 0.125, 0.125),
                Vec3::new(-0.125, 0.125, -0.125),
                Vec3::new(0.0, -0.125, -0.125),
                Vec3::new(0.0625, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        37 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.125),
                Vec3::new(-0.125, 0.0, -0.125),
                Vec3::new(-0.0625, -0.125, -0.125),
                Vec3::new(0.0, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        38 => (
            vec![
                Vec3::new(-0.125, 0.0, 0.125),
                Vec3::new(-0.125, -0.125, -0.125),
                Vec3::new(-0.0625, -0.125, 0.125),
            ],
            vec![[0, 1, 2]],
        ),
        39 => (
            vec![
                Vec3::new(0.125, 0.125, 0.125),
                Vec3::new(0.0625, 0.125, -0.125),
                Vec3::new(-0.0625, -0.125, -0.125),
                Vec3::new(0.0, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        40 => (
            vec![
                Vec3::new(0.0625, 0.125, 0.125),
                Vec3::new(0.0, 0.125, -0.125),
                Vec3::new(-0.125, -0.125, -0.125),
                Vec3::new(-0.0625, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        41 => (
            vec![
                Vec3::new(0.0, 0.125, 0.125),
                Vec3::new(-0.0625, 0.125, -0.125),
                Vec3::new(-0.125, 0.0, -0.125),
                Vec3::new(-0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        42 => (
            vec![
                Vec3::new(-0.0625, 0.125, 0.125),
                Vec3::new(-0.125, 0.125, -0.125),
                Vec3::new(-0.125, 0.0, 0.125),
            ],
            vec![[0, 1, 2]],
        ),
        43 => (
            vec![
                Vec3::new(0.0625, 0.125, 0.125),
                Vec3::new(0.0, 0.125, -0.125),
                Vec3::new(0.0625, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        44 => (
            vec![
                Vec3::new(0.0, 0.125, 0.125),
                Vec3::new(-0.0625, 0.125, -0.125),
                Vec3::new(0.0, -0.125, -0.125),
                Vec3::new(0.0625, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        45 => (
            vec![
                Vec3::new(-0.0625, 0.125, 0.125),
                Vec3::new(-0.125, 0.125, -0.125),
                Vec3::new(-0.0625, -0.125, -0.125),
                Vec3::new(0.0, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        46 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.125),
                Vec3::new(-0.125, -0.125, -0.125),
                Vec3::new(-0.0625, -0.125, 0.125),
            ],
            vec![[0, 1, 2]],
        ),
        47 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.125),
                Vec3::new(0.125, 0.125, 0.0),
                Vec3::new(0.125, 0.0, 0.125),
            ],
            vec![[0, 1, 2]],
        ),
        48 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.0),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
                Vec3::new(-0.125, 0.0, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        49 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.125),
                Vec3::new(0.125, 0.125, 0.0625),
                Vec3::new(0.125, 0.0625, 0.125),
            ],
            vec![[0, 1, 2]],
        ),
        50 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.0625),
                Vec3::new(0.125, 0.125, 0.0),
                Vec3::new(0.125, 0.0, 0.125),
                Vec3::new(-0.125, 0.0625, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        51 => (
            vec![
                Vec3::new(-0.125, 0.125, 0.0),
                Vec3::new(0.125, 0.125, -0.0625),
                Vec3::new(0.125, -0.0625, 0.125),
                Vec3::new(-0.125, 0.0, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        52 => (
            vec![
                Vec3::new(-0.125, 0.125, -0.0625),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
                Vec3::new(-0.125, -0.0625, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        53 => (
            vec![
                Vec3::new(0.0, 0.125, 0.125),
                Vec3::new(-0.125, 0.125, -0.125),
                Vec3::new(0.0, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        54 => (
            vec![
                Vec3::new(0.125, 0.125, 0.125),
                Vec3::new(0.0, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
            ],
            vec![[0, 1, 2]],
        ),
        55 => (
            vec![
                Vec3::new(-0.0625, 0.125, 0.125),
                Vec3::new(-0.125, 0.125, -0.125),
                Vec3::new(0.0, -0.125, -0.125),
                Vec3::new(0.0625, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        56 => (
            vec![
                Vec3::new(0.0, 0.125, 0.125),
                Vec3::new(-0.0625, 0.125, -0.125),
                Vec3::new(0.0625, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        57 => (
            vec![
                Vec3::new(0.0625, 0.125, 0.125),
                Vec3::new(0.0, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, 0.0, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        58 => (
            vec![
                Vec3::new(0.125, 0.125, 0.125),
                Vec3::new(0.0625, 0.125, -0.125),
                Vec3::new(0.125, 0.0, -0.125),
            ],
            vec![[0, 1, 2]],
        ),
        59 => (
            vec![
                Vec3::new(0.0625, 0.125, 0.125),
                Vec3::new(0.0, 0.125, -0.125),
                Vec3::new(-0.125, -0.125, -0.125),
                Vec3::new(-0.0625, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        60 => (
            vec![
                Vec3::new(0.125, 0.125, 0.125),
                Vec3::new(0.0625, 0.125, -0.125),
                Vec3::new(-0.0625, -0.125, -0.125),
                Vec3::new(0.0, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        61 => (
            vec![
                Vec3::new(0.125, 0.0, 0.125),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.0, -0.125, -0.125),
                Vec3::new(0.0625, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        62 => (
            vec![
                Vec3::new(0.125, 0.0, -0.125),
                Vec3::new(0.0625, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2]],
        ),
        63 => (
            vec![
                Vec3::new(-0.0625, 0.125, 0.125),
                Vec3::new(-0.125, 0.125, -0.125),
                Vec3::new(-0.0625, -0.125, -0.125),
                Vec3::new(0.0, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        64 => (
            vec![
                Vec3::new(0.0, 0.125, 0.125),
                Vec3::new(-0.0625, 0.125, -0.125),
                Vec3::new(0.0, -0.125, -0.125),
                Vec3::new(0.0625, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        65 => (
            vec![
                Vec3::new(0.0625, 0.125, 0.125),
                Vec3::new(0.0, 0.125, -0.125),
                Vec3::new(0.0625, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ],
            vec![[0, 1, 2], [0, 2, 3]],
        ),
        66 => (
            vec![
                Vec3::new(0.125, 0.125, 0.125),
                Vec3::new(0.0625, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
            ],
            vec![[0, 1, 2]],
        ),
        _ => return None,
    };

    Some(Mesh::signle_color_lh(vertices, triangles, color))
}

fn surface_mesh_multiple_color(shape: i32, color: Color4) -> Option<Mesh> {
    match shape {
        3 => {
            let center = Vec3::new(0.125, 0.0, 0.0);
            let angle_offset = Some(22.5f32.to_radians());
            let outer_radius = 0.0625 / 22.5f32.to_radians().cos();
            let inner_radius = outer_radius - 0.01;
            Some(Mesh::combined([
                regular_polygon_yz(
                    center,
                    8,
                    inner_radius,
                    None,
                    angle_offset,
                    SURFACE_COLOR_BLACK,
                ),
                regular_polygon_yz(
                    center,
                    8,
                    outer_radius,
                    Some(inner_radius),
                    angle_offset,
                    Color4::WHITE,
                ),
            ]))
        }
        4 | 5 => {
            let mut vertices = Vec::with_capacity(8);
            vertices.append(&mut vec![
                Vec3::new(0.125, 0.125, 0.125),
                Vec3::new(0.125, 0.125, -0.125),
                Vec3::new(0.125, -0.125, -0.125),
                Vec3::new(0.125, -0.125, 0.125),
            ]);
            if shape == 4 {
                vertices.append(&mut vec![
                    Vec3::new(0.125, 0.03125, 0.03125),
                    Vec3::new(0.125, 0.03125, -0.03125),
                    Vec3::new(0.125, -0.03125, -0.03125),
                    Vec3::new(0.125, -0.03125, 0.03125),
                ]);
            } else {
                vertices.append(&mut vec![
                    Vec3::new(0.125, 0.041667, 0.0),
                    Vec3::new(0.125, 0.0, -0.041667),
                    Vec3::new(0.125, -0.041667, 0.0),
                    Vec3::new(0.125, 0.0, 0.041667),
                ]);
            }
            let triangles_color = vec![
                [0, 1, 4],
                [1, 5, 4],
                [1, 2, 5],
                [2, 6, 5],
                [2, 3, 6],
                [3, 7, 6],
                [3, 0, 7],
                [0, 4, 7],
            ];
            let triangles_grey = vec![[4, 5, 6], [4, 6, 7]];
            Some(Mesh::multiple_color_lh(
                vertices,
                vec![
                    (triangles_color, color),
                    (triangles_grey, SURFACE_COLOR_GREY),
                ],
            ))
        }
        _ => None,
    }
}

fn regular_polygon_yz(
    center: Vec3,
    n: usize,
    radius: f32,
    inner_radius: Option<f32>,
    angle_offset: Option<f32>,
    color: Color4,
) -> Mesh {
    let mut vertices = Vec::with_capacity(if inner_radius.is_none() { n } else { 2 * n });

    for i in 0..n {
        let theta = 2.0 * PI / (n as f32) * (i as f32) + angle_offset.unwrap_or(0.0);
        let u = Vec3::new(0.0, theta.sin(), theta.cos());
        vertices.push(center + radius * u);
        if let Some(inner_radius) = inner_radius {
            vertices.push(center + inner_radius * u);
        }
    }

    let triangles: Vec<[usize; 3]> = if inner_radius.is_none() {
        (1..(n - 1)).map(|i| [0, i, i + 1]).collect()
    } else {
        (0..n)
            .flat_map(|i| {
                let i0 = 2 * i;
                let i1 = i0 + 1;
                let i2 = (i0 + 2) % (2 * n);
                let i3 = i2 + 1;
                [[i0, i2, i3], [i0, i3, i1]]
            })
            .collect()
    };

    Mesh::signle_color_lh(vertices, triangles, color)
}
