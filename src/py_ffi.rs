use nom::number::Endianness;
use pyo3::{prelude::*, wrap_pyfunction};

use crate::mesh::py_ffi::*;
use crate::mesh::*;
use crate::object::py_ffi::*;
use crate::object::*;

use crate::skeleton::py_ffi::*;

#[pyfunction]
fn my_rust_function() -> usize {
    42
}

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
