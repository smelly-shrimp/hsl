use std::fs;

use crate::tok::Span;

pub struct SrcMan {
    srcs: Vec<Src>,
}

pub struct Src {
    path: String,
    cont: String,
}

impl SrcMan {
    pub fn new() -> Self {
        Self { srcs: Vec::new() }
    }

    pub fn load(&mut self, path: String) -> usize {
        self.srcs.push(Src::new(path));
        self.srcs.len().saturating_sub(1)
    }

    pub fn src(&self, sid: usize) -> &str {
        &self.srcs.get(sid).expect("HANDLE ERR! no-src").cont
    }

    pub fn slice(&self, span: &Span) -> &str {
        let cont = self.src(span.0);
        cont.get(span.1..span.2).expect("HANDLE ERR! out-of-bounds")
    }
}

impl Src {
    fn new(path: String) -> Self {
        let mut cont = fs::read_to_string(&path).expect("HANDLE ERR! no-file");
        let len = cont.trim().len();
        cont.truncate(len);
        Self { path: path, cont }
    }
}
