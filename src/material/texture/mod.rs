use super::*;

#[cfg(feature = "pyo3")]
pub mod py_ffi;
mod read;
#[cfg(test)]
mod test;

type Matrix4 = mint::RowMatrix4<f32>;

#[derive(Debug, Clone, PartialEq)]
pub struct Texture {
    pub sampler_flags: SamplerFlags,
    pub id: u32,
    pub flags: TextureFlags,
    ex_shader: [u8; 8], //unknown. Always null
    pub weight: f32,    //always 1.0
    pub coordinate_matrix: Matrix4,
}

#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

#[cfg_attr(feature = "pyo3", pyclass)]
#[derive(Debug, Default, Clone, PartialEq)]
pub struct SamplerFlags {
    u_repeat: bool,
    v_repeat: bool,
    u_mirror: bool,
    v_mirror: bool,
    ignore_alpha: bool,
    blend: u8,       //5 bits
    alpha_blend: u8, //5 bits
    border: bool,
    clamp_to_edge: bool,
    filter: u8,        //3 bits
    mipmap: u8,        //2 bits
    mipmap_bias: u8,   //8 bits
    aniso_filters: u8, //2 bits
}

#[derive(Debug, Clone, PartialEq)]
pub struct TextureFlags {
    pub map: TextureMap,
    pub uv_index: Option<UvIndex>,
    pub uv_translation: UvTranslationType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UvTranslationType {
    None,
    Uv,
    Sphere, //Enviornmental
    Cube,   //Enviornmental
}

#[derive(Debug, Clone, PartialEq)]
pub enum TextureMap {
    None,
    Color,
    Normal,
    Specular,
    Height,
    Reflection,
    Translucency,
    Transparency,
    Sphere,
    Cube,
}

#[derive(Debug, Clone, PartialEq)]
pub enum UvIndex {
    Index0 = 0,
    Index1,
    Index2,
    Index3,
    Index4,
    Index5,
    Index6,
    Index7,
}

impl UvIndex {
    const NONE: u32 = 15;
}
