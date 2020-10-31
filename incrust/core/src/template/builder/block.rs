// use crate::template::ast::TemplateBlock;
//
// pub struct BlockBuilder<Fun> {
//     complete: Fun,
//     name: String,
//     content: Vec<TemplateBlock>,
// }
//
// impl<Fun, Parent> BlockBuilder<Fun>
// where
//     Fun: FnOnce(TemplateBlock) -> Parent,
// {
//     pub fn new(complete: Fun, name: impl Into<String>) -> Self {
//         BlockBuilder {
//             complete,
//             name: name.into(),
//             content: Vec::new(),
//         }
//     }
//
//     pub fn push(mut self, block: TemplateBlock) -> Self {
//         self.content.push(block);
//         self
//     }
//
//     pub fn plain_text(mut self, text: impl Into<String>) -> Self {
//         let content = text.into();
//         self.push(TemplateBlock::PlainText { content })
//     }
//
//     pub fn block(mut self, name: impl Into<String>) -> BlockBuilder<impl FnOnce(TemplateBlock) -> Self> {
//         BlockBuilder::new(move |block| self.push(block), name)
//     }
//
//     pub fn finish_block(self) -> Parent {
//         let block = TemplateBlock::Block {
//             name: self.name,
//             content: self.content,
//         };
//         (self.complete)(block)
//     }
// }
// // use crate::template::ast::TemplateBlock;
// //
// // pub struct BlockBuilder<Parent> {
// //     parent: Parent,
// //     name: String,
// //     content: Vec<TemplateBlock>,
// // }
// //
// // impl<Parent: BuilderParent> BlockBuilder<Parent> {
// //     pub fn new(parent: Parent, name: impl Into<String>) -> Self {
// //         let name = name.into();
// //         BlockBuilder {
// //             parent,
// //             name,
// //             content: Vec::new(),
// //         }
// //     }
// //
// //     pub fn push(&mut self, block: TemplateBlock) {
// //         self.content.push(block);
// //     }
// //
// //     pub fn plain_text(mut self, text: impl Into<String>) -> Self {
// //         let content = text.into();
// //         self.push(TemplateBlock::PlainText { content });
// //         self
// //     }
// //
// //     pub fn block(mut self, name: impl Into<String>) -> BlockBuilder<impl FnOnce(Self) -> Self> {
// //         BlockBuilder::new(self, name)
// //     }
// //
// //     pub fn finish_block(self) -> Parent {
// //         let block = TemplateBlock::Block {
// //             name: self.name,
// //             content: self.content
// //         };
// //         self.parent.complete_block(block)
// //     }
// // }
