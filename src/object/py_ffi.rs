use pyo3::prelude::*;

use crate::mesh::{py_ffi::*};
use crate::skeleton::{py_ffi::*};

use super::*;

#[pyclass(module = "objset")]
#[derive(Debug, Default, Clone)]
pub struct PObject {
    #[pyo3(get, set)]
    pub id: usize,
    #[pyo3(get, set)]
    pub name: String,
    // #[pyo3(get, set)]
    pub materials: Vec<Material>,
    #[pyo3(get, set)]
    pub meshes: Vec<PyMesh>,
    // #[pyo3(get, set)]
    pub skin: Skin,
    // #[pyo3(get, set)]
    pub bounding_sphere: BoundingSphere,
}

#[pyclass(module = "objset")]
#[derive(Debug, Default)]
pub struct PyObjectSet {
    #[pyo3(get, set)]
    pub objects: Vec<PObject>,
    #[pyo3(get, set)]
    pub skeletons: Vec<PySkeleton>,
    #[pyo3(get, set)]
    pub tex_ids: Vec<usize>,
}

impl From<Object<'_>> for PObject {
    fn from(obj: Object<'_>) -> Self {
        let Object {
            name,
            meshes,
            id,
            materials,
            skin,
            bounding_sphere,
        } = obj;
        let name = name.into();
        let meshes = meshes.into_iter().map(Into::into).collect();
        Self {
            name,
            id,
            materials,
            meshes,
            skin,
            bounding_sphere,
        }
    }
}

impl From<ObjectSet<'_>> for PyObjectSet {
    fn from(objset: ObjectSet<'_>) -> Self {
        let ObjectSet {
            objects,
            skeletons,
            tex_ids,
        } = objset;
        let objects = objects.into_iter().map(Into::into).collect();
        let skeletons = skeletons.into_iter().map(Into::into).collect();
        Self {
            objects,
            skeletons,
            tex_ids,
        }
    }
}
