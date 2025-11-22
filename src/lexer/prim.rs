use crate::{
    cur::Cur, lexer::Lexer, tok::{IsVoid, Span, Tok}
};

impl<'a> Lexer<'a> {
    pub fn to_tok(&mut self) -> Tok {
        if !self.eat('<') {
            return Tok::Text {
                cont: self.lex_text(),
            };
        }

        if self.eat('!') {
            let cont = self.lex_doctype();
            self.expect('>');
            return Tok::Doctype { cont };
        }

        let name = self.lex_id();
        let attrs = self.lex_attrs();

        if self.text(&name) == "include" {
            for (key, val) in &attrs {
                if self.text(&key) == "src" {
                    let sid = self.sm.load(String::from(self.text(&val)));
                    self.curs.push(Cur::new(sid));
                    // lex
                }
            }
        }

        if self.text(&name).is_void() {
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
            children.push(self.to_tok());
        }

        if self.peek() == '/' {
            self.expect('<');
            self.next();

            let name_close = self.lex_id();

            if self.text(&name) != self.text(&name_close) {
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

    fn lex_id(&mut self) -> Span {
        let start = self.next_while(|c| c.is_alphanumeric());
        Span(self.sid(), start, self.pos)
    }

    fn lex_text(&mut self) -> Span {
        let start = self.next_while(|c| c != '<');
        Span(self.sid(), start, self.pos)
    }

    fn lex_val(&mut self) -> Span {
        self.expect('"');
        let start = self.next_while(|c| c != '"');
        self.expect('"');

        Span(self.sid(), start, self.pos - 1)
    }

    fn lex_attrs(&mut self) -> Vec<(Span, Span)> {
        let mut attrs = Vec::new();

        while self.curr() != '>' && self.curr() != '/' {
            self.expect_space();

            let key = self.lex_id();
            if self.text(&key).is_empty() {
                return attrs;
            }

            let val = if self.eat('=') {
                self.lex_val()
            } else {
                Span(self.sid(), self.pos, self.pos)
            };

            attrs.push((key, val));
        }

        attrs
    }

    fn lex_doctype(&mut self) -> Span {
        let start = self.next_while(|c| c != '>');
        Span(self.sid(), start, self.pos)
    }
}
