use pyo3::PyResult;
#[cfg(feature = "pyo3")]
use pyo3::{prelude::*, *};

use super::*;

#[pyclass(module = "objset")]
#[derive(Debug, Default, Clone)]
pub struct PVertexBuffers {
    #[pyo3(get, set)]
    pub positions: Vec<(f32, f32, f32)>,
    #[pyo3(get, set)]
    pub normals: Vec<(f32, f32, f32)>,
    #[pyo3(get, set)]
    pub tangents: Vec<(f32, f32, f32)>,
    #[pyo3(get, set)]
    pub uv1: Vec<(f32, f32)>,
    #[pyo3(get, set)]
    pub uv2: Vec<(f32, f32)>,
    #[pyo3(get, set)]
    pub uv3: Vec<(f32, f32)>,
    #[pyo3(get, set)]
    pub uv4: Vec<(f32, f32)>,
    #[pyo3(get, set)]
    pub color1: Vec<(f32, f32, f32, f32)>,
    #[pyo3(get, set)]
    pub color2: Vec<(f32, f32, f32, f32)>,
    #[pyo3(get, set)]
    pub joint_weights: Vec<(f32, f32, f32, f32)>,
    #[pyo3(get, set)]
    pub joint_indices: Vec<(f32, f32, f32, f32)>,
}

//workaround cause cfg_attr doesn't work on the getter setter shorthand
#[pymethods]
impl BoneWeight {
    #[getter]
    fn get_index(&self) -> PyResult<Option<u16>> {
        Ok(self.index)
    }
    #[getter]
    fn get_weight(&self) -> PyResult<f32> {
        Ok(self.weight)
    }
    #[setter]
    fn set_index(&mut self, value: Option<u16>) -> PyResult<()> {
        self.index = value;
        Ok(())
    }
    #[setter]
    fn set_weight(&mut self, value: f32) -> PyResult<()> {
        self.weight = value;
        Ok(())
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!("BoneWeight {:?}: {}", self.index, self.weight))
    }
}

pub type Index = (u16, u16, u16);

#[pyclass(module = "objset")]
#[derive(Debug, Default, Clone, Copy)]
pub struct PyBoneWeights {
    #[pyo3(get, set)]
    first: BoneWeight,
    #[pyo3(get, set)]
    second: BoneWeight,
    #[pyo3(get, set)]
    third: BoneWeight,
    #[pyo3(get, set)]
    fourth: BoneWeight,
}

#[pyclass(module = "objset")]
#[derive(Debug, Default, Clone)]
pub struct PyMesh {
    #[pyo3(get, set)]
    pub vertex_buffers: PVertexBuffers,
    #[pyo3(get, set)]
    pub submeshes: Vec<PySubMesh>,
    #[pyo3(get, set)]
    pub name: String,
}

#[pyclass(module = "objset")]
#[derive(Debug, Default, Clone)]
pub struct PySubMesh {
    pub bounding_sphere: BoundingSphere,
    #[pyo3(get, set)]
    pub indicies: Vec<Index>,
    #[pyo3(get, set)]
    pub bone_indicies: Vec<u16>, //originally u16
    #[pyo3(get, set)]
    pub material_index: u32, //originally u32
    #[pyo3(get, set)]
    pub mat_uv_indicies: [u8; 8], //originally bool
}

