use nom::number::complete::u32;
use nom::number::Endianness;
use nom::IResult;

use crate::util::read::{at_offset, count_then_offset, offset_then, usize};

use super::*;

impl<'a> Object<'a> {
    pub fn parse(endian: Endianness) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Object<'a>> {
        move |i0: &'a [u8]| {
            let cto = |x| count_then_offset(i0, usize(u32(endian)), x);

            let (i, signature) = u32(endian)(i0)?;
            println!("sig {:#X}", signature);
            //skip 4 bytes
            let i = &i[4..];
            let (i, bounding_sphere) = BoundingSphere::parse(i, endian)?;
            let (i, meshes) = cto(Mesh::parse(i0, endian))(i)?;
            let (i, materials) =
                count_then_offset(i0, usize(u32(endian)), Material::parse(endian))(i)?;
            Ok((
                i,
                Self {
                    meshes,
                    bounding_sphere,
                    materials,
                    ..Default::default()
                },
            ))
        }
    }
}

impl<'a> ObjectSet<'a> {
    pub fn parse(endian: Endianness) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], ObjectSet<'a>> {
        use nom::bytes::complete::take_until;
        use nom::combinator::map;
        use nom::combinator::opt;
        use nom::multi::count;
        move |i0: &'a [u8]| {
            let cstr = map(take_until("\0"), String::from_utf8_lossy);
            let offset_cstr = offset_then(i0, cstr, endian);

            let (i, signature) = u32(endian)(i0)?;
            let (i, object_cnt) = usize(u32(endian))(i)?;
            let (i, bone_cnt) = usize(u32(endian))(i)?;
            let (i, object_tbl_ptr) = usize(u32(endian))(i)?;
            let (i, skel_tbl_ptr) = usize(u32(endian))(i)?;
            let (i, obj_names_ptr) = usize(u32(endian))(i)?;
            let (i, obj_id_ptr) = usize(u32(endian))(i)?;
            let (i, tex_id_ptr) = usize(u32(endian))(i)?;
            let (i, tex_id_cnt) = usize(u32(endian))(i)?;

            dbg!(signature);
            dbg!(object_cnt);
            dbg!(bone_cnt);
            dbg!(object_tbl_ptr);
            dbg!(skel_tbl_ptr);
            dbg!(obj_names_ptr);
            dbg!(obj_id_ptr);
            dbg!(tex_id_ptr);

            let (_, mut objects) = at_offset(
                object_tbl_ptr,
                count(offset_then(i0, Object::parse(endian), endian), object_cnt),
            )(i0)?;
            let (_, skeletons) = at_offset(
                skel_tbl_ptr,
                count(
                    opt(offset_then(i0, Skeleton::parse(i0, endian), endian)),
                    object_cnt,
                ),
            )(i0)?;
            let (_, obj_names) = at_offset(obj_names_ptr, count(offset_cstr, object_cnt))(i0)?;
            let (_, obj_id) = at_offset(obj_id_ptr, count(u32(endian), object_cnt))(i0)?;
            let (_, tex_ids) = at_offset(tex_id_ptr, count(u32(endian), tex_id_cnt))(i0)?;

            for (((obj, name), id), skeleton) in objects
                .iter_mut()
                .zip(obj_names.into_iter())
                .zip(obj_id.into_iter())
                .zip(skeletons.into_iter())
            {
                println!("{}: {}", id, name);
                obj.name = name;
                obj.id = id;
                obj.skeleton = skeleton;
            }

            Ok((
                i,
                Self {
                    signature,
                    objects,
                    tex_ids,
                },
            ))
        }
    }
}

// #[cfg(feature="pyo3")]
// use pyo3::{prelude::*, wrap_pyfunction};

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &[u8] = include_bytes!("../../assets/suzanne_obj.bin");
    const OBJECT: usize = 0x40;

    #[test]
    fn objectset_read() {
        let (_, objset) = ObjectSet::parse(Endianness::Little)(INPUT).unwrap();
        assert_eq!(objset.objects.len(), 1);
        assert_eq!(objset.objects[0].name, "suzanne");
        assert_eq!(objset.objects[0].id, 7);
        assert_eq!(objset.tex_ids.len(), 1);
    }

    #[test]
    fn object_read() {
        let (_, objset) = Object::parse(Endianness::Little)(&INPUT[OBJECT..]).unwrap();
        assert_eq!(objset.materials.len(), 1);
        assert_eq!(objset.meshes.len(), 1);
        assert_eq!(objset.skeleton, None);
    }
}
