use std::ops::{AddAssign, Mul, MulAssign, Neg, RangeInclusive};

use nom::{
    Err, IResult,
    bytes::complete::tag,
    character::complete::{digit1, one_of},
    combinator::{map_opt, opt},
    error::{ErrorKind, ParseError},
    sequence::separated_pair,
};

pub fn ascii_digit_to_value(character: u8) -> Option<u8> {
    Some(match character {
        ch @ (b'0'..=b'9') => ch - b'0',
        ch @ (b'a'..=b'f') => ch - b'a' + 10,
        ch @ (b'A'..=b'F') => ch - b'A' + 10,
        _ => return None,
    })
}

pub fn parse_unsigned_radix<'a, I, U>(input: I, radix: u8) -> Option<U>
where
    I: IntoIterator<Item = &'a u8>,
    U: AddAssign<U> + MulAssign<U> + From<u8>,
{
    let mut number = 0.into();
    for ch in input.into_iter().copied() {
        let value = ascii_digit_to_value(ch)?;
        if value >= radix {
            return None;
        }
        number *= radix.into();
        number += value.into();
    }
    Some(number)
}

pub fn unsigned_parser_radix<'a, U, E>(radix: u8) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], U, E>
where
    E: ParseError<&'a [u8]>,
    U: AddAssign<U> + MulAssign<U> + From<u8>,
{
    map_opt(digit1, move |digits| parse_unsigned_radix(digits, radix))
}

pub fn parse_unsigned<U>(input: &[u8]) -> IResult<&[u8], U>
where
    U: AddAssign<U> + MulAssign<U> + From<u8>,
{
    unsigned_parser_radix(10)(input)
}

pub fn parse_range_unsigned<'a, S, U>(
    mut sep: S,
) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], RangeInclusive<U>>
where
    U: AddAssign<U> + MulAssign<U> + From<u8>,
    S: FnMut(&'a [u8]) -> IResult<&'a [u8], &'a [u8]>,
{
    move |input| {
        let (rest, (from, to)) = separated_pair(parse_unsigned, &mut sep, parse_unsigned)(input)?;
        Ok((rest, from..=to))
    }
}

pub fn signed_parser_radix<'a, I, E>(radix: u8) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], I, E>
where
    E: ParseError<&'a [u8]>,
    I: AddAssign<I> + MulAssign<I> + Mul<I, Output = I> + From<u8> + Neg<Output = I>,
{
    move |input| {
        if input.is_empty() {
            return Err(Err::Error(ParseError::from_error_kind(
                input,
                ErrorKind::Fail,
            )));
        }
        let (_, prefix) = opt(one_of("+-"))(&input[0..1])?;
        let (positive, sub_input) = if let Some(prefix) = prefix {
            if prefix == '+' {
                (true, &input[1..])
            } else {
                (false, &input[1..])
            }
        } else {
            (true, input)
        };
        map_opt(digit1, move |digits| {
            parse_unsigned_radix(digits, radix).map(|u: I| if positive { u } else { -u })
        })(sub_input)
    }
}

pub fn parse_signed<I>(input: &[u8]) -> IResult<&[u8], I>
where
    I: AddAssign<I> + MulAssign<I> + Mul<I, Output = I> + From<u8> + Neg<Output = I>,
{
    signed_parser_radix(10)(input)
}

#[allow(dead_code)]
pub fn parse_range_signed<I>(input: &[u8]) -> IResult<&[u8], RangeInclusive<I>>
where
    I: AddAssign<I> + MulAssign<I> + Mul<I, Output = I> + From<u8> + Neg<Output = I>,
{
    let (rest, (from, to)) = separated_pair(parse_signed, tag(b".."), parse_signed)(input)?;
    Ok((rest, from..=to))
}
