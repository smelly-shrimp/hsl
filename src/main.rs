#[derive(Debug)]
enum Tok<'a> {
    Tag {
        name: &'a str,
        children: Vec<Tok<'a>>,
    },
    CompTag {
        name: &'a str,
        children: Vec<Tok<'a>>,
    },
    Text {
        cont: &'a str,
    },
    Empty,
}

struct Lexer {
    chars: Vec<char>,
    pos: usize,
}

impl Lexer {
    fn new(src: &str) -> Self {
        Self {
            chars: src.chars().collect(),
            pos: 0,
        }
    }

    fn lex(&mut self, src: &str) {
        let mut toks = Vec::new();

        while !self.is_eof() {
            toks.push(self.to_tok(src));
        }

        println!("{:?}", toks);
    }

    fn to_tok<'a>(&mut self, src: &'a str) -> Tok<'a> {
        match self.curr() {
            '<' => {
                self.next();

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
            },
            _ => Tok::Text {
                cont: self.lex_text(src)
            }
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

    pub fn curr(&self) -> char {
        *self.chars.get(self.pos).unwrap_or(&'\0')
    }

    fn next(&mut self) -> char {
        self.pos += 1;
        *self.chars.get(self.pos - 1).unwrap_or(&'\0')
    }

    fn peek(&mut self) -> char {
        *self.chars.get(self.pos + 1).unwrap_or(&'\0')
    }

    fn eat(&mut self, c: char) -> bool {
        if self.curr() == c {
            self.next();
            return true;
        }

        false
    }

    fn is_eof(&self) -> bool {
        self.curr() == '\0'
    }

    fn expect(&mut self, c: char) {
        let next = self.next();
        if next != c {
            panic!("expected `{}`, got `{}`", c, next);
        }
    }

    fn next_while(&mut self, is: impl Fn(char) -> bool) -> usize {
        let start = self.pos;

        while !self.is_eof() && is(self.curr()) {
            self.next();
        }

        start
    }
}

fn main() {
    let input = "<div>
        <h1>Foo</h1>
        <@desc/>
    </div>";
    let mut lexer = Lexer::new(input);
    lexer.lex(&input);
}
