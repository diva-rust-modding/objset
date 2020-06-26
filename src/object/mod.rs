use super::*;

use crate::bounding::BoundingSphere;
use crate::material::*;
use crate::mesh::*;
use crate::skeleton::*;

#[cfg(feature = "pyo3")]
pub(crate) mod py_ffi;
mod read;

#[derive(Debug, Default)]
pub struct Object<'a> {
    pub id: usize,
    pub name: Cow<'a, str>,
    pub materials: Vec<Material>,
    pub meshes: Vec<Mesh<'a>>,
    pub bounding_sphere: BoundingSphere,
    pub skeleton: Option<Skeleton<'a>>,
}

#[derive(Debug, Default)]
pub struct ObjectSet<'a> {
    pub objects: Vec<Object<'a>>,
    pub tex_ids: Vec<usize>,
}
