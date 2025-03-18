use crate::gl_renderer::{Color4, Line, Mesh, SceneObject};
use glam::Vec3;

pub struct BoundingBoxObjectBuilder {
    corner_min: Vec3,
    corner_max: Vec3,
}

impl BoundingBoxObjectBuilder {
    pub fn new(corner_min: impl Into<Vec3>, corner_max: impl Into<Vec3>) -> Self {
        Self {
            corner_min: corner_min.into(),
            corner_max: corner_max.into(),
        }
    }

    pub fn from_voxel(voxel_min: impl Into<Vec3>, voxel_max: impl Into<Vec3>) -> Self {
        let corner_min = (voxel_min.into() - 0.5 * Vec3::ONE) * 0.25;
        let corner_max = (voxel_max.into() + 0.5 * Vec3::ONE) * 0.25;
        Self {
            corner_min,
            corner_max,
        }
    }

    pub fn objects(&self, mesh_color: Color4, line_color: Color4) -> (SceneObject, SceneObject) {
        let min_x = self.corner_min.x;
        let min_y = self.corner_min.y;
        let min_z = self.corner_min.z;
        let max_x = self.corner_max.x;
        let max_y = self.corner_max.y;
        let max_z = self.corner_max.z;
        let positions = vec![
            Vec3::new(min_x, min_y, min_z),
            Vec3::new(min_x, max_y, min_z),
            Vec3::new(min_x, max_y, max_z),
            Vec3::new(min_x, min_y, max_z),
            Vec3::new(max_x, min_y, min_z),
            Vec3::new(max_x, max_y, min_z),
            Vec3::new(max_x, max_y, max_z),
            Vec3::new(max_x, min_y, max_z),
        ];
        let quads = [
            [0, 1, 2, 3],
            [1, 0, 4, 5],
            [2, 1, 5, 6],
            [3, 2, 6, 7],
            [0, 3, 7, 4],
            [7, 6, 5, 4],
        ];
        let strokes = [
            vec![0, 1, 2, 3, 0],
            vec![0, 4],
            vec![1, 5],
            vec![2, 6],
            vec![3, 7],
            vec![4, 5, 6, 7, 4],
        ];

        (
            SceneObject::from_mesh(
                Mesh::single_color_lh(
                    positions.clone(),
                    quads.iter().map(|q| q.as_slice()).collect(),
                    mesh_color,
                )
                .flat(),
                None,
            ),
            SceneObject::from_line(
                Line::single_color_lh(
                    positions,
                    strokes.iter().map(|s| s.as_slice()).collect(),
                    line_color,
                    2.0,
                ),
                None,
            ),
        )
    }
}
