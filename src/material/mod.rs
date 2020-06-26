use mint;

#[cfg(feature = "pyo3")]
pub(crate) mod py_ffi;
mod read;
pub mod shader;
pub mod texture;

use super::*;

use self::shader::*;
use self::texture::*;

#[derive(Debug, PartialEq)]
pub enum ShaderType {
    Blinn,
    Chara,
    Cloth,
    Eyeball,
    Floor,
    Hair,
    Item,
    Puddle,
    Skin,
    Sky,
    Stage,
    Tights,
    Water01,
}

#[derive(Debug, Default)]
pub struct Material {
    shader: ShaderType,
    // shader_flags: ShaderFlags,
    textures: [Option<Texture>; 8],
    // blend_flags: BlendFlags,
    // colors: Color,
    pub name: String,
    pub bump_depth: f32,
}

type Rgb = mint::Vector3<f32>;
type Rgba = mint::Vector4<f32>;

#[derive(Debug)]
pub struct Color {
    pub diffuse: Rgb,
    pub transparency: f32,
    pub ambient: Rgba,
    pub specular: Rgb,
    ///Controls cubemap reflection
    ///goes from 0 to 1
    pub reflectivity: f32,
    pub emission: Rgb,
    ///Controls specular power
    ///goes from 0 to 128
    pub shininess: f32,
    pub intensity: f32,
}

#[derive(Debug, Default)]
pub struct ColorFlag {
    val: bool,
    alpha: bool,
}

#[derive(Debug, Default)]
pub struct TextureFlags {
    color00: ColorFlag,
    color01: ColorFlag,
    color02: ColorFlag,
    color03: ColorFlag,
    transparency: bool,
    specular: bool,
    normal01: bool,
    normal02: bool,
    enviornment: bool,
    translucency: bool,
    unk: bool,
    ibl_overide: bool,
}

#[derive(Debug, Default)]
pub struct BlendFlags {
    alpha_texture: bool,
    alpha_material: bool,
    masked: bool,
    double_sided: bool,
    normal_direction_light: bool,
    source_blend: BlendFactor,
    destination_blend: BlendFactor,
    blend_op: u8,
    z_bias: u8,
    no_fog: bool,
    unk: u8, //2 bit patterns
}

impl ShaderType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Blinn => "BLINN",
            Self::Chara => "CHARA",
            Self::Cloth => "CLOTH",
            Self::Eyeball => "EYEBALL",
            Self::Floor => "FLOOR",
            Self::Hair => "HAIR",
            Self::Item => "ITEM",
            Self::Puddle => "PUDDLE",
            Self::Skin => "SKIN",
            Self::Sky => "SKY",
            Self::Stage => "STAGE",
            Self::Tights => "TIGHTS",
            Self::Water01 => "WATER01",
        }
    }
}

impl Default for ShaderType {
    fn default() -> Self {
        Self::Blinn
    }
}
