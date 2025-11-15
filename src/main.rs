use std::str::Chars;

struct Tok {
    kind: TokKind,
    span: (usize, usize),
}

enum TokKind {
    Id,
    Val,
    LAngleBracket,
    Text,
}

struct Lexer<'a> {
    src: Chars<'a>,
}

impl<'a> Lexer<'a> {
    fn new(src: &'a str) -> Self {
        Self {
            src: src.chars(),
        }
    }

    fn next(&mut self) -> char {
        self.src.next().unwrap_or('\0')
    }
}

fn main() {
    let input = "<h1>Hello world!</h1>";
    let mut lexer = Lexer::new(input);
    println!("{}", lexer.next());
}
