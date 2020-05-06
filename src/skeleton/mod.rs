use mint::*;

use std::borrow::Cow;

#[cfg(feature = "pyo3")]
pub(crate) mod py_ffi;
pub mod read;

#[derive(Debug, Default, PartialEq)]
pub struct Skeleton<'a> {
    pub bones: Vec<Bone<'a>>,
}

#[derive(Debug, PartialEq)]
pub struct Bone<'a> {
    pub id: usize,
    pub inverse_bind_pose: RowMatrix4<f32>,
    pub name: Cow<'a, str>,
    pub parent: Option<usize>,
}

