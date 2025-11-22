use std::{
    env,
    fs::{self},
    time::Instant,
};

use crate::{lexer::Lexer, src::SrcMan};

mod cur;
mod lexer;
mod log;
mod src;
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

    let mut sm = SrcMan::new();
    let sid = sm.load(&fp);

    log::ok(&format!("read file `{}` in {:?}", &fp, start.elapsed()));
    let start = Instant::now();

    let mut lexer = Lexer::new(&mut sm, sid, &fc);
    let _ = lexer.lex(&fc);

    log::ok(&format!("lexed files in {:?}", start.elapsed()));

    // println!("\x1b[1;34m--- output:\x1b[0m\n{}", root);
}
