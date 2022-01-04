#[cfg(feature = "pyo3")]
use pyo3::prelude::*;

use nom::bytes::complete::take;
use nom::combinator::map;
use nom::multi::count;

use nom::number::Endianness;
use nom::IResult;
use nom_ext::r#trait::*;
use nom_ext::*;

use std::convert::TryFrom;

use super::*;
use crate::read::*;

use modular_bitfield::prelude::*;
#[bitfield]
#[derive(Debug)]
struct MeshInfoBitField {
    pub position: bool,
    pub normal: bool,
    pub tangent: bool,
    reserved: bool,
    pub uv1: bool,
    pub uv2: bool,
    pub uv3: bool,
    pub uv4: bool,
    pub color1: bool,
    pub color2: bool,
    pub bone_weight: bool,
    pub bone_index: bool,
    reserved1: B20,
}

impl MeshInfoBitField {
    fn parse(i0: &[u8]) -> IResult<&[u8], Self> {
        use std::convert::TryInto;
        let (i, val) = take(4usize)(i0)?;
        let val = Self::from_bytes(val.try_into().unwrap());
        Ok((i, val))
    }
}

struct MeshInfoOffsets {
    position: usize,
    normal: usize,
    tangent: usize,
    uv1: usize,
    uv2: usize,
    uv3: usize,
    uv4: usize,
    color1: usize,
    color2: usize,
    bone_weights: usize,
    bone_indicies: usize,
}

impl MeshInfoOffsets {
    fn parse(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], Self> {
        move |i0: &[u8]| {
            let offset = u32_usize(endian);
            let (i, position) = offset(i0)?;
            let (i, normal) = offset(i)?;
            let (i, tangent) = offset(i)?;
            //skip 4 bytes
            let i = &i[4..];
            let (i, uv1) = offset(i)?;
            let (i, uv2) = offset(i)?;
            let (i, uv3) = offset(i)?;
            let (i, uv4) = offset(i)?;
            let (i, color1) = offset(i)?;
            let (i, color2) = offset(i)?;
            let (i, bone_weights) = offset(i)?;
            let (i, bone_indicies) = offset(i)?;
            //skip the reserved section
            let i = &i[15 * 4..];
            Ok((
                i,
                Self {
                    position,
                    normal,
                    tangent,
                    uv1,
                    uv2,
                    uv3,
                    uv4,
                    color1,
                    color2,
                    bone_weights,
                    bone_indicies,
                },
            ))
        }
    }
}

fn vec2(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], Vec2> {
    use nom::number::complete::{be_f32, le_f32};
    use nom::sequence::tuple;
    move |i0: &[u8]| {
        let f32 = if endian == Endianness::Little {
            le_f32
        } else {
            be_f32
        };
        let (i, (x, y)) = tuple((f32, f32))(i0)?;
        Ok((i, Vec2 { x, y }))
    }
}
fn vec3(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], Vec3> {
    use nom::number::complete::{be_f32, le_f32};
    use nom::sequence::tuple;
    move |i0: &[u8]| {
        let f32 = if endian == Endianness::Little {
            le_f32
        } else {
            be_f32
        };
        let (i, (x, y, z)) = tuple((f32, f32, f32))(i0)?;
        Ok((i, Vec3 { x, y, z }))
    }
}
fn vec4(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], Vec4> {
    use nom::number::complete::{be_f32, le_f32};
    use nom::sequence::tuple;
    move |i0: &[u8]| {
        let f32 = if endian == Endianness::Little {
            le_f32
        } else {
            be_f32
        };
        let (i, (x, y, z, w)) = tuple((f32, f32, f32, f32))(i0)?;
        Ok((i, Vec4 { x, y, z, w }))
    }
}

fn vec4i(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], mint::Vector4<usize>> {
    use nom::number::complete::{be_f32, le_f32};
    use nom::sequence::tuple;
    move |i0: &[u8]| {
        let f32 = |x| u32_usize(endian)(x);
        let (i, (x, y, z, w)) = tuple((f32, f32, f32, f32))(i0)?;
        Ok((i, mint::Vector4 { x, y, z, w }))
    }
}

