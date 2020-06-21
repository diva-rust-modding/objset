use pyo3::prelude::*;

use super::*;

#[pyclass(module = "objset")]
#[derive(Debug, Default, Clone)]
pub struct PyMaterial {
    // shader: (),
    // shader_flags: ShaderFlags,
    // textures: [Option<Texture>; 8],
    // blend_flags: BlendFlags,
    // colors: Color,
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub bump_depth: f32,
}

impl From<Material> for PyMaterial {
    fn from(mat: Material) -> Self {
        let Material { name, bump_depth } = mat;
        Self { name, bump_depth }
    }
}
