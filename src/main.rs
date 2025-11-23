use std::{env, fs::OpenOptions, io::Write};

use crate::{builder::Builder, lexer::Lexer, src::SrcMan};

mod builder;
mod cur;
mod lexer;
mod log;
mod src;
mod tok;

fn main() {
    let mut args = env::args();
    args.next();

    let fi = args.next().unwrap_or_else(|| {
        log::err("no input file");
    });

    let fo = if let Some(p) = args.next() {
        log::ok(&format!("writing to file (`{}`)", p));
        Some(p)
    } else {
        log::ok("writing to stdout (no output file provided)");
        None
    };

    let mut sm = SrcMan::new();
    let sid = sm.load(fi);

    let mut lexer = Lexer::new(&mut sm, sid);
    let root = lexer.lex();
    // println!("{:?}", root);

    let builder = Builder::new(&sm);
    let res = builder.build(&root);

    if let Some(fo) = fo {
        let mut f = OpenOptions::new()
            .create(true)
            .write(true)
            .open(fo)
            .unwrap_or_else(|_| log::err("cannot write to file"));
        f.write_all(res.as_bytes())
            .unwrap_or_else(|_| log::err("cannot write to file "));
    } else {
        println!("{}", res);
        // println!("\x1b[1;34m--- output:\x1b[0m\n{}", res);
    }
}
