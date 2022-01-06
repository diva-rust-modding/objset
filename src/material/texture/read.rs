use nom::number::complete::*;
use nom::number::Endianness;
use nom::IResult;
use std::convert::TryInto;

use super::*;
use crate::util::read::*;

impl Texture {
    pub fn parse_eight(i: &[u8]) -> IResult<&[u8], [Option<Self>; 8]> {
        let (i, t1) = Self::parse(i)?;
        let (i, t2) = Self::parse(i)?;
        let (i, t3) = Self::parse(i)?;
        let (i, t4) = Self::parse(i)?;
        let (i, t5) = Self::parse(i)?;
        let (i, t6) = Self::parse(i)?;
        let (i, t7) = Self::parse(i)?;
        let (i, t8) = Self::parse(i)?;
        let arr = [t1, t2, t3, t4, t5, t6, t7, t8];
        Ok((i, arr))
    }
    pub fn parse(i: &[u8]) -> IResult<&[u8], Option<Self>> {
        let (i, sampler_flags) = SamplerFlags::parse(i)?;
        let (i, id) = u32(Endianness::Little)(i)?;
        let (i, flags) = TextureFlags::parse(i)?;
        let ex_shader: [u8; 8] = i[..8].try_into().unwrap();
        let i = &i[8..];
        let (i, weight) = le_f32(i)?;
        let (i, coordinate_matrix) = mat44(Endianness::Little)(i)?;
        let i = &i[4 * 8..];
        if id == 0xFFFFFFFF {
            return Ok((i, None));
        }
        Ok((
            i,
            Some(Self {
                sampler_flags,
                id,
                flags,
                ex_shader,
                weight,
                coordinate_matrix,
            }),
        ))
    }
}

impl SamplerFlags {
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let (i, sampler_flags) = be_u32(i)?;
        // println!("sampler flags {:#X}", sampler_flags);
        Ok((i, Self::default()))
    }
}

impl TextureFlags {
    //TODO: read this properly
    fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        let b = i[0];
        let uv_index = b >> 4;
        println!("uv index: {}", uv_index);
        let uv_index = UvIndex::from_byte(uv_index).unwrap_or(UvIndex::None);
        let map = TextureMap::from_byte(b & 0x0F).unwrap();
        let b = i[1];
        let uv_translation = b;
        let flags = Self {
            map,
            uv_index,
            uv_translation: UvTranslationType::None,
        };
        Ok((&i[4..], flags))
    }
}

impl UvIndex {
    fn from_byte(id: u8) -> Option<Self> {
        Some(match id {
            15 => Self::None,
            0 => Self::Index0,
            1 => Self::Index1,
            2 => Self::Index2,
            3 => Self::Index3,
            4 => Self::Index4,
            5 => Self::Index5,
            6 => Self::Index6,
            7 => Self::Index7,
            _ => return None,
        })
    }
}

impl TextureMap {
    // fn parse(i: &[u8]) -> IResult<&[u8], Self> {
    //     use nom::bits::complete::*;
    //     let b = i[0];
    //     match
    //     todo!()
    // }
    pub fn from_byte(b: u8) -> Option<Self> {
        Some(match b {
            0 => Self::None,
            1 => Self::Color,
            2 => Self::Normal,
            3 => Self::Specular,
            4 => Self::Height,
            5 => Self::Reflection,
            6 => Self::Translucency,
            7 => Self::Transparency,
            8 => Self::Sphere,
            9 => Self::Cube,
            e => {
                println!("UNKNOWN MAP: {}", e);
                return None;
            }
        })
    }
}

fn vec4(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], Vec4> {
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

fn mat44(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], Matrix4> {
    use nom::sequence::tuple;
    move |i: &[u8]| {
        let (i, (x, y, z, w)) = tuple((vec4(endian), vec4(endian), vec4(endian), vec4(endian)))(i)?;
        Ok((i, Matrix4 { x, y, z, w }))
    }
}
