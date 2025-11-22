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
        Self {
            srcs: Vec::new(),
        }
    }

    pub fn load(&mut self, path: &str) {
        self.srcs.push(Src::new(path));
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
