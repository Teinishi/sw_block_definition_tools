use crate::gl_renderer;
use byteorder::{LittleEndian, ReadBytesExt};
use core::fmt;
use std::{
    fs,
    io::{self, Cursor, Read},
    path::Path,
};

#[derive(Debug)]
pub struct SwMesh {
    _mesh_type: SwMeshType,
    _vertex_count: u16,
    vertices: Vec<SwMeshVertex>,
    _index_count: u32,
    _triangle_count: u32,
    triangles: Vec<SwMeshTriangle>,
    _submesh_count: u16,
    _submeshes: Vec<SwSubmesh>,
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
            return Err(SwMeshFromFileError::ParseError(
                "File is beginning with unexpected bytes.".to_string(),
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
        let triangle_count = index_count / 3;

        let mut triangles = Vec::with_capacity(triangle_count.try_into().unwrap());
        for _ in 0..triangle_count {
            triangles.push(SwMeshTriangle::from_binary(&mut cur)?);
        }

        let submesh_count = cur.read_u16::<LittleEndian>()?;

        let mut submeshes = Vec::with_capacity(submesh_count.into());
        for _ in 0..submesh_count {
            submeshes.push(SwSubmesh::from_binary(&mut cur)?);
        }

        Ok(Self {
            _mesh_type: mesh_type.unwrap(),
            _vertex_count: vertex_count,
            vertices,
            _index_count: index_count,
            _triangle_count: triangle_count,
            triangles,
            _submesh_count: submesh_count,
            _submeshes: submeshes,
        })
    }

    pub fn into_mesh(&self) -> gl_renderer::Mesh {
        let vertices: Vec<gl_renderer::MeshVertex> =
            self.vertices.iter().map(|v| v.into_mesh_vertex()).collect();
        let triangles = self.triangles.iter().map(|t| t.into_usize_arr()).collect();

        gl_renderer::Mesh {
            vertices,
            triangles,
        }
    }
}

#[derive(Debug)]
pub enum SwMeshType {
    Mesh,
    Phys,
}

#[derive(Debug)]
pub enum SwMeshFromFileError {
    IoError(io::Error),
    Utf8Error(std::str::Utf8Error),
    ParseError(String),
}

impl fmt::Display for SwMeshFromFileError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IoError(err) => err.fmt(f),
            Self::Utf8Error(err) => err.fmt(f),
            Self::ParseError(message) => write!(f, "{}", message),
        }
    }
}

impl From<io::Error> for SwMeshFromFileError {
    fn from(err: io::Error) -> Self {
        SwMeshFromFileError::IoError(err)
    }
}

impl From<std::str::Utf8Error> for SwMeshFromFileError {
    fn from(err: std::str::Utf8Error) -> Self {
        SwMeshFromFileError::Utf8Error(err)
    }
}

#[derive(Debug)]
pub struct SwMeshVertex {
    position: SwMeshVec3,
    color: SwMeshColor4,
    normal: SwMeshVec3,
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

    pub fn into_mesh_vertex(&self) -> gl_renderer::MeshVertex {
        gl_renderer::MeshVertex {
            position: self.position.into_vec3(),
            color: self.color.into_color4(),
            normal: self.normal.into_vec3(),
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

    pub fn into_usize_arr(&self) -> [usize; 3] {
        self.indices.map(|i| i as usize)
    }
}

#[derive(Debug)]
pub struct SwSubmesh {
    _index_buffer_start: u32,
    _index_buffer_length: u32,
    _shader_id: u16,
    _bounds_min: SwMeshVec3,
    _bounds_max: SwMeshVec3,
    _name_len: u16,
    _name: Result<String, std::string::FromUtf8Error>,
}

impl SwSubmesh {
    fn from_binary(cur: &mut Cursor<Vec<u8>>) -> std::io::Result<Self> {
        let index_buffer_start = cur.read_u32::<LittleEndian>()?;
        let index_buffer_length = cur.read_u32::<LittleEndian>()?;
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
            _index_buffer_start: index_buffer_start,
            _index_buffer_length: index_buffer_length,
            _shader_id: shader_id,
            _bounds_min: bounds_min,
            _bounds_max: bounds_max,
            _name_len: name_len,
            _name: String::from_utf8(name),
        })
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

    fn into_vec3(&self) -> glam::Vec3 {
        glam::Vec3::new(self.x, self.y, self.z)
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

    fn into_color4(&self) -> gl_renderer::Color4 {
        gl_renderer::Color4 {
            r: self.r as f32 / 255.0,
            g: self.g as f32 / 255.0,
            b: self.b as f32 / 255.0,
            a: self.a as f32 / 255.0,
        }
    }
}
