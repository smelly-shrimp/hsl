#[derive(Debug)]
enum Tok<'a> {
    Tag {
        name: &'a str,
        children: Vec<Tok<'a>>,
    },
    Text {
        cont: &'a str,
    }
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
            match self.curr() {
                '<' => {
                    self.next();

                    let id = self.lex_id(src);

                    if self.curr() == '/' {
                        self.next();
                        self.expect('>');

                        toks.push(Tok::Tag {
                            name: id,
                            children: Vec::new(),
                        });

                        continue;
                    }

                    self.expect('>');

                    toks.push(Tok::Tag {
                        name: id,
                        children: Vec::new(),
                    });
                },
                _ => toks.push(Tok::Text {
                    cont: self.lex_text(src)
                })
            }
        }

        println!("{:?}", toks);
    }

    fn lex_id<'a>(&mut self, src: &'a str) -> &'a str {
        let start = self.next_until(|c| c.is_alphanumeric());
        &src[start..self.pos]
    }

    fn lex_text<'a>(&mut self, src: &'a str) -> &'a str {
        let start = self.next_until(|c| c != '<');
        &src[start..self.pos]
    }

    fn next(&mut self) -> char {
        self.pos += 1;
        *self.chars.get(self.pos - 1).unwrap_or(&'\0')
    }

    pub fn curr(&self) -> char {
        *self.chars.get(self.pos).unwrap_or(&'\0')
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

    fn next_until(&mut self, is_char: impl Fn(char) -> bool) -> usize {
        let start = self.pos;

        while !self.is_eof() && is_char(self.curr()) {
            self.next();
        }

        start
    }
}

fn main() {
    let input = "<h1>Hello world!";
    // let input = "<h1>";
    let mut lexer = Lexer::new(input);
    lexer.lex(&input);
}
