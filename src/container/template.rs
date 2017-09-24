use std::convert::AsRef;
use std::borrow::Cow;
use std::collections::HashMap;

use abc::*;
use container::expression::*;
use container::parsed::{ParsedNodes, ParsedNode};
use container::stack::VarContext;


pub type Nodes = Vec<Node>;


#[derive(Debug, Default, PartialEq, Clone)]
pub struct Template {
    pub root: Nodes,
    pub blocks: HashMap<Cow<'static, str>, Nodes>,
    pub extends: Option<FullExpression>,
}

impl AsRef<Template> for Template {
    fn as_ref(&self) -> &Template {
        self
    }
}

impl Template {
    pub fn parse(templ: &str) -> TemplateParseResult<Template> {
        use nom::IResult::*;

        let nodes = ::parser::text(templ.as_bytes());
        // trace!(" == parsed == {:?}", &parsed);
        match nodes {
            Incomplete(_) => unreachable!(),
            Error(err) => Err(TemplateParseError::Syntax(format!("{:?}", err).into())),
            Done(_, nodes) => Template::from_parsed(nodes),
        }
    }

    pub fn from_parsed(parsed: ParsedNodes) -> TemplateParseResult<Template> {
        fn process(templ: &mut Template, parsed: ParsedNodes, need_strip_first: bool, need_strip_last: bool) -> TemplateParseResult<Nodes> {
            let mut nodes: Nodes = Default::default();

            // TODO String
            struct TextCompleter {
                pub text: Vec<String>,
                pub need_strip_left: bool,
            }

            impl TextCompleter {
                pub fn new(need_strip_left: bool) -> Self {
                    TextCompleter {
                        text: Default::default(),
                        need_strip_left: need_strip_left,
                    }
                }

                fn prepare(&mut self, need_strip_right: bool) -> Option<String> {
                    if self.text.is_empty() {
                        return None;
                    }

                    let txt: String = self.text.join("");
                    self.text.clear();

                    let txt: Cow<str> = match (self.need_strip_left, need_strip_right) {
                        (true, true) => txt.trim().into(),
                        (true, false) => txt.trim_left().into(),
                        (false, true) => txt.trim_right().into(),
                        (false, false) => txt.into(),
                    };

                    if txt.is_empty() {
                        None
                    } else {
                        Some(txt.into())
                    }
                }

                pub fn complete(&mut self, nodes: &mut Nodes, need_strip_right: bool) {
                    if let Some(txt) = self.prepare(need_strip_right) {
                        nodes.push(Node::Text(txt));
                    }
                    self.need_strip_left = false;
                }

                pub fn compress(&mut self, need_strip_right: bool) {
                    if !need_strip_right {
                        return;
                    }
                    if let Some(txt) = self.prepare(need_strip_right) {
                        self.text.push(txt);
                    }
                }
            }

            let mut completer = TextCompleter::new(need_strip_first);

            for node in parsed {
                match node {
                    ParsedNode::Comment(_) => {}, // Do nothing
                    ParsedNode::Text(node) => {
                        completer.text.push(node);
                    },
                    ParsedNode::Raw(node) => {
                        completer.compress(node.begin.strip_left);
                        completer.text.push(node.text);
                        completer.need_strip_left = node.end.strip_right;
                    },
                    ParsedNode::Mustache(node) => {
                        completer.complete(&mut nodes, false);
                        completer.need_strip_left = false;
                        nodes.push(node.into());
                    },
                    ParsedNode::For(node) => {
                        completer.complete(&mut nodes, node.begin.strip_left);
                        completer.need_strip_left = node.end.strip_right;
                        nodes.push(ForStatement {
                            expression: node.begin.expression,
                            key_var: node.key_var,
                            value_var: node.value_var,
                            block: process(templ, node.block, node.begin.strip_right, node.end.strip_left)?,
                        }.into());
                    },
                    ParsedNode::If(node) => {
                        completer.complete(&mut nodes, node.if_branches[0].begin.strip_left);
                        completer.need_strip_left = node.end.strip_right;
                        let last_strip_left = match node.else_branch {
                            Some(ref els) => [els.begin.strip_left],
                            None => [node.end.strip_left]
                        };
                        let strip_left: Vec<bool> = node.if_branches.iter().skip(1)
                            .map(|next| next.begin.strip_left)
                            .chain(last_strip_left.into_iter().cloned())
                            .collect();
                        let mut if_branches = Vec::with_capacity(node.if_branches.len());
                        for (branch, strip_left) in node.if_branches.into_iter().zip(strip_left.into_iter()) {
                            if_branches.push(IfBranch {
                                expr: branch.begin.expression,
                                block: process(templ, branch.block, branch.begin.strip_right, strip_left)?,
                            });
                        }
                        let else_branch = match node.else_branch {
                            Some(branch) => Some(process(templ, branch.block, branch.begin.strip_right, node.end.strip_left)?),
                            None => None,
                        };
                        nodes.push(IfStatement {
                            if_branches: if_branches,
                            else_branch: else_branch,
                        }.into());
                    },
                    ParsedNode::Block(node) => {
                        completer.complete(&mut nodes, node.begin.strip_left);
                        completer.need_strip_left = node.end.strip_right;
                        let block_nodes = process(templ, node.block, node.begin.strip_right, node.end.strip_left)?;
                        templ.blocks.insert(node.begin.name.clone().into(), block_nodes);
                        nodes.push(Node::Block(node.begin.name));
                    },
                    ParsedNode::Extends(stmt) => {
                        if templ.extends.is_some() {
                            Err(TemplateParseError::Syntax("Too many `extends` sections".into()))?;
                        }
                        completer.complete(&mut nodes, stmt.strip_left);
                        completer.need_strip_left = stmt.strip_right;
                        templ.extends = Some(stmt.expression);
                    },
                    ParsedNode::Include(stmt) => {
                        completer.complete(&mut nodes, stmt.strip_left);
                        completer.need_strip_left = stmt.strip_right;
                        nodes.push(Node::Include(stmt.expression));
                    },
                };
            }

            completer.complete(&mut nodes, need_strip_last);
            Ok(nodes)
        }

        let mut template = Template {
            root: Default::default(),
            blocks: Default::default(),
            extends: None,
        };

        template.root = process(&mut template, parsed, false, false)?;

        Ok(template)
    }

