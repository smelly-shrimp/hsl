#[derive(Clone, Debug)]
pub struct Span(pub usize, pub usize, pub usize);

#[derive(Clone, Debug)]
pub enum Tok {
    Root {
        children: Vec<Tok>,
    },
    Doctype {
        cont: Span,
    },
    Tag {
        name: Span,
        attrs: Vec<(Span, Vec<Span>)>,
        children: Vec<Tok>,
    },
    VoidTag {
        name: Span,
        attrs: Vec<(Span, Vec<Span>)>,
    },
    Text {
        parts: Vec<Span>,
    },
    // TextConcat {
    //     parts: Vec<Span>,
    // }
}

// impl Display for Tok {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         match self {
//             Self::Root { children } => {
//                 let children = fmt_children(children);
//                 write!(f, "{}", children)
//             }
//             Self::Tag {
//                 name,
//                 attrs,
//                 children,
//             } => {
//                 let f_attrs = fmt_attrs(attrs);
//                 let f_children = fmt_children(children);
//
//                 if name == &"include" {
//                     let Some((_, path)) = attrs.iter().find(|(name, _)| name == &"src") else {
//                         log::err("no src attribute inside <include>");
//                     };
//
//                     let Ok(cont) = fs::read_to_string(path) else {
//                         log::err(&format!("no file `{}`", path));
//                     };
//
//                     return write!(f, "{}", cont);
//                 }
//
//                 let mut sep = "\n";
//
//                 if f_children.is_empty() {
//                     sep = "";
//                 }
//
//                 if children.len() == 1
//                     && let Tok::Text { .. } = children[0]
//                 {
//                     sep = "";
//                 }
//
//                 write!(
//                     f,
//                     "<{}{}>{}{}{}</{}>",
//                     name, f_attrs, sep, f_children, sep, name
//                 )
//             }
//             Self::VoidTag { name, attrs } => {
//                 let attrs = fmt_attrs(attrs);
//                 write!(f, "<{}{}>\n", name, attrs)
//             }
//             Self::Doctype { cont } => write!(f, "<!{}>", cont),
//             Self::Text { cont } => write!(f, "{}", cont.trim()),
//         }
//     }
// }

// fn fmt_attrs(attrs: &Vec<(&str, &str)>) -> String {
//     attrs
//         .iter()
//         .map(|(key, val)| format!(" {}=\"{}\"", key, val))
//         .collect::<String>()
// }
//
// fn fmt_children(children: &Vec<Tok>) -> String {
//     children
//         .iter()
//         .map(|tok| format!("{}", tok))
//         .collect::<String>()
// }

pub trait IsVoid {
    fn is_void(&self) -> bool;
}

impl IsVoid for str {
    fn is_void(&self) -> bool {
        matches!(
            self,
            "area"
                | "base"
                | "br"
                | "col"
                | "embed"
                | "hr"
                | "img"
                | "input"
                | "link"
                | "meta"
                | "source"
                | "track"
                | "wbr"
                | "children"
        )
    }
}
