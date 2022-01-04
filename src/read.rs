use nom::combinator::map;
use nom::error::ParseError;
use nom::IResult;
use nom::{multi::count, number::Endianness};

use std::borrow::Cow;
use std::convert::TryInto;

pub fn string64(i: &[u8]) -> IResult<&[u8], Cow<'_, str>> {
    use nom::bytes::complete::take_till;
    use nom::combinator::map;
    let (i, name) = map(take_till(|c| c == 0), String::from_utf8_lossy)(i)?;
    let i = &i[64 - name.len()..];
    Ok((i, name))
}

pub fn f32(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], f32> {
    use nom::number::complete::{be_f32, le_f32};
    move |i: &[u8]| match endian {
        Endianness::Little => le_f32(i),
        Endianness::Big => be_f32(i),
    }
}

pub fn u16(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], u16> {
    use nom::number::complete::{be_u16, le_u16};
    move |i: &[u8]| match endian {
        Endianness::Little => le_u16(i),
        Endianness::Big => be_u16(i),
    }
}

pub fn u32(endian: Endianness) -> impl Fn(&[u8]) -> IResult<&[u8], u32> {
    use nom::number::complete::{be_u32, le_u32};
    move |i: &[u8]| match endian {
        Endianness::Little => le_u32(i),
        Endianness::Big => be_u32(i),
    }
}

pub fn usize<'a, F, O, E>(f: F) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], usize, E>
where
    F: Fn(&'a [u8]) -> IResult<&'a [u8], O, E>,
    O: TryInto<usize>,
    E: ParseError<&'a [u8]>,
{
    map(f, |v| v.try_into().ok().unwrap())
}

pub fn u32_usize<'a>(endian: Endianness) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], usize> {
    usize(u32(endian))
}

use nom::{InputIter, InputTake};
pub fn at_offset<I, O, F>(offset: usize, f: F) -> impl Fn(I) -> IResult<I, O>
where
    I: InputIter + InputTake + Clone,
    F: Fn(I) -> IResult<I, O>,
{
    use nom::bytes::complete::*;
    move |i: I| {
        let (i0, _) = take(offset)(i.clone())?;
        let (_, v) = f(i0)?;
        Ok((i, v))
    }
}

pub fn offset_read_then<'a, O, F, F1, U>(
    i0: &'a [u8],
    f1: F1,
    f: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], O>
where
    F: Fn(&'a [u8]) -> IResult<&'a [u8], O>,
    F1: Fn(&'a [u8]) -> IResult<&'a [u8], U>,
    U: TryInto<usize>,
{
    move |i: &'a [u8]| {
        let (i1, offset) = f1(i)?;
        let offset = offset.try_into().ok().unwrap();
        let f0 = |x| f(x);
        let (_, v) = at_offset(offset, f0)(i0)?;
        Ok((i1, v))
    }
}

pub fn offset_then<'a, O, F>(
    i0: &'a [u8],
    f: F,
    endian: Endianness,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], O>
where
    F: Fn(&'a [u8]) -> IResult<&'a [u8], O>,
{
    offset_read_then(i0, u32_usize(endian), f)
}

pub fn offset_read_table<'a, O, F, F1, U>(
    i0: &'a [u8],
    f1: F1,
    f: F,
    cnt: usize,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<O>>
where
    F: Fn(&'a [u8]) -> IResult<&'a [u8], O>,
    F1: Fn(&'a [u8]) -> IResult<&'a [u8], U>,
    U: TryInto<usize>,
{
    move |i: &[u8]| {
        // let (i1, offset) = f1(i)?;
        let f1 = |x| f1(x);
        let (i1, offsets) = count(usize(f1), cnt)(i)?;
        let f0 = |x| f(x);
        let mut res = vec![];
        for offset in offsets {
            let (_, val) = at_offset(offset, f0)(i0)?;
            res.push(val);
        }
        Ok((i1, res))
    }
}

pub fn offset_table<'a, O, F>(
    i0: &'a [u8],
    f: F,
    cnt: usize,
    endian: Endianness,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<O>>
where
    F: Fn(&'a [u8]) -> IResult<&'a [u8], O>,
{
    offset_read_table(i0, u32(endian), f, cnt)
}

pub fn count_then_offset<'a, O, F, F1, U>(
    i0: &'a [u8],
    f1: F1,
    f: F,
) -> impl Fn(&'a [u8]) -> IResult<&'a [u8], Vec<O>>
where
    F: Fn(&'a [u8]) -> IResult<&'a [u8], O>,
    F1: Fn(&'a [u8]) -> IResult<&'a [u8], U>,
    U: TryInto<usize>,
{
    move |i: &[u8]| {
        let (i1, cnt) = f1(i)?;
        let cnt = cnt.try_into().ok().unwrap();
        let f1 = |x| f1(x);
        let f = |x| f(x);
        let (i1, v) = offset_read_then(i0, f1, count(f, cnt))(i1)?;
        Ok((i1, v))
    }
}
