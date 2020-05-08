use nom::number::Endianness;
use nom::IResult;
use nom_ext::r#trait::*;
use nom_ext::*;

use super::*;
use crate::read::*;

impl Material {
    pub fn parse(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], Self> {
        move |i: &[u8]| {
            //currently unimplemented
            let i = &i[0x4B0 - 64 - 4 * 16..];
            let (i, name) = string64(i)?;
            let (i, bump_depth) = f32(endian)(i)?;
            //skip reserved
            let i = &i[15 * 4..];
            Ok((
                i,
                Self {
                    name: name.into(),
                    bump_depth,
                    ..Default::default()
                },
            ))
        }
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
    const I: &[u8] = include_bytes!("../../assets/rinitm001_obj.bin");
    const MAT_OFF: usize = 0x129B34;

    use super::*;

    #[test]
    fn read_material() {
        let input = &I[MAT_OFF..];
        println!("{:X?}", &input[0x4B0 - 64 - 4 * 16..0x4B0]);
        let (_, mat) = Material::parse(Endianness::Little)(input).unwrap();
        assert_eq!(mat.name, "green_light_CH_ITEM_SD008Z");
        assert_eq!(mat.bump_depth, 1.);
    }
}