    pub fn get_parent(&self, context: &VarContext) -> RenderResult<Option<Self>> {
        use ::renderer::evaluator::eval_expr;

        Ok(if let Some(fe) = self.extends.as_ref() {
            Some({
                let name = eval_expr(context, &fe.expr)?
                    .ok_or(LoadError::BadName("Can't evaluate name (None result)".into()))?;
                let name = name.try_as_string()
                    .ok_or(LoadError::BadName("Name is not string".into()))?;
                context.env().get_template(&name)?
            })
        } else {
            None
        })
    }
}


#[derive(Debug, PartialEq, Clone)]
pub enum Node {
    Text(String),
    Mustache(Mustache),
    For(ForStatement),
    If(IfStatement),
    Block(String),
    Include(FullExpression),
}


#[derive(Debug, PartialEq, Clone)]
pub struct IfBranch {
    pub expr: FullExpression,
    pub block: Nodes,
}


#[derive(Debug, PartialEq, Clone)]
pub struct IfStatement {
    pub if_branches: Vec<IfBranch>,
    pub else_branch: Option<Nodes>,
}


#[derive(Debug, PartialEq, Clone)]
pub struct ForStatement {
    pub expression: FullExpression,
    pub block: Nodes,
    pub key_var: Option<String>,
    pub value_var: String,
}

impl From<Mustache> for Node { fn from(v: Mustache) -> Self { Node::Mustache(v) } }

impl From<IfStatement> for Node { fn from(v: IfStatement) -> Self { Node::If(v) } }
impl From<ForStatement> for Node { fn from(v: ForStatement) -> Self { Node::For(v) } }
