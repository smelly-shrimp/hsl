use std::fmt::Display;

#[derive(Debug)]
pub enum Tok<'a> {
    Doctype {
        cont: String,
    },
    Tag {
        name: &'a str,
        attrs: Vec<(&'a str, &'a str)>,
        children: Vec<Tok<'a>>,
    },
    VoidTag {
        name: &'a str,
        attrs: Vec<(&'a str, &'a str)>,
    },
    Text {
        cont: &'a str,
    },
}

fn fmt_attrs(attrs: &Vec<(&str, &str)>) -> String {
    attrs
        .iter()
        .map(|(key, val)| format!(" {}=\"{}\"", key, val))
        .collect::<String>()
}

fn fmt_children(children: &Vec<Tok>) -> String {
    children
        .iter()
        .map(|tok| format!("{}", tok))
        .collect::<String>()
}

impl<'a> Display for Tok<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tag {
                name,
                attrs,
                children,
            } => {
                let attrs = fmt_attrs(attrs);
                let children = fmt_children(children);
                write!(f, "<{}{}>\n{}\n</{}>", name, attrs, children, name)
            }
            Self::VoidTag { name, attrs } => {
                let attrs = fmt_attrs(attrs);
                write!(f, "<{}{}>", name, attrs)
            }
            Self::Doctype { cont } => write!(f, "<!{}>", cont),
            Self::Text { cont } => write!(f, "{}", cont),
        }
    }
}
