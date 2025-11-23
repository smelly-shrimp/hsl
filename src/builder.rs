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
        self.build_tok(root, 0)
    }

    pub fn build_tok(&self, tok: &Tok, indent: usize) -> String {
        let fmt = match tok {
            Tok::Root { children } => self
                .fmt_children(children, indent.saturating_sub(1))
                .trim()
                .into(),
            Tok::Doctype { cont } => {
                format!("<!doctype {}>", self.sm.slice(&cont))
            }
            Tok::Text { parts } => parts
                .iter()
                .map(|p| self.sm.slice(p))
                .collect::<String>()
                .trim()
                .into(),
            Tok::Tag {
                name,
                attrs,
                children,
            } => {
                let name = self.sm.slice(&name);
                let children = self.fmt_children(children, indent);
                let attrs = self.fmt_attrs(attrs);

                format!("<{}{}>{}</{}>", name, attrs, children, name)
            }
            Tok::VoidTag { name, attrs } => {
                let name = self.sm.slice(&name);
                let attrs = self.fmt_attrs(attrs);

                format!("<{}{}>", name, attrs)
            }
        };

        fmt
    }

    fn fmt_children(&self, children: &Vec<Tok>, indent: usize) -> String {
        let mut fmt = String::new();
        let is_text_only =
            children.len() == 1 && matches!(children[0], Tok::Text { .. });

        for child in children {
            let prefix = if !is_text_only {
                &format!("\n{}", "\t".repeat(indent))
            } else {
                ""
            };

            fmt.push_str(&format!(
                "{}{}",
                prefix,
                self.build_tok(child, indent + 1)
            ));
        }

        if children.len() > 0 && !is_text_only {
            fmt.push_str(&format!(
                "\n{}",
                "\t".repeat(indent.saturating_sub(1))
            ));
        }

        fmt
    }

    fn fmt_attrs(&self, attrs: &[(Span, Vec<Span>)]) -> String {
        attrs
            .iter()
            .map(|(key, val)| {
                format!(
                    " {}=\"{}\"",
                    self.sm.slice(key),
                    val.iter().map(|v| self.sm.slice(v)).collect::<String>()
                )
            })
            .collect()
    }
}
