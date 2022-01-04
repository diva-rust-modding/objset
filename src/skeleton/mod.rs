use mint::*;

use std::borrow::Cow;

#[cfg(feature = "pyo3")]
pub(crate) mod py_ffi;
pub mod read;

#[derive(Debug, Default, PartialEq)]
pub struct Skeleton<'a> {
    pub bones: Vec<Bone<'a>>,
}

const EXDATA_ID: usize = 0x8000;

#[derive(Debug, PartialEq)]
pub struct Bone<'a> {
    pub id: usize,
    pub inverse_bind_pose: RowMatrix4<f32>,
    pub name: Cow<'a, str>,
    pub exdata: Option<ExData>,
    pub parent: Option<usize>,
}

type ExData = ();

impl Bone<'_> {
    fn serial_id(&self) -> usize {
        match self.exdata {
            Some(_) => self.id | EXDATA_ID,
            None => self.id,
        }
    }
}