impl VertexBuffers {
    fn parse<'b, 'a: 'b>(
        cnt: usize,
        offests: MeshInfoOffsets,
        endian: Endianness,
    ) -> impl Fn(&'b [u8]) -> IResult<&'b [u8], Self> {
        move |i0: &[u8]| {
            let v2 = |x| {
                if x != 0 {
                    at_offset(x, count(vec2(endian), cnt))(i0)
                } else {
                    Ok((i0, vec![]))
                }
            };
            let v3 = |x| {
                if x != 0 {
                    at_offset(x, count(vec3(endian), cnt))(i0)
                } else {
                    Ok((i0, vec![]))
                }
            };
            let v4 = |x| {
                if x != 0 {
                    at_offset(x, count(vec4(endian), cnt))(i0)
                } else {
                    Ok((i0, vec![]))
                }
            };

            let positions = v3(offests.position)?.1;
            let normals = v3(offests.normal)?.1;
            let tangents = v3(offests.tangent)?.1;
            let uv1 = v2(offests.uv1)?.1;
            let uv2 = v2(offests.uv2)?.1;
            let uv3 = v2(offests.uv3)?.1;
            let uv4 = v2(offests.uv4)?.1;
            let color1 = v4(offests.color1)?.1;
            let color2 = v4(offests.color2)?.1;
            let bone_weights = v4(offests.bone_weights)?.1;
            let bone_indicies = v4(offests.bone_indicies)?.1;

            let id = |x| if x != -1. { Some(x as u16) } else { None };

            let weights = bone_weights
                .into_iter()
                .zip(bone_indicies.into_iter())
                .map(|(w, i)| {
                    BoneWeights([
                        BoneWeight {
                            index: id(i.x),
                            weight: w.x,
                        },
                        BoneWeight {
                            index: id(i.y),
                            weight: w.y,
                        },
                        BoneWeight {
                            index: id(i.z),
                            weight: w.z,
                        },
                        BoneWeight {
                            index: id(i.w),
                            weight: w.w,
                        },
                    ])
                })
                .collect();

            Ok((
                i0,
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
                    weights,
                },
            ))
        }
    }
}

impl<'b> Mesh<'b> {
    pub fn parse<'a: 'b>(
        i0: &'a [u8],
        endian: Endianness,
    ) -> impl Fn(&'b [u8]) -> IResult<&'b [u8], Mesh<'b>> {
        use nom::bytes::complete::*;
        use nom::sequence::tuple;
        move |i: &'b [u8]| {
            let cto = |f| count_then_offset(i0, u32_usize(endian), f);
            //skip 4 bytes
            println!("----------Mesh start----------");
            let i = &i[4..];
            let (i, bounding_sphere) = BoundingSphere::parse(i, endian)?;
            let (_, (submeshes_cnt, submeshes_ptr)) = tuple((u32(endian), u32(endian)))(i)?;
            dbg!(submeshes_cnt);
            dbg!(submeshes_ptr);
            let (i, submeshes) = cto(SubMesh::parse(i0, endian))(i)?;
            let (i, _attr) = MeshInfoBitField::parse(i)?;
            let (i, _stride) = u32(endian)(i)?;
            let (i, vert_count) = u32_usize(endian)(i)?;
            let (i, offsets) = MeshInfoOffsets::parse(endian)(i)?;
            let (_, vertex_buffers) = VertexBuffers::parse(vert_count, offsets, endian)(i0)?;
            let i = &i[4..];
            let (i, name) = string64(i)?;
            println!("{}", name);
            // println!("vert {} normals {} tangents {}", vertex_buffers.positions.len(), vertex_buffers.normals.len(), vertex_buffers.tangents.len());
            // println!("first vert {:?}", vertex_buffers.positions[0]);
            Ok((
                i,
                Self {
                    name,
                    vertex_buffers,
                    submeshes,
                    bounding_sphere,
                },
            ))
        }
    }
}

