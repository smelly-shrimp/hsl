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

        let mk_tag = |name, children| -> Tok {
            if is_comp {
                return Tok::CompTag {
                    name,
                    children
                };
            }

            Tok::Tag {
                name,
                children
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
}
