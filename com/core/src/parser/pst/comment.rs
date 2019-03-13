use nom::{Context::*, Err::*, ErrorKind::*, types::CompleteByteSlice as Slice};

use crate::container::pst::{self, ErrorKind::*};

pub fn comment(input: Slice) -> nom::IResult<Slice, pst::Node, pst::ErrorKind> {
    let (next, tag1) = complete!(input, tag!("{#"))
        .map_err(|_| Error(Code(input, Custom(NotRecognized))))?;

    let (next, comment) = take_until!(next, "#}")
        .map_err(|_| Failure(Code(input, Custom(UnclosedComment))))?;

    let (output, tag2) = tag!(next, "#}")
        .map_err(|_| Failure(Code(input, Custom(UnclosedComment))))?;

    let len = tag1.len() + comment.len() + tag2.len();
    let slice = &input[..len];
    Ok((output, pst::Node::Comment(slice)))
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Ok((Slice(EMPTY), pst::Node::Comment(&sample[..]))),
            comment(sample),
        );
    }

    fn not_recognized(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Error(Code(sample, Custom(NotRecognized)))),
            comment(sample),
        );
    }

    fn unclosed(sample: &str) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Err(Failure(Code(sample, Custom(UnclosedComment)))),
            comment(sample),
        );
    }

    #[test]
    fn test() {
        good("{##}");
        good("{###}");
        good("{# #}");
        good("{#  #}");
        good("{# {# #}");
        good("{# with space #}");
        good("{#without spaces#}");
        good("{# with {{ expression }} #}");
        good("{# with {% statement %} #}");
        good("{# with \new line #}");

        not_recognized("");
        not_recognized("{");
        not_recognized("{ # #}");

        unclosed("{#");
        unclosed("{# unclosed");
        unclosed("{#}");

        not_recognized("plain text");
        not_recognized("{{ expression }}");
        not_recognized("{% statement %}");
    }
}
