use std::fmt::Display;

#[derive(Debug)]
pub enum Tok<'a> {
    Tag {
        name: &'a str,
        attrs: Vec<(&'a str, &'a str)>,
        children: Vec<Tok<'a>>,
    },
    Doctype {
        cont: String,
    },
    CompTag {
        name: &'a str,
        attrs: Vec<(&'a str, &'a str)>,
        children: Vec<Tok<'a>>,
    },
    Text {
        cont: &'a str,
    },
}

impl<'a> Display for Tok<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Tag {
                name,
                attrs,
                children,
            } => {
                let f_attrs = attrs
                    .iter()
                    .map(|(key, val)| format!(" {}=\"{}\"", key, val))
                    .collect::<String>();

                let mut f_children = String::new();
                f_children.push_str(
                    &children
                        .iter()
                        .map(|tok| format!("{}", tok))
                        .collect::<String>(),
                );

                write!(f, "<{}{}>\n{}\n</{}>", name, f_attrs, f_children, name)
            }
            Self::Doctype { cont } => write!(f, "<!{}>", cont),
            Self::CompTag { name, .. } => {
                write!(f, "<@{}></@{}>", name, name)
            }
            Self::Text { cont } => write!(f, "{}", cont),
        }
    }
}
