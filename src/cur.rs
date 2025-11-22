pub struct Cur {
    sid: usize,
    pos: usize,
}

impl Cur {
    pub fn new(sid: usize) -> Self {
        Self { sid, pos: 0 }
    }
}
