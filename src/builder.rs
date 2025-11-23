use crate::{src::SrcMan, tok::Tok};

pub struct Builder<'a> {
    sm: &'a SrcMan,
}

impl<'a> Builder<'a> {
    pub fn new(sm: &'a SrcMan) -> Self {
        Self { sm }
    }

    pub fn build(&self, root: Tok) -> String {
        String::new()
    }
}
