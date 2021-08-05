use mint;

use std::borrow::Cow;

pub mod bounding;
pub mod material;
pub mod mesh;
pub mod object;
pub mod primitive;
#[cfg(feature = "pyo3")]
mod py_ffi;
pub(crate) mod read;
pub mod skeleton;

pub(crate) type Vec2 = mint::Vector2<f32>;
pub(crate) type Vec3 = mint::Vector3<f32>;
pub(crate) type Vec4 = mint::Vector4<f32>;
