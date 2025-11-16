use crate::{lexer::Lexer, tok::Tok};

impl Lexer {
    pub fn to_tok<'a>(&mut self, src: &'a str) -> Tok<'a> {
        if !self.eat('<') {
            return Tok::Text {
                cont: self.lex_text(src)
            };
        }

        let is_comp = self.eat('@');
        let id = self.lex_id(src);

        let attrs = self.lex_attrs(src);
        // let attrs = if self.curr() != '>' || self.curr() != '/' {
        //     self.lex_attrs(src)
        // } else { Vec::new() };

        let mk_tag = |name, children| -> Tok {
            if is_comp {
                return Tok::CompTag {
                    name,
                    attrs,
                    children,
                };
            }

            Tok::Tag {
                name,
                attrs,
                children,
            }
        };

        if self.eat('/') {
            self.expect('>');

            return mk_tag(id, Vec::new());
        }

        self.expect('>');

        let mut children = Vec::new();
        self.next_while(|c| matches!(c, ' ' | '\n'));
        while self.curr() != '<' || self.peek() != '/' {
            children.push(self.to_tok(src));
            self.next_while(|c| matches!(c, ' ' | '\n'));
        }

        if self.peek() == '/' {
            self.expect('<');
            self.next();

            if is_comp {
                self.expect('@');
            }

            let id_close = self.lex_id(src);
            if id != id_close {
                panic!("no corresponding closing tag");
            }
            self.expect('>');
        }

        self.next_while(|c| matches!(c, ' ' | '\n'));

        mk_tag(id, children)
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
            self.expect(' ');

            let key = self.lex_id(src);
            let val = if self.eat('=') {
                self.lex_val(src)
            } else { "" };

            attrs.push((key, val));
        }

        attrs
    }
}
