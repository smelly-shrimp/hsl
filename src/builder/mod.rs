use std::fmt::Display;

use crate::tok::Tok;

impl<'a> Display for Tok<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tag { name, attrs, children } => {
                let f_attrs = attrs
                    .iter()
                    .map(|(key, val)| format!(" {}=\"{}\"", key, val))
                    .collect::<String>();

                let mut f_children = String::new();
                f_children.push_str(&children
                    .iter()
                    .map(|tok| format!("{}", tok))
                    .collect::<String>());

                write!(f, "<{}{}>\n{}\n</{}>", name, f_attrs, f_children, name)
            }
            Self::CompTag { name, attrs, children } => {
                write!(f, "<@{}></@{}>", name, name)
            }
            Self::Text { cont } => {
                write!(f, "{}", cont)
            }
        }
    }
}
