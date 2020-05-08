pub struct ShaderFlags {
    vertex_translation: VertexTranslationType,
    color_source: ColorSource,
    lambdet: bool,
    phong: bool,
    per_pixel: bool,
    double_sided: bool,
    bump_map: BumpMapType,
    fresnel: u8,
    line_light: u8,
    receive_shadows: bool,
    cast_shadows: bool,
    specular_quality: SpecularQuality,
    aniso_dir: AnisotropicDirection,
}

#[derive(Debug)]
pub enum BlendFactor {
    Zero,
    One,
    SrcColor,
    InvSrcColor,
    SrcAlpha,
    InvSrcAlpha,
    DstAlpha,
    InvDstAlpha,
    DstColor,
    InvDstColor,
}

impl Default for BlendFactor {
    fn default() -> Self {
        Self::Zero
    }
}

pub enum VertexTranslationType {
    Default,
    Envelope,
    Morphing,
}

pub enum ColorSource {
    MaterialColor,
    VertexColor,
    VertexMorph,
}

pub enum BumpMapType {
    None,
    Dot,
    Env,
}

pub enum SpecularQuality {
    Low,
    High,
}

pub enum AnisotropicDirection {
    Normal,
    U,
    V,
    Radial,
}
