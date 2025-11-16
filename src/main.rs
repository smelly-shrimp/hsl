use std::{env, fs::{self}};

use crate::lexer::Lexer;

mod tok;
mod lexer;

fn main() {
    let mut args = env::args();
    args.next();

    let fp = args.next().unwrap_or_else(|| {
        panic!("no input file");
    });

    let Ok(fc) = fs::read_to_string(fp) else {
        panic!("no such file");
    };

    // let input = r#"<div id="this-one" class="sth">
    //     <h1>Foo</h1>
    //     <@desc/>
    // </div>"#;

    let mut lexer = Lexer::new(&fc);
    lexer.lex(&fc);
}
