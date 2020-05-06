#[cfg(feature="pyo3")]
use pyo3::{prelude::*, wrap_pyfunction, *};

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
    pub bone_weights: Vec<(f32, f32, f32, f32)>,
    #[pyo3(get, set)]
    pub bone_indicies: Vec<(f32, f32, f32, f32)>,
}

pub type Index = (usize, usize, usize);

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

#[cfg_attr(feature="pyo3", pyclass(module = "objset"))]
#[derive(Debug, Default, Clone)]
pub struct PySubMesh {
    pub bounding_sphere: BoundingSphere,
    #[pyo3(get, set)]
    pub indicies: Vec<Index>,
    #[pyo3(get, set)]
    pub bone_indicies: Vec<usize>,   //originally u16
    #[pyo3(get, set)]
    pub material_index: usize,       //originally u32
    #[pyo3(get, set)]
    pub mat_uv_indicies: Vec<usize>, //originally bool
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
        let bone_weights = v4(vbo.bone_weights);
        let bone_indicies = v4(vbo.bone_indicies);
        Self { positions, normals, tangents, uv1, uv2, uv3, uv4, color1, color2, bone_weights, bone_indicies }
    }
}

impl From<Mesh<'_>> for PyMesh {
    fn from(mesh: Mesh<'_>) -> Self {
        let Mesh { vertex_buffers, name, submeshes, bounding_sphere } = mesh;
        let name = name.into();
        let vertex_buffers = vertex_buffers.into();
        let submeshes = submeshes.into_iter().map(Into::into).collect();
        Self { name, vertex_buffers, submeshes }
    }
}

impl From<SubMesh> for PySubMesh {
    fn from(submesh: SubMesh) -> Self {
        let SubMesh { bounding_sphere, indicies, bone_indicies, mat_uv_indicies, material_index } = submesh;
        let  indicies = match indicies {
            Primitives::Triangles(v) => v.into_iter().map(|v| (v.x, v.y, v.z)).collect(),
            Primitives::TriangleStrips(v) => {
                let tristrips = v.into_iter().map(|v| (v.x, v.y, v.z)).collect();
                tristrips_to_tris(tristrips)
                // tris.into_iter().flat_map(|(a, b, c)| vec![a, b, c]).collect()
            }
            _ => todo!()
        };
        Self { bounding_sphere, bone_indicies, material_index, mat_uv_indicies, indicies }
    }
}

fn tristrips_to_tris(idx: Vec<Index>) -> Vec<Index> {
    let idx: Vec<usize> = idx.iter().flat_map(|x| vec![x.0, x.1, x.2]).collect();
    let mut vec: Vec<Index> = vec![];
    let mut dir = -1;
    let (mut a, mut b, mut c) = (0, 0, 0);
    let mut i = 0;
    a = idx[i];
    i += 1;
    b = idx[i];
    i += 1;
    while i < idx.len() {
        c = idx[i];
        i += 1;
        if c == 0xFFFF {
            a = idx[i];
            i += 1;
            b = idx[i];
            i += 1;
            dir = -1;
        } else {
            dir *= -1;
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


// fn create_pymesh(mesh: Mesh<'_>) -> PyMesh {
//     let index = match &mesh.submeshes[0].indicies {
//         Primitives::TriangleStrips(v) => v,
//         _ => todo!()
//     };
//     let index = index.iter().map(|v| (v.x, v.y, v.z)).collect();
//     let index = tristrips_to_tris(index);
//     // (mesh.vertex_buffers.into(), index)
//     PyMesh { vertex_buffers:  mesh.vertex_buffers.into(), name: mesh.name.into(), triangles: index}
// } 
