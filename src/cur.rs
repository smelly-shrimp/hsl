use crate::tok::{Span, Tok};

pub struct Cur {
    sid: usize,
    pos: usize,
    attrs: Vec<(Span, Vec<Span>)>,
    children: Vec<Tok>,
}

impl Cur {
    pub fn new(
        sid: usize,
        attrs: Vec<(Span, Vec<Span>)>,
        children: Vec<Tok>,
    ) -> Self {
        Self {
            sid,
            pos: 0,
            attrs,
            children,
        }
    }

    pub fn sid(&self) -> usize {
        self.sid
    }

    pub fn pos(&self) -> usize {
        self.pos
    }

    pub fn inc_pos(&mut self) {
        self.pos += 1;
    }

    pub fn attrs(&self) -> &[(Span, Vec<Span>)] {
        &self.attrs
    }

    pub fn children(&self) -> &Vec<Tok> {
        &self.children
    }
}
