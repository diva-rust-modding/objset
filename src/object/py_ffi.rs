use pyo3::prelude::*;
use pyo3::wrap_pyfunction;
use pyo3::PyObjectProtocol;
use pyo3::PyResult;

use crate::material::py_ffi::*;
use crate::mesh::py_ffi::*;
use crate::skeleton::py_ffi::*;

use std::fs::File;

use super::*;

#[pyclass(module = "objset")]
#[derive(Debug, Default, Clone)]
pub struct PObject {
    #[pyo3(get, set)]
    pub id: usize,
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub materials: Vec<PyMaterial>,
    #[pyo3(get, set)]
    pub meshes: Vec<PyMesh>,
    #[pyo3(get, set)]
    pub skeleton: Option<PySkeleton>,
    // #[pyo3(get, set)]
    pub bounding_sphere: BoundingSphere,
}

#[pyclass(module = "objset")]
#[derive(Debug, Default)]
pub struct PyObjectSet {
    #[pyo3(get, set)]
    pub objects: Vec<PObject>,
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
            skeleton,
            bounding_sphere,
        } = obj;
        let name = name.into();
        let meshes = meshes.into_iter().map(Into::into).collect();
        let skeleton = skeleton.map(|s| PySkeleton::from(s));
        let materials = materials.into_iter().map(Into::into).collect();
        Self {
            name,
            id,
            materials,
            meshes,
            bounding_sphere,
            skeleton,
        }
    }
}

impl From<ObjectSet<'_>> for PyObjectSet {
    fn from(objset: ObjectSet<'_>) -> Self {
        let ObjectSet { objects, tex_ids } = objset;
        let objects = objects.into_iter().map(Into::into).collect();
        Self { objects, tex_ids }
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for PyObjectSet {
    fn __repr__(&'p self) -> PyResult<String> {
        Ok(format!(
            "PObjectSet: {} object(s), {} texture id(s)",
            self.objects.len(),
            self.tex_ids.len()
        ))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for PObject {
    fn __repr__(&'p self) -> PyResult<String> {
        let skel = match self.skeleton {
            Some(_) => " has skeleton",
            None => "",
        };
        Ok(format!(
            "PObject {}: {}{} {} mesh(es) {} mat(s)",
            self.id,
            self.name,
            skel,
            self.meshes.len(),
            self.materials.len()
        ))
    }
}
