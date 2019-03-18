use nom::{Context::*, Err::*, ErrorKind::*, types::CompleteByteSlice as Slice};

use crate::container::pst::{*, ErrorKind::*};
use super::*;

pub fn template<'i>(input: Slice<'i>) -> nom::IResult<Slice<'i>, Template<'i>, ErrorKind> {
    fold_many0!(input,
        alt!(
            plaintext => { |v| TemplatePart::PlainText(v) } |
            comment => { |v| TemplatePart::Comment(v) } |
            expression_tag => { |v| TemplatePart::ExpressionTag(v) } |
            statement_tag => { |v| TemplatePart::StatementTag(v) }
        ),
        Template::default(),
        |mut acc: Template<'i>, item| {
            acc.0.push(item);
            acc
        }
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    const EMPTY: &[u8] = &[];

    fn good(sample: &str, templ: Template) {
        let sample = Slice(sample.as_bytes());
        assert_eq!(
            Ok((Slice(EMPTY), templ)),
            template(sample),
        );
    }

    #[test]
    fn simple() {
        good("", Template::default());

        good(" ", Template(vec![PlainText(b" ").into()]));

        good(r#"{# comment #}"#, Template(vec![Comment(br#"{# comment #}"#).into()]));

        good(r#"{% statement %}"#, Template(vec![
            StatementTag::new(StatementExpression(Identifier(b"statement"), None), false, false).into(),
        ]));

        good(r#"{{ expression }}"#, Template(vec![
            ExpressionTag::new(Operations(Identifier(b"expression").into(), vec![]), false, false).into(),
        ]));
    }

    #[test]
    fn complex() {
        let sample = "\
{% extend \"main\" %}
\
        ";
        good(sample, Template(vec![
            StatementTag::new(StatementExpression(
                Identifier(b"extend"),
                Some(Operations(StringLiteral(b"main").into(), vec![]))
            ), false, false).into(),

            PlainText(b"\n").into(),
        ]));
    }
}
