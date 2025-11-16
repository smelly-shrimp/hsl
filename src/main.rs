use std::{env, fs::{self}};

use crate::{lexer::Lexer};

mod log;
mod tok;
mod lexer;
mod builder;

fn main() {
    let mut args = env::args();
    args.next();

    let fp = args.next().unwrap_or_else(|| {
        log::err("no input file");
    });

    let Ok(fc) = fs::read_to_string(&fp) else {
        log::err(&format!("no file `{}`", &fp));
    };

    log::ok(&format!("reading file `{}`", &fp));

    let mut lexer = Lexer::new(&fc);
    let toks = lexer.lex(&fc);

    for tok in toks {
        println!("{}", tok);
    }
}
