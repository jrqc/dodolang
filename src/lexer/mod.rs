pub mod lexer;

fn is_letter(ch: char) -> bool {
    ch.is_alphabetic() || ch == '_'
}
fn is_digit(ch: char) -> bool {
    ch.is_digit(10)
}
