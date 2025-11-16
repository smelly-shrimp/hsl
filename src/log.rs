pub fn ok(msg: &str) {
    eprintln!("\x1b[1;32mOK\x1b[0m {}", msg);
}

pub fn err(msg: &str) -> ! {
    eprintln!("\x1b[1;31mERR\x1b[0m {}", msg);
    std::process::exit(1);
}