impl SubMesh {
    fn parse<'b, 'a: 'b>(
        i0: &'a [u8],
        endian: Endianness,
    ) -> impl Fn(&'b [u8]) -> IResult<&'b [u8], Self> {
        use nom::sequence::tuple;
        move |i: &'b [u8]| {
            let cto = |f| count_then_offset(i0, u32_usize(endian), f);
            let (i, _unused_flags) = u32(endian)(i)?;
            let (i, bounding_sphere) = BoundingSphere::parse(i, endian)?;
            let (i, material_index) = u32(endian)(i)?;
            let mut mat_uv_indicies = [0; 8];
            mat_uv_indicies.copy_from_slice(&i[..8]);
            let i = &i[8..];
            let (i, bone_indicies) = cto(u16(endian))(i)?;
            let (i, _bones_per_vertex) = u32_usize(endian)(i)?;
            let (i, primitive) = PrimitiveType::parse(endian)(i)?;
            let primitive = primitive.expect("Unexpected primitive type found");
            let (i, index_format) = IndexType::parse(endian)(i)?;
            let index_format = index_format.expect("Unexpected index format found");
            println!("{:?}", index_format);
            // let (i, index_cnt) = u32_usize(endian)(i)?;
            // let (i, indicies) = offset_then(
            //     i0,
            //     Primitives::parse(index_format, primitive_type, index_cnt, endian),
            //     endian,
            // )(i)?;
            let (i, indicies) = count_then_offset(i0, u32_usize(endian), u16(endian))(i)?;
            let (i, _flags) = u32(endian)(i)?;
            //skip the reserved data
            let i = &i[6 * 4..];
            let (i, _index_offset) = u32_usize(endian)(i)?;
            Ok((
                i,
                Self {
                    bounding_sphere,
                    primitive,
                    indicies,
                    bone_indicies,
                    material_index,
                    mat_uv_indicies,
                },
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const I: &[u8] = include_bytes!("../../assets/suzanne_obj.bin");
    const OBJ_OFF: usize = 0x40;
    const MESH_OFF: usize = OBJ_OFF + 0x50;
    const SUBMESH_OFF: usize = OBJ_OFF + 296;

    const BOUNDS: BoundingSphere = BoundingSphere {
        center: Vec3 {
            x: 0.,
            y: 0.,
            z: 0.,
        },
        radius: 1.3671875,
    };

    #[test]
    fn read_attrs() {
        let i = &[0x17, 0x0C, 0, 0];
        let (_, attr) = MeshInfoBitField::parse(i).unwrap();
        assert!(attr.position());
        assert!(attr.normal());
        assert!(attr.tangent());
        assert!(attr.uv1());
        assert!(attr.bone_weight());
        assert!(attr.bone_index());
    }

    #[test]
    fn read_mesh() {
        let global = &I[OBJ_OFF..];
        let input = &I[MESH_OFF..];
        let (_, mesh) = Mesh::parse(global, Endianness::Little)(input).unwrap();
        assert_eq!(mesh.bounding_sphere, BOUNDS);
        assert_eq!(mesh.vertex_buffers.positions.len(), 1966);
        assert_eq!(mesh.name, "Suzanne");
    }
    #[test]
    fn read_submesh() {
        let global = &I[OBJ_OFF..];
        let input = &I[SUBMESH_OFF..];
        let (_, submesh) = SubMesh::parse(global, Endianness::Little)(input).unwrap();
        assert_eq!(submesh.bounding_sphere, BOUNDS);
        assert_eq!(submesh.material_index, 0);
        assert_eq!(submesh.mat_uv_indicies, [0; 8]);
        assert_eq!(submesh.bone_indicies, &[0]);
        assert_eq!(submesh.primitive, PrimitiveType::Triangle);
        assert_eq!(submesh.indicies[0], 0);
    }
}
