use std::fs;

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

    pub fn load(&mut self, path: &str) -> usize {
        self.srcs.push(Src::new(path));
        self.srcs.len().saturating_sub(1)
    }

    pub fn src(&mut self, sid: usize) -> &str {
        &self.srcs.get(sid).expect("HANDLE ERR! no-src").cont
    }
}

impl Src {
    fn new(path: &str) -> Self {
        let cont = fs::read_to_string(path).expect("HANDLE ERR! no-file");
        Self {
            path: path.into(),
            cont,
        }
    }
}
