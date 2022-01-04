use nom::bytes::complete::take_until;
use nom::multi::count;
use nom::number::Endianness;
use nom::IResult;
use nom_ext::*;

use crate::Vec4;

use super::*;

impl<'b, 'a: 'b> Skeleton<'a> {
    pub fn parse(
        i0: &'a [u8],
        endian: Endianness,
    ) -> impl Fn(&'b [u8]) -> IResult<&'b [u8], Skeleton<'a>> {
        use nom::combinator::map;
        move |i: &'b [u8]| {
            let cstr = map(take_until("\0"), String::from_utf8_lossy);
            let offset_cstr = offset_then(i0, cstr, endian);

            let (i, id_offset) = u32_usize(endian)(i)?;
            let (i, transform_offset) = u32_usize(endian)(i)?;
            let (i, name_offset) = u32_usize(endian)(i)?;
            let (i, _exp_block_ptr) = u32_usize(endian)(i)?;
            let (i, bone_cnt) = u32_usize(endian)(i)?;
            let (i, parent_offset) = u32_usize(endian)(i)?;

            dbg!(id_offset);
            dbg!(transform_offset);
            dbg!(name_offset);
            dbg!(parent_offset);

            // let read = |offset, f| at_offset(offset, count(f, bone_cnt))(i0);

            let parent = map(i32(endian), |x| if x == -1 { None } else { Some(x as u32) });

            let (_, ids) = at_offset(id_offset, count(u32(endian), bone_cnt))(i0)?;
            let (_, transforms) = at_offset(transform_offset, count(mat44(endian), bone_cnt))(i0)?;
            let (_, names) = at_offset(name_offset, count(offset_cstr, bone_cnt))(i0)?;
            let (_, parents) = at_offset(parent_offset, count(parent, bone_cnt))(i0)?;

            let bones = ids
                .into_iter()
                .zip(transforms.into_iter())
                .zip(names.into_iter())
                .zip(parents.into_iter())
                .map(|(((id, transform), name), parent)| Bone {
                    id,
                    inverse_bind_pose: transform,
                    name,
                    exdata: if id & EXDATA_ID != 0 { Some(()) } else { None },
                    parent,
                })
                .collect();
            Ok((i, Self { bones }))
        }
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

fn mat44(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], RowMatrix4<f32>> {
    use nom::sequence::tuple;
    move |i: &[u8]| {
        let (i, (x, y, z, w)) = tuple((vec4(endian), vec4(endian), vec4(endian), vec4(endian)))(i)?;
        Ok((i, RowMatrix4 { x, y, z, w }))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const I: &[u8] = include_bytes!("../../assets/suzanne_obj.bin");
    const SKEL_OFF: usize = 164768;

    const TRANS: RowMatrix4<f32> = RowMatrix4 {
        x: Vector4 {
            x: 0.01,
            y: -0.0,
            z: 0.0,
            w: -0.0,
        },
        y: Vector4 {
            x: -0.0,
            y: 0.01,
            z: -0.0,
            w: 0.0,
        },
        z: Vector4 {
            x: 0.0,
            y: -0.0,
            z: 0.01,
            w: -0.0,
        },
        w: Vector4 {
            x: -0.0,
            y: 0.0,
            z: -0.0,
            w: 1.0,
        },
    };
    const BONE: Bone<'_> = Bone {
        name: Cow::Borrowed("Bone"),
        inverse_bind_pose: TRANS,
        id: 4294967295,
        exdata: Some(()),
        parent: Some(84288768),
    };

    #[test]
    fn read_skeleton() {
        let input = &I[SKEL_OFF..];
        let (_, skel) = Skeleton::parse(I, Endianness::Little)(input).unwrap();
        assert_eq!(skel.bones.len(), 1);
        assert_eq!(skel.bones[0], BONE);
    }
}
