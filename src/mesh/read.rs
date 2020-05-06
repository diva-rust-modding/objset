#[cfg(feature = "pyo3")]
use pyo3::{prelude::*, wrap_pyfunction};

use nom::bytes::complete::take;
use nom::combinator::map;
use nom::multi::count;

use nom::number::Endianness;
use nom::IResult;
use nom_ext::r#trait::*;
use nom_ext::*;

use std::convert::TryFrom;

use super::*;

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
        let (i, val) = take(4usize)(i0)?;
        let val = Self::try_from(val).expect("this should never happen");
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
                    bone_weights,
                    bone_indicies,
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
            let (i, name) = map(take_till(|c| c == 0), String::from_utf8_lossy)(i)?;
            let i = &i[64 - name.len()..];
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
        move |i: &'b [u8]| {
            // println!("SUBMESH BEGIN");
            let cto = |f| count_then_offset(i0, u32_usize(endian), f);
            println!("\t----------SubMesh start----------");
            println!("\t left: {}", 0x12FEE0 - i.len());
            //skip 4 bytes
            let i = &i[4..];
            let (i, bounding_sphere) = BoundingSphere::parse(i, endian)?;
            println!("\t bounds: {:?}", bounding_sphere);
            let (i, material_index) = u32_usize(endian)(i)?;
            let mut mat_uv_indicies = [0; 8];
            mat_uv_indicies.copy_from_slice(&i[..8]);
            let i = &i[8..];
            let (i, bone_indicies) = cto(usize(u16(endian)))(i)?;
            let (i, _bones_per_vert) = u32(endian)(i)?;
            println!("\tmat idx: {} matuv: {:?} bones: {}, bones per vert {}", material_index, mat_uv_indicies, bone_indicies.len(), _bones_per_vert);
            let (i, primitive) = PrimitiveType::parse(endian)(i)?;
            let primitive = primitive.expect("Unknown primtiive type");
            let (i, index_type) = IndexType::parse(endian)(i)?;
            let index_type = index_type.expect("Unknown index type");
            let (i, index_count) = u32_usize(endian)(i)?;
            let len = i.len();
            let (i, indicies) = offset_then(
                i0,
                Primitives::parse(index_type, primitive, index_count, endian),
                endian,
            )(i)?;
            println!("index len {}", index_count);
            let len = len -i.len();
            let i = &i[8..];
            let i = &i[32-8..];
            println!("\tsubmesh end - index len {}", len);
            // println!("{:?} {:?}", bone_indicies, primitive);
            Ok((
                i,
                Self {
                    bounding_sphere,
                    material_index,
                    mat_uv_indicies,
                    bone_indicies,
                    indicies,
                },
            ))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const I: &[u8] = include_bytes!("../../assets/mikitm030_obj.bin");
    const OBJ_OFF: usize = 0x580;
    const MESH_OFF: usize = 0x5D0;
    const SUBMESH_OFF: usize = 0xC90;

    const BOUNDS: BoundingSphere = BoundingSphere {
        center: Vec3 {
            x: 0.,
            y: 1.4760411,
            z: -0.063142225,
        },
        radius: 0.14804693,
    };

    #[test]
    fn read_attrs() {
        let i = &[0x17, 0x0C, 0, 0];
        let (_, attr) = MeshInfoBitField::parse(i).unwrap();
        assert!(attr.get_position());
        assert!(attr.get_normal());
        assert!(attr.get_tangent());
        assert!(attr.get_uv1());
        assert!(attr.get_bone_weight());
        assert!(attr.get_bone_index());
    }

    #[test]
    fn read_mesh() {
        let global = &I[OBJ_OFF..];
        let input = &I[MESH_OFF..];
        let (_, mesh) = Mesh::parse(global, Endianness::Little)(input).unwrap();
        assert_eq!(
            mesh.bounding_sphere,
            BoundingSphere {
                center: Vec3 {
                    x: 0.0,
                    y: 1.4760411,
                    z: -0.063142225
                },
                radius: 0.14804693
            }
        );
        assert_eq!(mesh.vertex_buffers.positions.len(), 1510);
        assert_eq!(mesh.name, "headset_MZ");
    }
    #[test]
    fn read_submesh() {
        let global = &I[OBJ_OFF..];
        let input = &I[SUBMESH_OFF..];
        let (_, submesh) = SubMesh::parse(global, Endianness::Little)(input).unwrap();
        assert_eq!(submesh.bounding_sphere, BOUNDS);
        assert_eq!(submesh.material_index, 4);
        assert_eq!(submesh.mat_uv_indicies, [0; 8]);
        assert_eq!(submesh.bone_indicies, &[0]);
        assert_eq!(
            PrimitiveType::from(submesh.indicies),
            PrimitiveType::TriangleStrip
        );
    }
}
