use std::borrow::Cow;


pub type TemplateParseResult<T> = Result<T, TemplateParseError>;

#[derive(Debug)]
pub enum TemplateParseError {
    Syntax(Cow<'static, str>),
}

//quick_error! {
//    #[derive(Debug)]
//    pub enum ParseError {
//        Syntax(err: String) {
//            from()
//        },
//    }
//}
