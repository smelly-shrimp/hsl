use std::env;

use crate::{builder::Builder, lexer::Lexer, src::SrcMan};

mod cur;
mod lexer;
mod log;
mod src;
mod tok;
mod builder;

fn main() {
    let mut args = env::args();
    args.next();

    let fp = args.next().unwrap_or_else(|| {
        log::err("no input file");
    });

    let mut sm = SrcMan::new();
    let sid = sm.load(fp);

    let mut lexer = Lexer::new(&mut sm, sid);
    let root = lexer.lex();
    println!("{:?}", root);

    let builder = Builder::new(&sm);
    println!("{}", builder.build(root));

    // println!("\x1b[1;34m--- output:\x1b[0m\n{}", root);
}
