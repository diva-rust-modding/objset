use super::*;
use pyo3::{prelude::*, wrap_pyfunction};

#[pyclass(module = "objset")]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PySkeleton {
    #[pyo3(get, set)]
    pub bones: Vec<PyBone>,
}

#[pyclass(module = "objset")]
#[derive(Debug, PartialEq, Clone)]
pub struct PyBone {
    #[pyo3(get, set)]
    pub id: usize,
    #[pyo3(get, set)]
    pub inverse_bind_pose: [f32; 16],
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub parent: Option<usize>,
}

impl From<Skeleton<'_>> for PySkeleton {
    fn from(skel: Skeleton<'_>) -> Self {
        let bones = skel.bones.into_iter().map(Into::into).collect();
        Self { bones }
    }
}

impl From<Bone<'_>> for PyBone {
    fn from(Bone { id, inverse_bind_pose, name, parent }: Bone<'_>) -> Self {
        Self { id, name: name.into(), parent, inverse_bind_pose: inverse_bind_pose.into() }
    }
}
