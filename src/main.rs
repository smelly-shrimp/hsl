use std::{
    env,
    fs::{self},
    time::Instant,
};

use crate::lexer::Lexer;

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
    let root = lexer.lex(&fc);

    log::ok(&format!("lexed files in {:?}", start.elapsed()));

    println!("\x1b[1;34m--- output:\x1b[0m\n{}", root);
}
