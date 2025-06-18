use crate::gl_renderer;
use byteorder::{LittleEndian, ReadBytesExt};
use core::fmt;
use glam::Vec3;
use std::{
    fs,
    io::{self, Cursor, Read},
    path::Path,
};

pub type SwMeshResult = Result<SwMesh, SwMeshFromFileError>;

#[derive(Debug)]
pub struct SwMesh {
    pub _mesh_type: SwMeshType,
    pub _vertex_count: u16,
    pub vertices: Vec<SwMeshVertex>,
    pub _index_count: u32,
    pub _triangle_count: u32,
    pub triangles: Vec<SwMeshTriangle>,
    pub _submesh_count: u16,
    pub submeshes: Vec<SwSubmesh>,
}

impl SwMesh {
    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, SwMeshFromFileError> {
        let mut cur = Cursor::new(fs::read(path)?);
        let data4: &mut [u8] = &mut [0; 4];

        cur.read_exact(data4)?;
        let mesh_type = if data4.iter().eq("mesh".as_bytes().iter()) {
            Some(SwMeshType::Mesh)
        } else if data4.iter().eq("phys".as_bytes().iter()) {
            Some(SwMeshType::Phys)
        } else {
            None
        };

        if mesh_type.is_none() {
            return Err(SwMeshFromFileError::Parse(
                "Mesh file is beginning with unexpected bytes.".to_string(),
            ));
        }

        let _header0 = cur.read_u16::<LittleEndian>()?;
        let _header1 = cur.read_u16::<LittleEndian>()?;
        let vertex_count = cur.read_u16::<LittleEndian>()?;
        let _header3 = cur.read_u16::<LittleEndian>()?;
        let _header4 = cur.read_u16::<LittleEndian>()?;

        let mut vertices = Vec::with_capacity(vertex_count.into());
        for _ in 0..vertex_count {
            vertices.push(SwMeshVertex::from_binary(&mut cur)?);
        }

        let index_count = cur.read_u32::<LittleEndian>()?;
        if index_count % 3 != 0 {
            return Err(SwMeshFromFileError::Parse(format!(
                "Mesh file contains unexpected binary sequence: index_count={} is invalid.",
                index_count
            )));
        }
        let triangle_count = index_count / 3;

        let mut triangles = Vec::with_capacity(triangle_count.try_into().unwrap());
        for _ in 0..triangle_count {
            triangles.push(SwMeshTriangle::from_binary(&mut cur)?);
        }

        let submesh_count = cur.read_u16::<LittleEndian>()?;

        let mut submeshes = Vec::with_capacity(submesh_count.into());
        for _ in 0..submesh_count {
            submeshes.push(SwSubmesh::from_binary(&mut cur, index_count)?);
        }

        Ok(Self {
            _mesh_type: mesh_type.unwrap(),
            _vertex_count: vertex_count,
            vertices,
            _index_count: index_count,
            _triangle_count: triangle_count,
            triangles,
            _submesh_count: submesh_count,
            submeshes,
        })
    }
}

#[derive(Debug)]
pub enum SwMeshType {
    Mesh,
    Phys,
}

#[derive(Debug)]
pub enum SwMeshFromFileError {
    Io(io::Error),
    Utf8(std::str::Utf8Error),
    Parse(String),
}

impl fmt::Display for SwMeshFromFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Io(err) => err.fmt(f),
            Self::Utf8(err) => err.fmt(f),
            Self::Parse(message) => write!(f, "{}", message),
        }
    }
}

impl From<io::Error> for SwMeshFromFileError {
    fn from(err: io::Error) -> Self {
        SwMeshFromFileError::Io(err)
    }
}

impl From<std::str::Utf8Error> for SwMeshFromFileError {
    fn from(err: std::str::Utf8Error) -> Self {
        SwMeshFromFileError::Utf8(err)
    }
}

#[derive(Debug)]
pub struct SwMeshVertex {
    pub position: SwMeshVec3,
    pub color: SwMeshColor4,
    pub normal: SwMeshVec3,
}

impl SwMeshVertex {
    fn from_binary(cur: &mut Cursor<Vec<u8>>) -> std::io::Result<Self> {
        let position = SwMeshVec3::from_binary(cur)?;
        let color = SwMeshColor4::from_binary(cur)?;
        let normal = SwMeshVec3::from_binary(cur)?;
        Ok(Self {
            position,
            color,
            normal,
        })
    }

