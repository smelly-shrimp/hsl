use crate::lexer::Lexer;

mod tok;
mod lexer;

fn main() {
    let input = r#"<div id="this-one" class="sth">
        <h1>Foo</h1>
        <@desc/>
    </div>"#;

    let mut lexer = Lexer::new(input);
    lexer.lex(&input);
}
