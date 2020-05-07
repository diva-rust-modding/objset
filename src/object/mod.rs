use super::*;

use crate::bounding::BoundingSphere;
use crate::mesh::*;
use crate::skeleton::*;

#[cfg(feature = "pyo3")]
pub(crate) mod py_ffi;
mod read;

#[derive(Debug, Default)]
pub struct Object<'a> {
    id: usize,
    name: Cow<'a, str>,
    materials: Vec<Material>,
    meshes: Vec<Mesh<'a>>,
    skin: Skin,
    bounding_sphere: BoundingSphere,
}

#[derive(Debug, Default)]
pub struct ObjectSet<'a> {
    objects: Vec<Object<'a>>,
    skeletons: Vec<Skeleton<'a>>,
    tex_ids: Vec<usize>,
}

#[derive(Debug, Default, Clone)]
pub struct Material();
#[derive(Debug, Default, Clone)]
pub struct Skin();
