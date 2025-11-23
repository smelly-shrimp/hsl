use crate::{
    src::SrcMan,
    tok::{Span, Tok},
};

pub struct Builder<'a> {
    sm: &'a SrcMan,
}

impl<'a> Builder<'a> {
    pub fn new(sm: &'a SrcMan) -> Self {
        Self { sm }
    }

    pub fn build(&self, root: &Tok) -> String {
        self.build_tok(root)
    }

    pub fn build_tok(&self, tok: &Tok) -> String {
        match tok {
            Tok::Root { children } => {
                self.fmt_children(children)
            }
            Tok::Doctype { cont } => {
                format!("<!doctype {}>", self.sm.slice(&cont))
            }
            Tok::Text { parts } => {
                parts.iter().map(|p| self.sm.slice(p)).collect()
            }
            Tok::Tag {
                name,
                attrs,
                children,
            } => {
                let name = self.sm.slice(&name);
                let children = self.fmt_children(children);
                let attrs = self.fmt_attrs(attrs);

                format!("<{}{}>{}</{}>", name, attrs, children, name)
            }
            Tok::VoidTag { name, attrs } => {
                let name = self.sm.slice(&name);
                let attrs = self.fmt_attrs(attrs);

                format!("<{}{}>", name, attrs)
            }
        }
    }

    fn fmt_children(&self, children: &Vec<Tok>) -> String {
        children.iter().map(|c| self.build_tok(c)).collect()
    }

    fn fmt_attrs(&self, attrs: &Vec<(Span, Span)>) -> String {
        attrs
            .iter()
            .map(|(key, val)| {
                format!(" {}=\"{}\"", self.sm.slice(key), self.sm.slice(val))
            })
            .collect()
    }
}
