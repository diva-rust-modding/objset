#[cfg(feature = "pyo3")]
use pyo3::{prelude::*, *};

use crate::bounding::*;
use crate::primitive::*;
use crate::{Vec2, Vec3, Vec4};

use std::borrow::Cow;

#[cfg(feature = "pyo3")]
pub(crate) mod py_ffi;
mod read;

#[derive(Debug, Default)]
pub struct Mesh<'a> {
    pub bounding_sphere: BoundingSphere,
    pub submeshes: Vec<SubMesh>,
    pub vertex_buffers: VertexBuffers,
    pub name: Cow<'a, str>,
}

#[cfg_attr(feature = "pyo3", pyclass(module = "objset"))]
#[derive(Debug, Default)]
pub struct VertexBuffers {
    pub positions: Vec<Vec3>,
    pub normals: Vec<Vec3>,
    pub tangents: Vec<Vec3>,
    pub uv1: Vec<Vec2>,
    pub uv2: Vec<Vec2>,
    pub uv3: Vec<Vec2>,
    pub uv4: Vec<Vec2>,
    pub color1: Vec<Vec4>,
    pub color2: Vec<Vec4>,
    pub weights: Vec<BoneWeights>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct BoneWeights([BoneWeight; 4]);

#[cfg_attr(feature = "pyo3", pyclass)]
#[derive(Debug, Default, PartialEq, Clone, Copy)]
pub struct BoneWeight {
    index: Option<usize>,
    weight: f32,
}

#[cfg_attr(feature = "pyo3", pymethods)]
impl VertexBuffers {
    fn get_positions(&self) -> Vec<(f32, f32, f32)> {
        self.positions.iter().map(|v| (v.x, v.y, v.z)).collect()
    }
}

#[cfg_attr(feature = "pyo3", pyclass(module = "objset"))]
#[derive(Debug, Default)]
pub struct SubMesh {
    //only available on old formats
    pub bounding_sphere: BoundingSphere,
    pub primitive: PrimitiveType,
    pub indicies: Vec<usize>,
    pub bone_indicies: Vec<usize>, //originally u16
    pub material_index: usize,     //originally u32
    pub mat_uv_indicies: [u8; 8],  //originally bool
}
