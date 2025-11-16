use std::{iter::Peekable, str::Chars};

#[derive(Debug)]
enum El<'a> {
    Id(&'a str),
    Text(&'a str),
}

#[derive(Debug)]
struct Tag<'a> {
    name: &'a str,
}

impl<'a> Tag<'a> {
    fn new(name: &El<'a>) -> Self {
        let El::Id(name) = name else {
            panic!("name is expected to be ID");
        };

        Tag {
            name,
        }
    }
}

struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
    pos: usize,
    els: Vec<Tag<'a>>,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            chars: src.chars().peekable(),
            pos: 0,
            els: Vec::new(),
        }
    }

    fn lex(&mut self, src: &'a str) {
        let mut tags = Vec::new();

        while let Some(next) = self.next() {
            if next == '<' {
                let id = self.lex_id(src);
                self.expect('>');
                tags.push(Tag::new(&id));
            }
        }

        println!("{:?}", tags);
    }

    fn lex_id(&mut self, src: &'a str) -> El<'a> {
        let start = self.pos;
        while let Some(next) = self.peek() {
            if !next.is_alphanumeric() {
                break;
            }

            self.next();
        }

        El::Id(&src[start..self.pos])
    }

    fn next(&mut self) -> Option<char> {
        self.pos += 1;
        self.chars.next()
    }

    fn peek(&mut self) -> Option<&char> {
        self.chars.peek()
    }

    fn expect(&mut self, c: char) {
        if let Some(next) = self.next() {
            if next != c {
                panic!("expected {}, got {}", c, next);
            }
        }
    }
}

fn main() {
    // let input = "<h1>Hello world!</h1>";
    let input = "<h1>";
    let mut lexer = Lexer::new(input);
    lexer.lex(&input);
}
