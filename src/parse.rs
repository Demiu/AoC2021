use std::ops::{AddAssign, MulAssign};

use nom::{character::complete::digit1, combinator::map_opt, error::ParseError, IResult};

fn ascii_digit_to_value(character: u8) -> Option<u8> {
    Some(match character {
        ch @ (b'0'..=b'9') => (ch - b'0'),
        ch @ (b'a'..=b'f') => (ch - b'a' + 10),
        ch @ (b'A'..=b'F') => (ch - b'A' + 10),
        _ => return None,
    })
}

fn parse_unsigned_radix<U>(input: &[u8], radix: u8) -> Option<U>
where
    u8: Into<U>,
    U: AddAssign<U> + MulAssign<U>,
{
    let mut index = 0;
    let mut number = 0.into();
    while let Some(value) = input.get(index).copied().and_then(ascii_digit_to_value) {
        if value >= radix {
            return None;
        }
        number *= radix.into();
        number += value.into();
        index += 1;
    }
    if index == 0 {
        None
    } else {
        Some(number)
    }
}

pub fn unsigned_parser_radix<'a, U, E>(radix: u8) -> impl FnMut(&'a [u8]) -> IResult<&'a [u8], U, E>
where
    E: ParseError<&'a [u8]>,
    U: AddAssign<U> + MulAssign<U>,
    u8: Into<U>,
{
    map_opt(digit1, move |digits| parse_unsigned_radix(digits, radix))
}

pub fn parse_unsigned<'a, U>(input: &'a [u8]) -> IResult<&'a [u8], U>
where
    U: AddAssign<U> + MulAssign<U>,
    u8: Into<U>, 
{
    unsigned_parser_radix(10)(input)
}
