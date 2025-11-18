use crate::{
    lexer::Lexer,
    tok::{IsVoid, Tok},
};

impl Lexer {
    pub fn to_tok<'a>(&mut self, src: &'a str) -> Tok<'a> {
        if !self.eat('<') {
            return Tok::Text {
                cont: self.lex_text(src),
            };
        }

        if self.eat('!') {
            let cont = self.lex_doctype(src);
            self.expect('>');
            return Tok::Doctype { cont: cont.into() };
        }

        let name = self.lex_id(src);
        let attrs = self.lex_attrs(src);

        if name.is_void() {
            return Tok::VoidTag { name, attrs };
        }

        if self.eat('/') {
            self.expect('>');

            return Tok::Tag {
                name,
                attrs,
                children: Vec::new(),
            };
        }

        self.expect('>');

        let mut children = Vec::new();
        self.eat_space();
        while self.curr() != '<' || self.peek() != '/' {
            children.push(self.to_tok(src));
            self.eat_space();
        }

        if self.peek() == '/' {
            self.expect('<');
            self.next();

            let id_close = self.lex_id(src);
            if name != id_close {
                panic!("no corresponding closing tag");
            }
            self.expect('>');
        }

        self.eat_space();

        Tok::Tag {
            name,
            attrs,
            children,
        }
    }

    fn lex_id<'a>(&mut self, src: &'a str) -> &'a str {
        let start = self.next_while(|c| c.is_alphanumeric());
        &src[start..self.pos]
    }

    fn lex_text<'a>(&mut self, src: &'a str) -> &'a str {
        let start = self.next_while(|c| c != '<');
        &src[start..self.pos]
    }

    fn lex_val<'a>(&mut self, src: &'a str) -> &'a str {
        self.expect('"');
        let start = self.next_while(|c| c != '"');
        self.expect('"');

        &src[start..self.pos - 1]
    }

    fn lex_attrs<'a>(&mut self, src: &'a str) -> Vec<(&'a str, &'a str)> {
        let mut attrs = Vec::new();

        while self.curr() != '>' && self.curr() != '/' {
            self.expect_space();

            let key = self.lex_id(src);
            let val = if self.eat('=') { self.lex_val(src) } else { "" };

            attrs.push((key, val));
        }

        attrs
    }

    fn lex_doctype<'a>(&mut self, src: &'a str) -> &'a str {
        let start = self.next_while(|c| c != '>');
        &src[start..self.pos]
    }
}
