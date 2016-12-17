pub type TemplateParseResult<T> = Result<T, TemplateParseError>;

#[derive(Debug)]
pub enum TemplateParseError {
    Syntax(String),
}

//quick_error! {
//    #[derive(Debug)]
//    pub enum ParseError {
//        Syntax(err: String) {
//            from()
//        },
//    }
//}
