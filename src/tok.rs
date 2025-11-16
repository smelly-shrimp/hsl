#[derive(Debug)]
pub enum Tok<'a> {
    Tag {
        name: &'a str,
        children: Vec<Tok<'a>>,
    },
    CompTag {
        name: &'a str,
        children: Vec<Tok<'a>>,
    },
    Text {
        cont: &'a str,
    },
    Empty,
}
