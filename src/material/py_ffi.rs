use pyo3::prelude::*;
use pyo3::PyResult;

use super::*;

use super::texture::py_ffi::*;

#[pyclass(module = "objset")]
#[derive(Debug, Clone)]
pub struct PyMaterial {
    #[pyo3(get, set)]
    pub shader: String,
    // shader_flags: ShaderFlags,
    #[pyo3(get, set)]
    pub textures: Vec<PyTexture>,
    // blend_flags: BlendFlags,
    // colors: Color,
    #[pyo3(get, set)]
    pub name: String,
    #[pyo3(get, set)]
    pub bump_depth: f32,
}

impl From<Material> for PyMaterial {
    fn from(mat: Material) -> Self {
        let Material {
            name,
            bump_depth,
            textures,
            shader,
        } = mat;
        let textures = textures
            .iter()
            .filter(|x| x.is_some())
            .cloned()
            .map(|x| x.unwrap().into())
            .collect();
        let shader = shader.as_str().to_string();
        Self {
            name,
            bump_depth,
            textures,
            shader,
        }
    }
}

#[pymethods]
impl PyMaterial {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "PyMaterial {}: {} {} texture(s)",
            self.name,
            self.shader,
            self.textures.len()
        ))
    }
}
