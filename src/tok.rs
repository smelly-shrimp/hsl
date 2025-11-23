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
}

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
