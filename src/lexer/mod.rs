use crate::tok::Tok;

mod prim;

pub struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    pub fn new(src: &str) -> Self {
        Self {
            chars: src.chars().collect(),
            pos: 0,
        }
    }

    pub fn lex<'a>(&mut self, src: &'a str) -> Vec<Tok<'a>> {
        let mut toks = Vec::new();

        while !self.is_eof() {
            toks.push(self.to_tok(src));
        }

        println!("{:?}", toks);
        toks
    }

    pub fn curr(&self) -> char {
        *self.chars.get(self.pos).unwrap_or(&'\0')
    }

    pub fn next(&mut self) -> char {
        self.pos += 1;
        *self.chars.get(self.pos - 1).unwrap_or(&'\0')
    }

    pub fn peek(&mut self) -> char {
        *self.chars.get(self.pos + 1).unwrap_or(&'\0')
    }

    pub fn eat(&mut self, c: char) -> bool {
        if self.curr() == c {
            self.next();
            return true;
        }

        false
    }

    pub fn is_eof(&self) -> bool {
        self.curr() == '\0'
    }

    pub fn expect(&mut self, c: char) {
        let next = self.next();
        if next != c {
            panic!("expected `{}`, got `{}`", c, next);
        }
    }

    pub fn next_while(&mut self, is: impl Fn(char) -> bool) -> usize {
        let start = self.pos;

        while !self.is_eof() && is(self.curr()) {
            self.next();
        }

        start
    }

    pub fn eat_space(&mut self) {
        self.next_while(|c| c == ' ' || c == '\t' || c == '\n');
    }

    pub fn expect_space(&mut self) {
        let start = self.next_while(|c| c == ' ' || c == '\t' || c == '\n');
        if start == self.pos {
            panic!("expected <space>");
        }
    }
}
