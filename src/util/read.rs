use nom::multi::count;
use nom::number::complete::*;
use nom::number::Endianness;
use nom::IResult;
use nom::Parser;
use nom::ToUsize;
use nom::{combinator::map, error::ParseError};

use std::borrow::Cow;
use std::convert::TryInto;

pub fn string64(i: &[u8]) -> IResult<&[u8], Cow<'_, str>> {
    use nom::bytes::complete::take_till;
    use nom::combinator::map;
    let (i, name) = map(take_till(|c| c == 0), String::from_utf8_lossy)(i)?;
    let i = &i[64 - name.len()..];
    Ok((i, name))
}

pub fn usize<I, F, O, E>(mut f: F) -> impl FnMut(I) -> IResult<I, usize, E>
where
    F: Parser<I, O, E>,
    O: ToUsize,
    E: ParseError<I>,
{
    move |i| {
        let (i, v) = f.parse(i)?;
        Ok((i, v.to_usize()))
    }
}

pub fn u32_usize<'a, E: ParseError<&'a [u8]>>(
    endian: Endianness,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], usize, E> {
    usize(u32(endian))
}

use nom::{InputIter, InputTake};
pub fn at_offset<I, O, E, F>(offset: usize, mut f: F) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputIter + InputTake + Clone,
    F: Parser<I, O, E>,
    E: ParseError<I>,
{
    use nom::bytes::complete::*;
    move |i: I| {
        let (i0, _) = take(offset)(i.clone())?;
        let (_, v) = f.parse(i0)?;
        Ok((i, v))
    }
}

// read_at_offset2(256)(string)(i0)
// read_at_offset(i0)(string)(256)
// read_at_offset(i0)(256)(string)
// read_at_offset(le_u32(i)?.1, string)(i0)
// read_offset_at(le_u32, string)(i0)
// read_offset_at(u32(endian), string)(i0)

pub fn offset_read_then<I, O, F, F1, U, E>(
    i0: I,
    mut f1: F1,
    mut f: F,
) -> impl FnMut(I) -> IResult<I, O, E>
where
    I: InputIter + InputTake + Clone,
    F: Parser<I, O, E>,
    F1: Parser<I, U, E>,
    U: ToUsize,
    E: ParseError<I>,
{
    move |i: I| {
        let (i1, offset) = f1.parse(i)?;
        let offset = offset.to_usize();
        let f0 = |x| f.parse(x);
        let (_, v) = at_offset(offset, f0)(i0.clone())?;
        Ok((i1, v))
    }
}

pub fn offset_then<'a, O, E, F>(
    i0: &'a [u8],
    f: F,
    endian: Endianness,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], O, E>
where
    F: Parser<&'a [u8], O, E>,
    E: ParseError<&'a [u8]>,
{
    offset_read_then(i0, u32_usize(endian), f)
}

pub fn offset_read_table<'a, O, E, F, F1, U>(
    i0: &'a [u8],
    mut f1: F1,
    mut f: F,
    cnt: usize,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<O>, E>
where
    F: Parser<&'a [u8], O, E>,
    F1: Parser<&'a [u8], U, E>,
    U: ToUsize,
    E: ParseError<&'a [u8]>,
{
    move |i: &[u8]| {
        let f1 = |x| f1.parse(x);
        let (i1, offsets) = count(usize(f1), cnt)(i)?;
        let mut res = vec![];
        for offset in offsets {
            let (_, val) = at_offset(offset, |x| f.parse(x)).parse(i0)?;
            res.push(val);
        }
        Ok((i1, res))
    }
}

pub fn offset_table<'a, O, E, F>(
    i0: &'a [u8],
    f: F,
    cnt: usize,
    endian: Endianness,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], Vec<O>, E>
where
    F: Parser<&'a [u8], O, E>,
    E: ParseError<&'a [u8]>,
{
    offset_read_table(i0, u32(endian), f, cnt)
}

pub fn count_then_offset<I, O, E, F, F1, U>(
    i0: I,
    mut f1: F1,
    mut f: F,
) -> impl FnMut(I) -> IResult<I, Vec<O>, E>
where
    I: InputIter + InputTake + Clone + PartialEq,
    F: Parser<I, O, E>,
    F1: Parser<I, U, E>,
    U: ToUsize,
    E: ParseError<I>,
{
    move |i: I| {
        let (i1, cnt) = f1.parse(i)?;
        let cnt = cnt.to_usize();
        let f1 = |x| f1.parse(x);
        let f = |x| f.parse(x);
        let (i1, v) = offset_read_then(i0.clone(), f1, count(f, cnt)).parse(i1)?;
        Ok((i1, v))
    }
}
