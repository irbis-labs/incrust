use incrust_format::prelude::*;

pub struct HtmlAttribute<N, V> {
    pub name: N,
    pub value: V,
}

impl<N, V> HtmlAttribute<N, V> {
    pub fn new(name: N, value: V) -> Self {
        HtmlAttribute { name, value }
    }
}

impl<N, V> fmt::Display for HtmlAttribute<N, V>
where
    N: fmt::Display,
    V: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        use fmt::Write;

        f.write_char(' ')?;
        self.name.fmt(f)?;
        f.write_str(r#"=""#)?;
        self.value.fmt(f)?;
        f.write_char('"')?;
        Ok(())
    }
}
