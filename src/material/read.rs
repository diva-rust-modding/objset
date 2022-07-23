use nom::number::complete::f32;
use nom::number::Endianness;
use nom::IResult;

use super::*;
use crate::util::read::*;

impl Material {
    pub fn parse(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], Self> {
        move |i: &[u8]| {
            //currently unimplemented
            let i = &i[8..];
            let (i, shader) = ShaderType::parse(i)?;
            let i = &i[4..];
            let (i, textures) = Texture::parse_eight(i)?;
            let i = &i[92..];
            // let i = &i[0x4B0 - 64 - 4 * 16..];
            let (i, name) = string64(i)?;
            let (i, bump_depth) = f32(endian)(i)?;
            //skip reserved
            let i = &i[15 * 4..];
            Ok((
                i,
                Self {
                    shader,
                    name: name.into(),
                    bump_depth,
                    textures,
                },
            ))
        }
    }
}

impl ShaderType {
    pub fn parse(i: &[u8]) -> IResult<&[u8], Self> {
        use nom::bytes::complete::*;
        let (i, magic) = take(8usize)(i)?;
        let magic = match magic {
            b"BLINN\0\0\0" => Self::Blinn,
            b"CHARA\0\0\0" => Self::Chara,
            b"CLOTH\0\0\0" => Self::Cloth,
            b"EYEBALL\0" => Self::Eyeball,
            b"FLOOR\0\0\0" => Self::Floor,
            b"HAIR\0\0\0\0" => Self::Hair,
            b"ITEM\0\0\0\0" => Self::Item,
            b"PUDDLE\0\0" => Self::Puddle,
            b"SKIN\0\0\0\0" => Self::Skin,
            b"SKY\0\0\0\0\0" => Self::Sky,
            b"STAGE\0\0\0" => Self::Stage,
            b"TIGHTS\0\0" => Self::Tights,
            b"WATER01\0" => Self::Water01,
            e => unreachable!(
                "Encountered unknown shader type `{}`",
                String::from_utf8_lossy(e)
            ),
        };
        Ok((i, magic))
    }
}

// impl Default for Material {
//     fn default() -> Self {
//         Self {
//             shader: (),
//             name: String::new(),
//             bump_depth: 1.0,
//             texture_flags: (),
//             shader_flags: (),
//             textures: (),
//             blend_flags: (),
//             colors: (),
//         }
//     }
// }

impl Default for Color {
    fn default() -> Self {
        let o = Rgb {
            x: 1.,
            y: 1.,
            z: 1.,
        };
        let o4 = Rgba {
            x: 1.,
            y: 1.,
            z: 1.,
            w: 1.,
        };
        let z = Rgb {
            x: 0.,
            y: 0.,
            z: 0.,
        };
        Self {
            diffuse: o,
            transparency: 1.,
            ambient: o4,
            specular: o,
            reflectivity: 1.,
            emission: z,
            shininess: 0.0,
            intensity: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    const I: &[u8] = include_bytes!("../../assets/suzanne_obj.bin");
    const MAT_OFF: usize = 0x27ED8;

    use super::*;

    #[test]
    fn read_material() {
        let input = &I[MAT_OFF..];
        let (_, mat) = Material::parse(Endianness::Little)(input).unwrap();
        let tex_len = mat.textures.iter().filter(|x| x.is_some()).count();
        assert_eq!(mat.name, "material");
        assert_eq!(mat.shader, ShaderType::Blinn);
        assert_eq!(tex_len, 1);
        assert_eq!(mat.bump_depth, 1.);
    }
}