#[pymethods]
impl PyMesh {
    fn get_mesh_indicies(&self) -> Vec<Index> {
        self.submeshes
            .iter()
            .flat_map(|x| x.indicies.clone())
            .collect()
    }
    fn get_submesh_ranges(&self) -> Vec<(usize, usize)> {
        self.submeshes
            .iter()
            .map(|x| x.indicies.len())
            .scan(0, |state, x| {
                let range = (*state, x);
                *state = x;
                Some(range)
            })
            .collect()
    }
    fn get_submesh_vbo(&self, submesh: PySubMesh) -> Option<SubMeshVBO> {
        let set = submesh.get_unqiue_indicies();
        let start = *set.iter().min()?;
        let end = start + *set.iter().max()?;
        let PVertexBuffers {
            positions,
            normals,
            tangents,
            uv1,
            uv2,
            uv3,
            uv4,
            color1,
            color2,
            joint_weights,
            joint_indices,
        } = &self.vertex_buffers;

        let positions = set
            .iter()
            .map(|&x| positions.get(x as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let normals = set
            .iter()
            .map(|&x| normals.get(x as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let tangents = set
            .iter()
            .map(|&x| tangents.get(x as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let uv1 = set
            .iter()
            .map(|&x| uv1.get(x as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let uv2 = set
            .iter()
            .map(|&x| uv2.get(x as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let uv3 = set
            .iter()
            .map(|&x| uv3.get(x as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let uv4 = set
            .iter()
            .map(|&x| uv4.get(x as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let color1 = set
            .iter()
            .map(|&x| color1.get(x as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let color2 = set
            .iter()
            .map(|&x| color2.get(x as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let joint_weights = set
            .iter()
            .map(|&x| joint_weights.get(x as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();
        let joint_indices = set
            .iter()
            .map(|&x| joint_indices.get(x as usize).cloned())
            .collect::<Option<Vec<_>>>()
            .unwrap_or_default();

        Some(SubMeshVBO {
            start,
            end,
            vbo: PVertexBuffers {
                positions,
                normals,
                tangents,
                uv1,
                uv2,
                uv3,
                uv4,
                color1,
                color2,
                joint_weights,
                joint_indices,
            },
        })
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "PyMesh: {} {} submesh(es)",
            self.name,
            self.submeshes.len()
        ))
    }
}

use std::collections::BTreeSet;

#[cfg_attr(feature = "pyo3", pyclass(module = "objset"))]
#[derive(Debug, Default, Clone)]
pub struct SubMeshVBO {
    #[pyo3(get, set)]
    start: u16,
    #[pyo3(get, set)]
    end: u16,
    #[pyo3(get, set)]
    vbo: PVertexBuffers,
}

#[pymethods]
impl PySubMesh {
    fn get_unqiue_indicies(&self) -> BTreeSet<u16> {
        self.indicies
            .iter()
            .flat_map(|(x, y, z)| vec![x, y, z])
            .cloned()
            .collect()
    }
    fn get_new_indices(&self) -> Vec<Index> {
        let unique = self.get_unqiue_indicies();
        let f = |x| unique.iter().position(|y| y == x).unwrap() as u16;
        self.indicies
            .iter()
            .map(|(a, b, c)| (f(a), f(b), f(c)))
            .collect()
    }
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "PySubMesh: {} faces, material id: {}, {} bone(s) rigged",
            self.indicies.len(),
            self.material_index,
            self.bone_indicies.len()
        ))
    }
}

impl From<VertexBuffers> for PVertexBuffers {
    fn from(vbo: VertexBuffers) -> PVertexBuffers {
        let v2 = |x: Vec<Vec2>| x.iter().map(|v| (v.x, v.y)).collect();
        let v3 = |x: Vec<Vec3>| x.iter().map(|v| (v.x, v.y, v.z)).collect();
        let v4 = |x: Vec<Vec4>| x.iter().map(|v| (v.x, v.y, v.z, v.w)).collect();

        let positions = v3(vbo.positions);
        let normals = v3(vbo.normals);
        let tangents = v3(vbo.tangents);
        let uv1 = v2(vbo.uv1);
        let uv2 = v2(vbo.uv2);
        let uv3 = v2(vbo.uv3);
        let uv4 = v2(vbo.uv4);
        let color1 = v4(vbo.color1);
        let color2 = v4(vbo.color2);
        let joint_weights = v4(vbo.joint_weights);
        let joint_indices = v4(vbo.joint_indices);
        Self {
            positions,
            normals,
            tangents,
            uv1,
            uv2,
            uv3,
            uv4,
            color1,
            color2,
            joint_weights,
            joint_indices,
        }
    }
}

#[pymethods]
impl PyBoneWeights {
    fn __repr__(&self) -> PyResult<String> {
        Ok(format!(
            "PyBoneWeights <{:?}: {}, {:?}: {}, {:?}: {}, {:?}: {}>",
            self.first.index,
            self.first.weight,
            self.second.index,
            self.second.weight,
            self.third.index,
            self.third.weight,
            self.fourth.index,
            self.fourth.weight
        ))
    }
}

impl From<BoneWeights> for PyBoneWeights {
    fn from(weights: BoneWeights) -> Self {
        let [first, second, third, fourth] = weights.0;
        Self {
            first,
            second,
            third,
            fourth,
        }
    }
}

impl From<Mesh<'_>> for PyMesh {
    fn from(mesh: Mesh<'_>) -> Self {
        let Mesh {
            vertex_buffers,
            name,
            submeshes,
            bounding_sphere: _,
        } = mesh;
        let name = name.into();
        let vertex_buffers = vertex_buffers.into();
        let submeshes = submeshes.into_iter().map(Into::into).collect();
        Self {
            name,
            vertex_buffers,
            submeshes,
        }
    }
}

impl From<SubMesh> for PySubMesh {
    fn from(submesh: SubMesh) -> Self {
        let SubMesh {
            bounding_sphere,
            primitive,
            indicies,
            bone_indicies,
            mat_uv_indicies,
            material_index,
        } = submesh;
        use PrimitiveType::*;
        let indicies = match primitive {
            Triangle => indicies.chunks(3).map(|x| (x[0], x[1], x[2])).collect(),
            TriangleStrip => {
                tristrips(indicies)
                // tris.into_iter().flat_map(|(a, b, c)| vec![a, b, c]).collect()
            }
            _ => todo!(),
        };
        Self {
            bounding_sphere,
            bone_indicies,
            material_index,
            mat_uv_indicies,
            indicies,
        }
    }
}

fn tristrips(idx: Vec<u16>) -> Vec<Index> {
    let mut vec = vec![];
    for indices in idx.split(|&x| x == 0xFFFF) {
        let mut indices = indices.iter();
        let mut a = *indices.next().unwrap();
        let mut b = *indices.next().unwrap();

        let dir_iter = [1, -1].iter().cycle();
        for (&c, &dir) in indices.zip(dir_iter) {
            if a != b && b != c && a != c {
                if dir > 0 {
                    vec.push((a, b, c));
                } else {
                    vec.push((a, c, b));
                }
            }
            a = b;
            b = c;
        }
    }
    vec
}
