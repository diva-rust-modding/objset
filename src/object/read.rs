use nom::number::Endianness;
use nom::IResult;
use nom_ext::r#trait::*;
use nom_ext::*;

use super::*;

impl<'a> Object<'a> {
    pub fn parse(endian: Endianness) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Object<'a>> {
        move |i0: &'a [u8]| {
            let cto = |x| count_then_offset(i0, u32_usize(endian), x);

            let (i, signature) = u32(endian)(i0)?;
            println!("sig {:#X}", signature);
            //skip 4 bytes
            let i = &i[4..];
            let (i, bounding_sphere) = BoundingSphere::parse(i, endian)?;
            let (i, meshes) = cto(Mesh::parse(i0, endian))(i)?;
            Ok((
                i,
                Self {
                    meshes,
                    bounding_sphere,
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
        use nom::multi::count;
        use nom_ext::*;
        move |i0: &'a [u8]| {
            let cstr = map(take_until("\0"), String::from_utf8_lossy);
            let offset_cstr = offset_then(i0, cstr, endian);

            let (i, sig) = u32(endian)(i0)?;
            let (i, object_cnt) = u32_usize(endian)(i)?;
            let (i, bone_cnt) = u32_usize(endian)(i)?;
            let (i, object_tbl_ptr) = u32_usize(endian)(i)?;
            let (i, skel_tbl_ptr) = u32_usize(endian)(i)?;
            let (i, obj_names_ptr) = u32_usize(endian)(i)?;
            let (i, obj_id_ptr) = u32_usize(endian)(i)?;
            let (i, tex_id_ptr) = u32_usize(endian)(i)?;
            let (i, tex_id_cnt) = u32_usize(endian)(i)?;

            dbg!(sig);
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
                    offset_then(i0, Skeleton::parse(i0, endian), endian),
                    object_cnt,
                ),
            )(i0)?;
            let (_, obj_names) = at_offset(obj_names_ptr, count(offset_cstr, object_cnt))(i0)?;
            let (_, obj_id) = at_offset(obj_id_ptr, count(u32_usize(endian), object_cnt))(i0)?;
            let (_, tex_ids) = at_offset(tex_id_ptr, count(u32_usize(endian), tex_id_cnt))(i0)?;

            for ((obj, name), id) in objects
                .iter_mut()
                .zip(obj_names.into_iter())
                .zip(obj_id.into_iter())
            {
                println!("{}: {}", id, name);
                obj.name = name;
                obj.id = id;
            }

            Ok((
                i,
                Self {
                    objects,
                    skeletons,
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

    const INPUT: &[u8] = include_bytes!("../../assets/mikitm030_obj.bin");
    const RININPUT: &[u8] = include_bytes!("../../assets/rinitm001_obj.bin");
    const OBJECT: usize = 0x580;

    #[test]
    fn objectset_read() {
        let _objset = ObjectSet::parse(Endianness::Little)(INPUT);
        // let (_, objset) = ObjectSet::parse(Endianness::Little)(INPUT).unwrap();
        // assert_eq!(objset.objects.len(), 1);
    }
    #[test]
    fn rin_objectset_read() {
        let _objset = ObjectSet::parse(Endianness::Little)(RININPUT);
        todo!()
        // let (_, objset) = ObjectSet::parse(Endianness::Little)(INPUT).unwrap();
        // assert_eq!(objset.objects.len(), 1);
    }

    #[test]
    fn object_read() {
        let input = &INPUT[OBJECT..];
        let (_, _object) = Object::parse(Endianness::Little)(input).unwrap();
    }
}
