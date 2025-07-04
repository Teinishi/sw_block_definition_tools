use crate::sw_gl_3d::{Color4, Line, Mesh, SceneObject, Submesh};
use glam::{Quat, Vec3};
use std::{collections::HashSet, f32::consts::PI, ops};
use strum::VariantArray;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Vec3I {
    x: i32,
    y: i32,
    z: i32,
}

impl Vec3I {
    pub fn new(x: i32, y: i32, z: i32) -> Self {
        Self { x, y, z }
    }

    fn as_vec3(&self) -> Vec3 {
        Vec3::new(self.x as f32, self.y as f32, self.z as f32)
    }

    const X: Self = Vec3I { x: 1, y: 0, z: 0 };
    const Y: Self = Vec3I { x: 0, y: 1, z: 0 };
    const Z: Self = Vec3I { x: 0, y: 0, z: 1 };
}

impl From<&(i32, i32, i32)> for Vec3I {
    fn from(value: &(i32, i32, i32)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}

impl ops::Add<Vec3I> for Vec3I {
    type Output = Self;
    fn add(self, rhs: Vec3I) -> Self {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::Sub<Vec3I> for Vec3I {
    type Output = Self;
    fn sub(self, rhs: Vec3I) -> Self::Output {
        Self {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VariantArray)]
enum FaceOrientation {
    PosX,
    NegX,
    PosY,
    NegY,
    PosZ,
    NegZ,
}

impl FaceOrientation {
    fn normal(&self) -> Vec3I {
        match self {
            Self::PosX => Vec3I::new(1, 0, 0),
            Self::NegX => Vec3I::new(-1, 0, 0),
            Self::PosY => Vec3I::new(0, 1, 0),
            Self::NegY => Vec3I::new(0, -1, 0),
            Self::PosZ => Vec3I::new(0, 0, 1),
            Self::NegZ => Vec3I::new(0, 0, -1),
        }
    }

    fn rotation(&self) -> Quat {
        match self {
            Self::PosX => Quat::IDENTITY,
            Self::NegX => Quat::from_rotation_z(PI),
            Self::PosY => Quat::from_rotation_z(PI / 2.0),
            Self::NegY => Quat::from_rotation_z(-PI / 2.0),
            Self::PosZ => Quat::from_rotation_x(PI / 2.0).mul_quat(Quat::from_rotation_z(PI / 2.0)),
            Self::NegZ => {
                Quat::from_rotation_x(-PI / 2.0).mul_quat(Quat::from_rotation_z(PI / 2.0))
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct VoxelFace {
    position: Vec3I,
    orientation: FaceOrientation,
}

impl VoxelFace {
    fn vertices(&self) -> [Vec3; 4] {
        let rotation = self.orientation.rotation();
        [
            Vec3::new(0.125, 0.125, 0.125),
            Vec3::new(0.125, 0.125, -0.125),
            Vec3::new(0.125, -0.125, -0.125),
            Vec3::new(0.125, -0.125, 0.125),
        ]
        .map(|v| rotation.mul_vec3(v) + 0.25 * self.position.as_vec3())
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, VariantArray)]
enum EdgeDirection {
    X,
    Y,
    Z,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct VoxelEdge {
    position: Vec3I,
    direction: EdgeDirection,
}

impl VoxelEdge {
    fn x(position: Vec3I) -> Self {
        Self {
            position,
            direction: EdgeDirection::X,
        }
    }

    fn y(position: Vec3I) -> Self {
        Self {
            position,
            direction: EdgeDirection::Y,
        }
    }

    fn z(position: Vec3I) -> Self {
        Self {
            position,
            direction: EdgeDirection::Z,
        }
    }

    fn neighbors(&self) -> [Vec3I; 4] {
        let (a, b) = match self.direction {
            EdgeDirection::X => (Vec3I::Y, Vec3I::Z),
            EdgeDirection::Y => (Vec3I::X, Vec3I::Z),
            EdgeDirection::Z => (Vec3I::X, Vec3I::Y),
        };
        [
            self.position,
            self.position - a,
            self.position - b,
            self.position - a - b,
        ]
    }

    fn vertices(&self) -> [Vec3; 2] {
        let d = match self.direction {
            EdgeDirection::X => Vec3I::X,
            EdgeDirection::Y => Vec3I::Y,
            EdgeDirection::Z => Vec3I::Z,
        };
        [
            (self.position.as_vec3() - 0.5 * Vec3::ONE) * 0.25,
            ((self.position + d).as_vec3() - 0.5 * Vec3::ONE) * 0.25,
        ]
    }
}

fn visible_faces(voxels: &[Vec3I]) -> Vec<VoxelFace> {
    let voxel_set: HashSet<Vec3I> = voxels.iter().cloned().collect();
    let mut faces = Vec::new();

    for pos in voxels {
        for orientation in FaceOrientation::VARIANTS {
            let neighbor = *pos + orientation.normal();
            if !voxel_set.contains(&neighbor) {
                faces.push(VoxelFace {
                    position: *pos,
                    orientation: *orientation,
                });
            }
        }
    }

    faces
}

fn edges_of_voxel(position: Vec3I) -> [VoxelEdge; 12] {
    [
        VoxelEdge::x(position),
        VoxelEdge::x(position + Vec3I::Y),
        VoxelEdge::x(position + Vec3I::Z),
        VoxelEdge::x(position + Vec3I::Y + Vec3I::Z),
        VoxelEdge::y(position),
        VoxelEdge::y(position + Vec3I::X),
        VoxelEdge::y(position + Vec3I::Z),
        VoxelEdge::y(position + Vec3I::X + Vec3I::Z),
        VoxelEdge::z(position),
        VoxelEdge::z(position + Vec3I::X),
        VoxelEdge::z(position + Vec3I::Y),
        VoxelEdge::z(position + Vec3I::X + Vec3I::Y),
    ]
}

fn visible_edges(voxels: &[Vec3I]) -> HashSet<VoxelEdge> {
    let voxel_set: HashSet<Vec3I> = voxels.iter().cloned().collect();
    let mut edges = HashSet::new();

    for pos in voxels {
        for edge in edges_of_voxel(*pos) {
            if !edge.neighbors().iter().all(|n| voxel_set.contains(n)) {
                edges.insert(edge);
            }
        }
    }

    edges
}

#[derive(Debug)]
pub struct VoxelMeshBuilder {
    positions: Vec<Vec3I>,
}

impl VoxelMeshBuilder {
    pub fn new<'a, T>(positions: &'a [T]) -> Self
    where
        Vec3I: From<&'a T>,
    {
        Self {
            positions: positions.iter().map(|p| p.into()).collect(),
        }
    }

    pub fn objects(
        &self,
        mesh_color: Color4,
        line_color: Color4,
        line_width: f32,
        z_offset: f32,
    ) -> Vec<SceneObject> {
        let faces = visible_faces(&self.positions);
        let faces_positions: Vec<Vec3> = faces.iter().flat_map(|f| f.vertices()).collect();
        let faces_polygons: Vec<[usize; 4]> = (0..faces.len())
            .map(|i| [4 * i, 4 * i + 1, 4 * i + 2, 4 * i + 3])
            .collect();

        let submesh = Submesh::single_color_lh(
            faces_positions,
            faces_polygons.iter().map(|a| a.as_slice()).collect(),
            mesh_color,
        )
        .flat();
        let obj_mesh = SceneObject::from_mesh(Mesh::from_submeshes(vec![submesh]), None)
            .set_z_offset(z_offset);

        let mut objects = Vec::with_capacity(2);
        objects.push(obj_mesh);

        if line_width > 0.0 {
            let edges: Vec<VoxelEdge> = visible_edges(&self.positions).into_iter().collect();
            let edges_positions: Vec<Vec3> = edges.iter().flat_map(|e| e.vertices()).collect();
            let edges_strokes: Vec<[usize; 2]> =
                (0..edges.len()).map(|i| [2 * i, 2 * i + 1]).collect();

            let line = Line::single_color_lh(
                edges_positions,
                edges_strokes.iter().map(|s| s.as_slice()).collect(),
                line_color,
                line_width,
            );
            let obj_lines = SceneObject::from_line(line, None).set_z_offset(z_offset - 1.0);

            objects.push(obj_lines);
        }

        objects
    }
}
