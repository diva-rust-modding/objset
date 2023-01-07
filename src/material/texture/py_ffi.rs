use pyo3::prelude::*;
use pyo3::PyResult;

use super::*;

#[pyclass]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PyTexture {
    #[pyo3(get, set)]
    sampler_flags: SamplerFlags,
    #[pyo3(get, set)]
    id: u32,
    #[pyo3(get, set)]
    flags: PyTextureFlags,
    #[pyo3(get, set)]
    ex_shader: [u8; 8], //unknown. Always null
    #[pyo3(get, set)]
    weight: f32, //always 1.0
    #[pyo3(get, set)]
    coordinate_matrix: [f32; 16],
}

#[pyclass]
#[derive(Debug, Default, PartialEq, Clone)]
pub struct PyTextureFlags {
    #[pyo3(get, set)]
    uv_index: Option<u8>,
    #[pyo3(get, set)]
    uv_translation: u8,
    /// 0: None,
    /// 1: Color,
    /// 2: Normal,
    /// 3: Specular,
    /// 4: Height,
    /// 5: Reflection,
    /// 6: Translucency,
    /// 7: Transparency,
    /// 8: Sphere,
    /// 9: Cube,
    #[pyo3(get, set)]
    map: u8,
}

impl From<Texture> for PyTexture {
    fn from(tex: Texture) -> Self {
        let Texture {
            sampler_flags,
            id,
            flags,
            ex_shader,
            weight,
            coordinate_matrix,
        } = tex;
        let flags = flags.into();
        let coordinate_matrix = coordinate_matrix.into();
        Self {
            sampler_flags,
            id,
            flags,
            ex_shader,
            weight,
            coordinate_matrix,
        }
    }
}

impl From<TextureFlags> for PyTextureFlags {
    fn from(flags: TextureFlags) -> Self {
        let TextureFlags {
            map,
            uv_index,
            uv_translation,
        } = flags;
        let map = map as u8;
        let uv_index = uv_index.map(|x| x as u8);
        let uv_translation = uv_translation as u8;
        Self {
            map,
            uv_index,
            uv_translation,
        }
    }
}

#[pymethods]
impl PyTexture {
    fn __repr__(&self) -> PyResult<String> {
        let format = TextureMap::from_byte(self.flags.map).unwrap_or(TextureMap::None);
        Ok(format!("PyTexture {:#X}: {:?} map", self.id, format))
    }
}

#[pymethods]
impl PyTextureFlags {
    fn __repr__(&self) -> PyResult<String> {
        let format = TextureMap::from_byte(self.map).unwrap_or(TextureMap::None);
        Ok(format!(
            "PyTextureFlags: {:?} UV {:?} {}",
            format, self.uv_index, self.uv_translation
        ))
    }
}
