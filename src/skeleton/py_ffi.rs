use super::*;
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::PyObjectProtocol;
use pyo3::PyResult;

#[pyclass]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PySkeleton {
    #[pyo3(get, set)]
    pub bones: Vec<PyBone>,
}

#[pyclass]
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
    pub exdata: Option<ExData>,
}

#[pymethods]
impl PyBone {
    fn bone_db_id(&self) -> usize {
        match self.exdata {
            Some(_) => self.id & (EXDATA_ID - 1),
            None => self.id,
        }
    }
}

#[pymethods]
impl PySkeleton {
    pub fn sort_by_parent(&self) -> Self {
        let mut slf = self.clone();
        slf.bones.sort_by(|x, y| {
            self.parent(x)
                .map(|u| u.id)
                .cmp(&self.parent(y).map(|u| u.id))
        });
        slf
    }
    pub fn parent_idx(&self, bone: &PyBone) -> Option<usize> {
        bone.parent
            .and_then(|b| self.bones.iter().position(|x| x.id == b))
    }
    fn parent(&self, bone: &PyBone) -> Option<PyBone> {
        bone.parent
            .and_then(|b| self.bones.iter().find(|x| x.id == b))
            .cloned()
    }
}

impl From<Skeleton<'_>> for PySkeleton {
    fn from(skel: Skeleton<'_>) -> Self {
        let mut bones = skel.bones.into_iter().map(Into::into).collect();
        Self { bones }
    }
}

impl From<PySkeleton> for Skeleton<'_> {
    fn from(skel: PySkeleton) -> Self {
        let bones = skel.bones.into_iter().map(Into::into).collect();
        Self { bones }
    }
}

impl From<Bone<'_>> for PyBone {
    fn from(
        Bone {
            id,
            inverse_bind_pose,
            name,
            parent,
            exdata,
        }: Bone<'_>,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            parent,
            inverse_bind_pose: inverse_bind_pose.into(),
            exdata,
        }
    }
}

impl From<PyBone> for Bone<'_> {
    fn from(
        PyBone {
            id,
            inverse_bind_pose,
            name,
            parent,
            exdata,
        }: PyBone,
    ) -> Self {
        Self {
            id,
            name: name.into(),
            parent,
            inverse_bind_pose: inverse_bind_pose.into(),
            exdata,
        }
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for PySkeleton {
    fn __repr__(&'p self) -> PyResult<String> {
        Ok(format!("PySkeleton with {} bone(s)", self.bones.len()))
    }
}

#[pyproto]
impl<'p> PyObjectProtocol<'p> for PyBone {
    fn __repr__(&'p self) -> PyResult<String> {
        Ok(format!(
            "PyBone {}: {} parent_id: {:?}",
            self.id, self.name, self.parent
        ))
    }
}
