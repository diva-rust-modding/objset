use crate::Vec3;
use nom::number::complete::le_f32;
use nom::number::Endianness;
use nom::IResult;

#[derive(Debug, PartialEq, Clone)]
pub struct AxisAlignedBoundingBox {
    pub min: Vec3,
    pub max: Vec3,
}
#[derive(Debug, PartialEq, Clone)]
pub struct BoundingBox {
    pub center: Vec3,
    pub sides: Vec3,
}
#[derive(Debug, PartialEq, Clone)]
pub struct BoundingSphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Default for BoundingSphere {
    fn default() -> Self {
        Self {
            center: Vec3 {
                x: 0.,
                y: 0.,
                z: 0.,
            },
            radius: 1.,
        }
    }
}

impl AxisAlignedBoundingBox {
    pub fn parse(i0: &[u8], _endian: Endianness) -> IResult<&[u8], Self> {
        let (i, x) = le_f32(i0)?;
        let (i, y) = le_f32(i)?;
        let (i, z) = le_f32(i)?;
        let min = Vec3 { x, y, z };
        let (i, x) = le_f32(i)?;
        let (i, y) = le_f32(i)?;
        let (i, z) = le_f32(i)?;
        let max = Vec3 { x, y, z };
        Ok((i, Self { min, max }))
    }
}
impl BoundingBox {
    pub fn parse(i0: &[u8], _endian: Endianness) -> IResult<&[u8], Self> {
        let (i, x) = le_f32(i0)?;
        let (i, y) = le_f32(i)?;
        let (i, z) = le_f32(i)?;
        let center = Vec3 { x, y, z };
        let (i, width) = le_f32(i)?;
        let (i, height) = le_f32(i)?;
        let (i, depth) = le_f32(i)?;
        let sides = Vec3 {
            x: width,
            y: height,
            z: depth,
        };
        Ok((i, Self { center, sides }))
    }
}
impl BoundingSphere {
    pub fn parse(i0: &[u8], _endian: Endianness) -> IResult<&[u8], BoundingSphere> {
        let (i, x) = le_f32(i0)?;
        let (i, y) = le_f32(i)?;
        let (i, z) = le_f32(i)?;
        let (i, radius) = le_f32(i)?;
        Ok((
            i,
            Self {
                center: Vec3 { x, y, z },
                radius,
            },
        ))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const I: &[u8] = &[
        0u8, 0, 0x80, 0x3F, 0, 0, 0, 0x40, 0, 0, 0x40, 0x40, 0, 0, 0x80, 0x40, 0, 0, 0xA0, 0x40, 0,
        0, 0xC0, 0x40,
    ];
    const SPHERE: BoundingSphere = BoundingSphere {
        center: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        radius: 4.0,
    };
    const BOX: BoundingBox = BoundingBox {
        center: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        sides: Vec3 {
            x: 4.,
            y: 5.,
            z: 6.,
        },
    };
    const AXIS: AxisAlignedBoundingBox = AxisAlignedBoundingBox {
        min: Vec3 {
            x: 1.0,
            y: 2.0,
            z: 3.0,
        },
        max: Vec3 {
            x: 4.,
            y: 5.,
            z: 6.,
        },
    };

    #[test]
    fn read_bounding_sphere() {
        let (_, bounding) = BoundingSphere::parse(I, Endianness::Little).unwrap();
        assert_eq!(bounding, SPHERE)
    }

    #[test]
    fn read_bounding_boxes() {
        let (_, bounding) = BoundingBox::parse(I, Endianness::Little).unwrap();
        let (_, axis) = AxisAlignedBoundingBox::parse(I, Endianness::Little).unwrap();
        assert_eq!(bounding, BOX);
        assert_eq!(axis, AXIS)
    }
}
