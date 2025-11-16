#[derive(Debug)]
pub enum Tok<'a> {
    Tag {
        name: &'a str,
        attrs: Vec<(&'a str, &'a str)>,
        children: Vec<Tok<'a>>,
    },
    CompTag {
        name: &'a str,
        attrs: Vec<(&'a str, &'a str)>,
        children: Vec<Tok<'a>>,
    },
    Text {
        cont: &'a str,
    },
}
