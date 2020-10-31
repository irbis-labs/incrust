use crate::template::ast::TemplateBlock;

pub trait BuilderParent<Parent> {
    fn complete_block(block: TemplateBlock) -> Parent;
}