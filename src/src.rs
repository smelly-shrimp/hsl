use std::fs;

use crate::{log, tok::Span};

pub struct SrcMan {
    srcs: Vec<Src>,
}

pub struct Src {
    cont: String,
}

impl SrcMan {
    pub fn new() -> Self {
        Self { srcs: Vec::new() }
    }

    pub fn load(&mut self, path: String) -> usize {
        self.srcs.push(Src::new(&path));
        self.srcs.len().saturating_sub(1)
    }

    pub fn src(&self, sid: usize) -> &str {
        let Some(src) = self.srcs.get(sid) else {
            log::err(&format!("no src with id {}", sid));
        };

        &src.cont
    }

    pub fn slice(&self, span: &Span) -> &str {
        let cont = self.src(span.0);
        cont.get(span.1..span.2).unwrap_or_else(|| {
            log::err(&format!("invalid range <{}..{}>", span.1, span.2))
        })
    }
}

impl Src {
    fn new(path: &str) -> Self {
        let Ok(cont) = fs::read_to_string(path) else {
            log::err(&format!("no file '{}'", path))
        };

        let cont = cont.trim().into();
        Self { cont }
    }
}
