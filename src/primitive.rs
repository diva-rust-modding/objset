#[cfg(feature="pyo3")]


use nom::combinator::map;
use nom::multi::count;
use nom::number::Endianness;
use nom::IResult;
use nom_ext::*;
use num_derive::*;

#[derive(Debug, PartialEq, PartialOrd, FromPrimitive, ToPrimitive, Copy, Clone)]
pub enum PrimitiveType {
    Point,         //0,
    Line,         //1,
    LineStrip,     //2,
    LineLoop,      //3,
    Triangle,      //4,
    TriangleStrip, //5,
    TriangleFan,   //6,
    Quad,          //7,
    QuadStrip,     //8,
    Polygon,       //9,
}

impl PrimitiveType {
    pub(crate) fn parse(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], Option<Self>> {
        use num_traits::FromPrimitive;
        move |i: &[u8]| {
            let (i, val) = u32(endian)(i)?;
            println!("primitive id: {}", val);
            let val = Self::from_u32(val);
            Ok((i, val))
        }
    }
}

use mint::{Vector2, Vector3, Vector4};

#[derive(Debug)]
pub enum Primitives {
    Points,
    Lines(Vec<Vector2<usize>>),
    LineStrips(Vec<Vector2<usize>>),
    LineLoops,
    Triangles(Vec<Vector3<usize>>),
    TriangleStrips(Vec<Vector3<usize>>),
    TriangleFans(Vec<Vector3<usize>>),
    Quads(Vec<Vector4<usize>>),
    QuadStrips(Vec<Vector4<usize>>),
    Polygons,
}
#[derive(Debug, FromPrimitive, Copy, Clone)]
pub(crate) enum IndexType {
    U8,
    U16,
    U32,
}

impl IndexType {
    pub(crate) fn parse(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], Option<Self>> {
        use num_traits::FromPrimitive;
        move |i: &[u8]| {
            let (i, val) = u32(endian)(i)?;
            let val = Self::from_u32(val);
            Ok((i, val))
        }
    }
}

impl Default for Primitives {
    fn default() -> Self {
        Self::Triangles(vec![])
    }
}

fn parse_vector3(
    idx_ty: IndexType,
    endian: Endianness,
) -> impl Fn(&[u8]) -> IResult<&[u8], Vector3<usize>> {
    use nom::number::complete::le_u8;
    use nom::sequence::tuple;
    use IndexType::*;
    move |i: &[u8]| {
        let (i, (x, y, z)) = match idx_ty {
            U8 => tuple((usize(le_u8), usize(le_u8), usize(le_u8)))(i),
            U16 => tuple((usize(u16(endian)), usize(u16(endian)), usize(u16(endian))))(i),
            U32 => tuple((usize(u32(endian)), usize(u32(endian)), usize(u32(endian))))(i),
        }?;
        Ok((i, Vector3 { x, y, z }))
    }
}

impl From<Primitives> for PrimitiveType {
    fn from(primitives: Primitives) -> Self {
        use Primitives::*;
        use PrimitiveType::*;
        match primitives {
            Points => Point,
            Lines(_)=> Line,
            LineStrips(_) => LineStrip,
            LineLoops => LineLoop,
            Triangles(_) => Triangle,
            TriangleStrips(_) => TriangleStrip,
            TriangleFans(_) => TriangleFan,
            Quads(_) => Quad,
            QuadStrips(_) => QuadStrip,
            Polygons => Polygon,
        }
    }
}

impl Primitives {
    pub(crate) fn parse(
        idx_ty: IndexType,
        primitive: PrimitiveType,
        cnt: usize,
        endian: Endianness,
    ) -> impl Fn(&[u8]) -> IResult<&[u8], Self> {
        use PrimitiveType::*;
        move |i: &[u8]| {
            let vec3 = count(parse_vector3(idx_ty, endian), cnt / 3);
            match primitive {
                Triangle => map(vec3, Primitives::Triangles)(i),
                TriangleStrip => map(vec3, Primitives::TriangleStrips)(i),
                TriangleFan => map(vec3, Primitives::TriangleFans)(i),
                e => todo!("Found {:?}", e),
            }
        }
    }
}
