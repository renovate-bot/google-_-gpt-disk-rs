// Copyright 2022 Google LLC
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// https://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use crate::{Guid, GuidFromStrError};

/// Parse a hexadecimal ASCII character as a `u8`.
const fn parse_byte_from_ascii_char(c: u8) -> Option<u8> {
    match c {
        b'0' => Some(0x0),
        b'1' => Some(0x1),
        b'2' => Some(0x2),
        b'3' => Some(0x3),
        b'4' => Some(0x4),
        b'5' => Some(0x5),
        b'6' => Some(0x6),
        b'7' => Some(0x7),
        b'8' => Some(0x8),
        b'9' => Some(0x9),
        b'a' | b'A' => Some(0xa),
        b'b' | b'B' => Some(0xb),
        b'c' | b'C' => Some(0xc),
        b'd' | b'D' => Some(0xd),
        b'e' | b'E' => Some(0xe),
        b'f' | b'F' => Some(0xf),
        _ => None,
    }
}

/// Macro replacement for the `?` operator, which cannot be used in
/// const functions.
macro_rules! mtry {
    ($expr:expr $(,)?) => {
        match $expr {
            Ok(val) => val,
            Err(err) => {
                return Err(err);
            }
        }
    };
}

/// Parse a pair of hexadecimal ASCII characters as a `u8`. For example,
/// `(b'1', b'a')` is parsed as `0x1a`.
#[allow(clippy::question_mark)] // `?` is not allowed in const functions.
const fn parse_byte_from_ascii_char_pair(a: u8, b: u8) -> Option<u8> {
    let a = if let Some(a) = parse_byte_from_ascii_char(a) {
        a
    } else {
        return None;
    };

    let b = if let Some(b) = parse_byte_from_ascii_char(b) {
        b
    } else {
        return None;
    };

    Some(a << 4 | b)
}

/// Parse a pair of hexadecimal ASCII characters at position `start` as
/// a `u8`.
const fn parse_byte_from_ascii_str_at(
    s: &[u8],
    start: u8,
) -> Result<u8, GuidFromStrError> {
    // This `as` conversion is needed because this is a const
    // function. It is always valid since `usize` is always bigger than
    // a u8.
    #![allow(clippy::as_conversions)]
    let start_usize = start as usize;

    if let Some(byte) =
        parse_byte_from_ascii_char_pair(s[start_usize], s[start_usize + 1])
    {
        Ok(byte)
    } else {
        Err(GuidFromStrError::Hex(start))
    }
}

pub(crate) const fn try_parse_guid(s: &str) -> Result<Guid, GuidFromStrError> {
    // Treat input as ASCII.
    let s = s.as_bytes();

    if s.len() != 36 {
        return Err(GuidFromStrError::Length);
    }

    let sep = b'-';
    if s[8] != sep {
        return Err(GuidFromStrError::Separator(8));
    }
    if s[13] != sep {
        return Err(GuidFromStrError::Separator(13));
    }
    if s[18] != sep {
        return Err(GuidFromStrError::Separator(18));
    }
    if s[23] != sep {
        return Err(GuidFromStrError::Separator(23));
    }

    Ok(Guid {
        time_low: [
            mtry!(parse_byte_from_ascii_str_at(s, 6)),
            mtry!(parse_byte_from_ascii_str_at(s, 4)),
            mtry!(parse_byte_from_ascii_str_at(s, 2)),
            mtry!(parse_byte_from_ascii_str_at(s, 0)),
        ],
        time_mid: [
            mtry!(parse_byte_from_ascii_str_at(s, 11)),
            mtry!(parse_byte_from_ascii_str_at(s, 9)),
        ],
        time_high_and_version: [
            mtry!(parse_byte_from_ascii_str_at(s, 16)),
            mtry!(parse_byte_from_ascii_str_at(s, 14)),
        ],
        clock_seq_high_and_reserved: mtry!(parse_byte_from_ascii_str_at(s, 19)),
        clock_seq_low: mtry!(parse_byte_from_ascii_str_at(s, 21)),
        node: [
            mtry!(parse_byte_from_ascii_str_at(s, 24)),
            mtry!(parse_byte_from_ascii_str_at(s, 26)),
            mtry!(parse_byte_from_ascii_str_at(s, 28)),
            mtry!(parse_byte_from_ascii_str_at(s, 30)),
            mtry!(parse_byte_from_ascii_str_at(s, 32)),
            mtry!(parse_byte_from_ascii_str_at(s, 34)),
        ],
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        assert_eq!(parse_byte_from_ascii_char_pair(b'1', b'a'), Some(0x1a));
        assert_eq!(parse_byte_from_ascii_char_pair(b'8', b'f'), Some(0x8f));

        assert_eq!(parse_byte_from_ascii_char_pair(b'g', b'a'), None);
        assert_eq!(parse_byte_from_ascii_char_pair(b'a', b'g'), None);
    }
}
