use crate::{
    cur::Cur,
    log,
    src::SrcMan,
    tok::{Span, Tok},
};

mod prim;

pub struct Lexer<'a> {
    sm: &'a mut SrcMan,
    curs: Vec<Cur>,
}

impl<'a> Lexer<'a> {
    pub fn new(sm: &'a mut SrcMan, sid: usize) -> Self {
        Self {
            sm,
            curs: vec![Cur::new(sid, Vec::new(), Vec::new())],
        }
    }

    pub fn lex(&mut self) -> Tok {
        let mut toks = Vec::new();

        while !self.is_eof() {
            self.eat_space();

            toks.push(self.to_tok());
        }

        self.curs.pop();
        Tok::Root { children: toks }
    }

    fn cur(&self) -> &Cur {
        self.curs
            .last()
            .unwrap_or_else(|| log::err("no source cursor"))
    }

    fn cur_mut(&mut self) -> &mut Cur {
        self.curs
            .last_mut()
            .unwrap_or_else(|| log::err("no source cursor"))
    }

    pub fn find_attr(&self, attr: &str) -> Vec<Span> {
        self.cur()
            .attrs()
            .iter()
            .find(|(key, _)| attr == self.text(&key))
            .unwrap_or_else(|| log::err(&format!("no attribute '{}'", attr)))
            .1
            .clone()
    }

    pub fn find_children(&self) -> Vec<Tok> {
        self.cur().children().clone()
    }

    pub fn sid(&self) -> usize {
        self.cur().sid()
    }

    pub fn pos(&self) -> usize {
        self.cur().pos()
    }

    pub fn text(&self, span: &Span) -> &str {
        let cont = self.sm.src(span.0);
        cont.get(span.1..span.2).unwrap_or("")
    }

    fn one(&self, ds: isize) -> &str {
        let start = (self.pos() as isize + ds) as usize;
        self.text(&Span(self.sid(), start, start + 1))
    }

    pub fn curr(&self) -> &str {
        self.one(0)
    }

    pub fn next(&mut self) -> &str {
        self.cur_mut().inc_pos();
        self.one(-1)
    }

    pub fn peek(&mut self) -> &str {
        self.one(1)
    }

    pub fn eat(&mut self, s: &str) -> bool {
        let is = self.curr() == s;
        if is {
            self.next();
        }
        is
    }

    pub fn is_eof(&self) -> bool {
        self.curr() == ""
    }

    pub fn expect(&mut self, s: &str) {
        let cs = self.next();
        if cs != s {
            log::err(&format!("expected `{}`, got `{}`", s, cs));
        }
    }

    pub fn next_while(&mut self, is: impl Fn(&str) -> bool) -> usize {
        let start = self.pos();

        while !self.is_eof() && is(self.curr()) {
            self.next();
        }

        start
    }

    pub fn expect_space(&mut self) {
        let start = self.next_while(|s| s == " " || s == "\t" || s == "\n");
        if start == self.pos() {
            log::err("expected `space`");
        }
    }

    pub fn eat_space(&mut self) {
        self.next_while(|s| s == " " || s == "\t" || s == "\n");
    }
}
