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

use cgmath::Matrix4;
impl Bone<'_> {
    pub fn local_bind_pose(&self) -> RowMatrix4<f32> {
        let bp: ColumnMatrix4<f32> = self.local_bind_pose_internal().into();
        bp.into()
    }
    fn local_bind_pose_internal(&self) -> Matrix4<f32> {
        use cgmath::SquareMatrix;
        let ibp = Matrix4::from(ColumnMatrix4::from(self.inverse_bind_pose));
        ibp.invert().unwrap()
    }
    fn serial_id(&self) -> usize {
        match self.exdata {
            Some(_) => self.id | EXDATA_ID,
            None => self.id,
        }
    }
}
