use mint;

#[cfg(feature="pyo3")]
pub(crate) mod py_ffi;
mod read;
pub mod shader;
pub mod texture;

use self::shader::*;
use self::texture::*;

#[derive(Debug, Default)]
pub struct Material {
    // shader: (),
    // shader_flags: ShaderFlags,
    // textures: [Option<Texture>; 8],
    // blend_flags: BlendFlags,
    // colors: Color,
    pub name: String,
    pub bump_depth: f32,
}

type Rgb = mint::Vector3<f32>;
type Rgba = mint::Vector4<f32>;

#[derive(Debug)]
pub struct Color {
    diffuse: Rgb,
    transparency: f32,
    ambient: Rgba,
    specular: Rgb,
    reflectivity: f32,
    emission: Rgb,
    shininess: f32,
    intensity: f32,
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
