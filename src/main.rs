use std::{
    env,
    fs::{self},
    time::Instant,
};

use crate::lexer::Lexer;

mod builder;
mod lexer;
mod log;
mod tok;

fn main() {
    let start = Instant::now();

    let mut args = env::args();
    args.next();

    let fp = args.next().unwrap_or_else(|| {
        log::err("no input file");
    });

    let Ok(fc) = fs::read_to_string(&fp) else {
        log::err(&format!("no file `{}`", &fp));
    };

    log::ok(&format!("read file `{}` in {:?}", &fp, start.elapsed()));
    let start = Instant::now();

    let mut lexer = Lexer::new(&fc);
    let toks = lexer.lex(&fc);

    log::ok(&format!("lexed files in {:?}", start.elapsed()));

    for tok in toks {
        println!("{}", tok);
    }
}
