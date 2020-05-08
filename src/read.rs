use nom::number::Endianness;
use nom::IResult;

use std::borrow::Cow;

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
