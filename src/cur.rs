use crate::tok::Span;

pub struct Cur {
    sid: usize,
    pos: usize,
    attrs: Vec<(Span, Span)>,
}

impl Cur {
    pub fn new(sid: usize, attrs: Vec<(Span, Span)>) -> Self {
        Self { sid, pos: 0, attrs }
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

    pub fn attrs(&self) -> &[(Span, Span)] {
        &self.attrs
    }
}
