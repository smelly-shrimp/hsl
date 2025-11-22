use crate::{
    lexer::Lexer,
    tok::{IsVoid, Span, Tok},
};

impl Lexer {
    pub fn to_tok(&mut self, src: &str) -> Tok {
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
            self.expect('>');
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
        while !self.is_eof() && (self.curr() != '<' || self.peek() != '/') {
            children.push(self.to_tok(src));
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

        if self.is_eof() {
            panic!("no corresponding closing tag");
        }

        Tok::Tag {
            name,
            attrs,
            children,
        }
    }

    fn lex_id(&mut self, src: &str) -> &str {
        let start = self.next_while(|c| c.is_alphanumeric());
        &src[start..self.pos]
    }

    fn lex_text(&mut self, src: &str) -> Span {
        let start = self.next_while(|c| c != '<');
        &src[start..self.pos]
    }

    fn lex_val(&mut self, src: &str) -> &str {
        self.expect('"');
        let start = self.next_while(|c| c != '"');
        self.expect('"');

        &src[start..self.pos - 1]
    }

    fn lex_attrs(&mut self, src: &str) -> Vec<(&str, &str)> {
        let mut attrs = Vec::new();

        while self.curr() != '>' && self.curr() != '/' {
            self.expect_space();

            let key = self.lex_id(src);
            if key.is_empty() {
                return attrs;
            }

            let val = if self.eat('=') { self.lex_val(src) } else { "" };

            attrs.push((key, val));
        }

        attrs
    }

    fn lex_doctype(&mut self, src: &str) -> &str {
        let start = self.next_while(|c| c != '>');
        &src[start..self.pos]
    }
}
