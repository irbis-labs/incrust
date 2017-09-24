use abc::{Filter, FilterResult};
use {VarContext, ex, Arg};


#[derive(Debug)]
pub struct NewlineToSpace;

impl Filter for NewlineToSpace {
    #[allow(unused_variables)]
    fn filter<'s: 'a, 'a>(&'s self, context: &'a VarContext, value: Option<Arg<'a>>) -> FilterResult<Arg<'a>> {
        match value {
            None => Ok(None),
            Some(value) => {
                let value = match value.try_as_string() {
                    None => String::new(),
                    Some(string) => string.replace(is_nl, " "),
                };
                Ok(Some(ex(value)))
            },
        }
    }
}

fn is_nl(c: char) -> bool {
    match c {
        '\n' | '\r' => true,
        _           => false,
    }
}


#[cfg(test)]
mod tests {
    #![cfg_attr(feature = "cargo-clippy", allow(used_underscore_binding))]

    use {Incrust, ex, Args};

    #[test]
    fn it_works() {
        let incrust = Incrust::default();
        let test = |expected, sample| {
            let args: Args = hashmap!{ "sample".into() => ex(sample) };
            assert_eq!(expected, incrust.render_text("{{ sample | newline_to_space }}", &args).unwrap());
        };

        test("", "");
        test(" ", " ");
        test(" ", "\n");
        test(" ", "\r");
        test("  ", "\r\n");
        test("  ", " \n");
        test("  ", "\n ");
        test("   ", " \n ");
        test("1 2", "1\n2");
    }
}
