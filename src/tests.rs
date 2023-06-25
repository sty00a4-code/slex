#[test]
fn simple() {
    use crate::lexer::Lexer;
    use crate::tokens::Token;
    let mut lexer = Lexer::new("1 + 2".to_string());
    let tokens = lexer.lex().unwrap();
    assert_eq!(tokens.len(), 3);
    assert_eq!(tokens[0].value, Token::Int(1));
    assert_eq!(tokens[1].value, Token::Symbol('+'));
    assert_eq!(tokens[2].value, Token::Int(2));
}
#[test]
fn symbols() {
    use crate::lexer::Lexer;
    use crate::tokens::Token;
    let mut lexer = Lexer::new("1 + 2 ++ ".to_string()).symbols(&["+", "++"]);
    let tokens = lexer.lex().unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].value, Token::Int(1));
    assert_eq!(tokens[1].value, Token::Symbol('+'));
    assert_eq!(tokens[2].value, Token::Int(2));
    assert_eq!(tokens[3].value, Token::LongSymbol("++".to_string()));
}
#[test]
fn keywords() {
    use crate::lexer::Lexer;
    use crate::tokens::Token;
    let mut lexer = Lexer::new("local a = 1".to_string())
        .symbols(&["="])
        .keywords(&["local"]);
    let tokens = lexer.lex().unwrap();
    assert_eq!(tokens.len(), 4);
    assert_eq!(tokens[0].value, Token::Keyword("local".to_string()));
    assert_eq!(tokens[1].value, Token::Ident("a".to_string()));
    assert_eq!(tokens[2].value, Token::Symbol('='));
    assert_eq!(tokens[3].value, Token::Int(1));
}
#[test]
fn simple_indent() {
    use crate::indent_lexer::Lexer;
    use crate::tokens::Token;
    let mut lexer = Lexer::new("    1 + 2".to_string());
    let lines = lexer.lex().unwrap();
    assert_eq!(lines[0].len(), 3);
    assert_eq!(lines[0].indent, 4);
    assert_eq!(lines[0].tokens[0].value, Token::Int(1));
    assert_eq!(lines[0].tokens[1].value, Token::Symbol('+'));
    assert_eq!(lines[0].tokens[2].value, Token::Int(2));
}
