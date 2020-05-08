use super::*;

type Matrix4 = mint::RowMatrix4<f32>;

pub struct Texture {
    sampler_flags: SamplerFlags,
    id: usize,
    flags: TextureFlags,
    ex_shader: [u8; 8],
    weight: f32,
    coordinate_matrix: Matrix4,
}

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

pub struct TextureFlags {
    map: TextureMap,
    uv_index: UvIndex,
    uv_translation: UvTranslationType,
}

pub enum UvTranslationType {
    None,
    Uv,
    Sphere, //Enviornmental
    Cube,   //Enviornmental
}

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

pub enum UvIndex {
    None = 15,
    Index0 = 0,
    Index1,
    Index2,
    Index3,
    Index4,
    Index5,
    Index6,
    Index7,
}