    pub fn as_mesh_vertex(&self) -> gl_renderer::MeshVertex {
        gl_renderer::MeshVertex {
            position: Vec3::new(self.position.x, self.position.y, -self.position.z),
            color: self.color.as_color4(),
            normal: Vec3::new(self.normal.x, self.normal.y, -self.normal.z),
        }
    }
}

#[derive(Debug)]
pub struct SwMeshTriangle {
    indices: [u16; 3],
}

impl SwMeshTriangle {
    fn from_binary(cur: &mut Cursor<Vec<u8>>) -> std::io::Result<Self> {
        let indices = [
            cur.read_u16::<LittleEndian>()?,
            cur.read_u16::<LittleEndian>()?,
            cur.read_u16::<LittleEndian>()?,
        ];
        Ok(Self { indices })
    }

    pub fn as_usize_arr(&self) -> [usize; 3] {
        self.indices.map(|i| i as usize)
    }
}

#[derive(Debug)]
pub struct SwSubmesh {
    pub index_buffer_start: u32,
    pub index_buffer_length: u32,
    pub shader_id: u16,
    pub _bounds_min: SwMeshVec3,
    pub _bounds_max: SwMeshVec3,
    pub _name_len: u16,
    pub _name: Result<String, std::string::FromUtf8Error>,
}

impl SwSubmesh {
    fn from_binary(
        cur: &mut Cursor<Vec<u8>>,
        index_count: u32,
    ) -> Result<Self, SwMeshFromFileError> {
        let index_buffer_start = cur.read_u32::<LittleEndian>()?;
        if index_buffer_start % 3 != 0 || index_buffer_start >= index_count {
            return Err(SwMeshFromFileError::Parse(format!(
                "Mesh file contains unexpected binary sequence: submesh.index_buffer_start={} is invalid.",
                index_buffer_start
            )));
        }
        let index_buffer_length = cur.read_u32::<LittleEndian>()?;
        if index_buffer_length % 3 != 0 || index_buffer_start + index_buffer_length > index_count {
            return Err(SwMeshFromFileError::Parse(format!(
                "Mesh file contains unexpected binary sequence: submesh.index_buffer_length={} is invalid.",
                index_buffer_length
            )));
        }
        let _header2 = cur.read_u16::<LittleEndian>()?;
        let shader_id = cur.read_u16::<LittleEndian>()?;
        let bounds_min = SwMeshVec3::from_binary(cur)?;
        let bounds_max = SwMeshVec3::from_binary(cur)?;
        let _header6 = cur.read_u16::<LittleEndian>()?;
        let name_len = cur.read_u16::<LittleEndian>()?;
        let mut name = Vec::with_capacity(name_len.into());
        for _ in 0..name_len {
            name.push(cur.read_u8()?);
        }
        let _header8 = SwMeshVec3::from_binary(cur)?;
        Ok(Self {
            index_buffer_start,
            index_buffer_length,
            shader_id,
            _bounds_min: bounds_min,
            _bounds_max: bounds_max,
            _name_len: name_len,
            _name: String::from_utf8(name),
        })
    }

    pub fn index_buffer_range(&self) -> std::ops::Range<u32> {
        let start_index = self.index_buffer_start / 3;
        let end_index = start_index + self.index_buffer_length / 3;
        start_index..end_index
    }
}

#[derive(Debug)]
pub struct SwMeshVec3 {
    x: f32,
    y: f32,
    z: f32,
}

impl SwMeshVec3 {
    fn from_binary(cur: &mut Cursor<Vec<u8>>) -> std::io::Result<Self> {
        let x = cur.read_f32::<LittleEndian>()?;
        let y = cur.read_f32::<LittleEndian>()?;
        let z = cur.read_f32::<LittleEndian>()?;
        Ok(Self { x, y, z })
    }
}

#[derive(Debug)]
pub struct SwMeshColor4 {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl SwMeshColor4 {
    fn from_binary(cur: &mut Cursor<Vec<u8>>) -> std::io::Result<Self> {
        let r = cur.read_u8()?;
        let g = cur.read_u8()?;
        let b = cur.read_u8()?;
        let a = cur.read_u8()?;
        Ok(Self { r, g, b, a })
    }

    fn as_color4(&self) -> gl_renderer::Color4 {
        gl_renderer::Color4 {
            r: self.r as f32 / 255.0,
            g: self.g as f32 / 255.0,
            b: self.b as f32 / 255.0,
            a: self.a as f32 / 255.0,
        }
    }
}
