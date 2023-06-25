#[test]
fn simple() {
    use crate::lexer::{Lexer, Token};
    let mut lexer = Lexer::new("1 + 2".to_string());
    let tokens = lexer.lex().unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].value, Token::Int(1));
    assert_eq!(tokens[1].value, Token::Symbol('+'));
    assert_eq!(tokens[2].value, Token::Int(2));
}
#[test]
fn symbols() {
    use crate::lexer::{Lexer, Token};
    let mut lexer = Lexer::new("1 + 2 ++ ".to_string()).symbols(&["+", "++"]);
    let tokens = lexer.lex().unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].value, Token::Int(1));
    assert_eq!(tokens[1].value, Token::Symbol('+'));
    assert_eq!(tokens[2].value, Token::Int(2));
    assert_eq!(tokens[3].value, Token::LongSymbol("++".to_string()));
}
