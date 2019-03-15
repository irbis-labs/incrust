use nom::{
    Context::*,
    Err::*,
    ErrorKind::*,
    types::CompleteByteSlice as Slice,
    is_alphabetic,
};

use crate::container::pst::{self, ErrorKind::*};

#[inline]
pub fn is_anu(ch: u8) -> bool {
    ch == b'_' || (ch as char).is_ascii_alphanumeric()
}

pub fn identifier(input: Slice) -> nom::IResult<Slice, pst::Identifier, pst::ErrorKind> {
    do_parse!(
        input,
        identifier: recognize!(do_parse!(
            take_while_m_n!(1, 1, is_alphabetic) >>
            take_while!(is_anu) >>
            (()) )) >>
        ( pst::Identifier(&identifier[..]) )
    )
        .map_err(|_| Error(Code(input, Custom(NotRecognized))))
}


#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Ok((Slice(EMPTY), pst::Identifier(&sample[..]))),
            identifier(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            identifier(sample),
        );
    }

    #[test]
    fn test() {
        good("a");
        good("ab");
        good("abc");
        good("a_b");
        good("a0");
        good("a_0");
        good("a0z");
        good("a0z_");
        good("Capitalized");
        good("UPPER");
        good("UPPER_SNAKE");

        not_recognized("");
        not_recognized("_");
        not_recognized("__");
        not_recognized("_a");
        not_recognized("_0");
        not_recognized("0");
        not_recognized("0_");
        not_recognized("0a");
        not_recognized("00");
    }
}
