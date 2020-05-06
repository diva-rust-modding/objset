use pyo3::{prelude::*, wrap_pyfunction};

use crate::mesh::py_ffi::*;
use crate::primitive::*;
use crate::skeleton::py_ffi::*;
use crate::skeleton::*;

use crate::*;

const I: &[u8] = include_bytes!("../assets/mikitm030_obj.bin");
const OBJ_OFF: usize = 0x580;
const MESH_OFF: usize = 0x5D0;
const SUBMESH_OFF: usize = 0xC90;

#[pyclass(module = "objset")]
#[derive(Debug, Default, Clone)]
pub struct PObject {
    #[pyo3(get, set)]
    id: usize,
    #[pyo3(get, set)]
    name: String,
    // #[pyo3(get, set)]
    materials: Vec<Material>,
    #[pyo3(get, set)]
    meshes: Vec<PyMesh>,
    // #[pyo3(get, set)]
    skin: Skin,
    // #[pyo3(get, set)]
    bounding_sphere: BoundingSphere,
}

#[pyclass(module = "objset")]
#[derive(Debug, Default)]
pub struct PyObjectSet {
    #[pyo3(get, set)]
    objects: Vec<PObject>,
    #[pyo3(get, set)]
    skeletons: Vec<PySkeleton>,
    #[pyo3(get, set)]
    tex_ids: Vec<usize>,
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

#[pyfunction]
fn my_rust_function() -> usize {
    42
}

// #[pyfunction]
// fn vertex_buffers() -> Vec<PyMesh> {
//     let global= &I[OBJ_OFF..];
//     let input = &I[MESH_OFF..];
//     let (_, obj) = Object::parse(Endianness::Little)(global).unwrap();
//     for mesh in &obj.meshes {
//         println!("{}", mesh.name);
//     }
//     obj.meshes.into_iter().map(create_pymesh).collect()
// }

use std::fs::File;
use std::io::Read;
#[pyfunction]
fn object_set(path: String) -> PyResult<PyObjectSet> {
    let mut file = File::open(path)?;
    let mut input = vec![];
    file.read_to_end(&mut input);
    let (_, objset) = ObjectSet::parse(Endianness::Little)(&input).unwrap();
    Ok(objset.into())
}

#[pymodule]
fn objset(_py: Python<'_>, m: &PyModule) -> PyResult<()> {
    m.add_wrapped(wrap_pyfunction!(my_rust_function))?;
    // m.add_wrapped(wrap_pyfunction!(vertex_buffers))?;
    m.add_wrapped(wrap_pyfunction!(object_set))?;
    m.add_class::<PObject>()?;
    m.add_class::<PyObjectSet>()?;
    m.add_class::<PySubMesh>()?;
    m.add_class::<PySkeleton>()?;
    m.add_class::<PyBone>()?;
    m.add_class::<PyMesh>()?;
    m.add_class::<SubMesh>()?;
    m.add_class::<VertexBuffers>()?;

    Ok(())
}
