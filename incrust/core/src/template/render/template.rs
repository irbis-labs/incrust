use std::fmt;

use crate::template::render::{RenderContent, RenderExtension};

pub enum RenderTemplate<'a> {
    Content(RenderContent<'a>),
    Extension(RenderExtension<'a>),
}

impl<'a> fmt::Display for RenderTemplate<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            RenderTemplate::Content(content) => content.fmt(f),
            RenderTemplate::Extension(extension) => extension.fmt(f),
        }
    }
}
