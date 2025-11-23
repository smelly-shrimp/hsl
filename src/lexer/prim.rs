use crate::{
    cur::Cur,
    lexer::Lexer,
    tok::{IsVoid, Span, Tok},
};

impl<'a> Lexer<'a> {
    pub fn to_tok(&mut self) -> Tok {
        let mut is_closed = false;

        if !self.eat("<") {
            return Tok::Text {
                parts: self.lex_text("<"),
            };
        }

        if self.eat("!") {
            let cont = self.lex_doctype();
            self.expect(">");
            return Tok::Doctype { cont };
        }

        let name = self.lex_id();
        let attrs = self.lex_attrs();
        let mut children = Vec::new();

        if self.text(&name).is_void() {
            self.expect(">");

            if self.text(&name) == "children" {
                return Tok::Root {
                    children: self.find_children(),
                };
            }

            return Tok::VoidTag { name, attrs };
        }

        if self.eat("/") {
            is_closed = true;
        }
        self.expect(">");

        if !is_closed {
            self.eat_space();
            while !self.is_eof() && (self.curr() != "<" || self.peek() != "/") {
                let child = self.to_tok();
                children.push(child);

                self.eat_space();
            }

            self.expect("<");
            self.expect("/");
            let name_close = self.lex_id();
            self.expect(">");

            if self.text(&name) != self.text(&name_close) {
                panic!("no corresponding closing tag");
            }
        }

        if self.text(&name) == "include" {
            for (key, val) in &attrs {
                if self.text(&key) == "src" {
                    let sid = self.sm.load(String::from(
                        self.text(&val.get(0).expect("HANDLE ERR! no-path")),
                    ));
                    self.curs.push(Cur::new(sid, attrs, children));
                    return self.lex();
                }
            }
        }

        Tok::Tag {
            name,
            attrs,
            children,
        }
    }

    fn lex_id(&mut self) -> Span {
        let start = self.next_while(|s| {
            let s = s.as_bytes()[0];
            s.is_ascii_alphanumeric() || s == 45 // 45 = '-'
        });
        Span(self.sid(), start, self.pos())
    }

    fn lex_text(&mut self, end: &str) -> Vec<Span> {
        let start = self.pos();
        let mut parts = Vec::new();
        let mut part = Span(self.sid(), start, start);

        while !self.is_eof() && self.curr() != end {
            let s = self.next();

            if s == "{" {
                parts.push(part.clone());

                let attr_key = self.lex_id();
                let attr_key = self.sm.slice(&attr_key);
                for part in self.find_attr(attr_key) {
                    parts.push(part);
                }
                part.1 = self.pos() + 1;
                part.2 = self.pos();

                self.expect("}");
            }

            part.2 += 1;
        }

        parts.push(part);
        parts
    }

    fn lex_val(&mut self) -> Vec<Span> {
        self.expect("\"");
        let val = self.lex_text("\"");
        self.expect("\"");

        val
    }

    fn lex_attrs(&mut self) -> Vec<(Span, Vec<Span>)> {
        let mut attrs = Vec::new();

        while self.curr() != ">" && self.curr() != "/" {
            self.expect_space();

            let key = self.lex_id();
            if self.text(&key).is_empty() {
                return attrs;
            }

            let val = if self.eat("=") {
                self.lex_val()
            } else {
                Vec::new()
            };

            attrs.push((key, val));
        }

        attrs
    }

    fn lex_doctype(&mut self) -> Span {
        let start = self.next_while(|c| c != ">");
        Span(self.sid(), start, self.pos())
    }
}
