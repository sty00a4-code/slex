#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String),
    Keyword(String),
    Int(i64),
    Float(f64),
    Char(char),
    String(String),
    Symbol(char),
    LongSymbol(String),
}
impl Token {
    pub fn name(&self) -> String {
        match self {
            Token::Ident(_) => "identifier".to_string(),
            Token::Keyword(kw) => kw.to_string(),
            Token::Int(_) => "integer".to_string(),
            Token::Float(_) => "decimal point number".to_string(),
            Token::Char(_) => "character".to_string(),
            Token::String(_) => "string".to_string(),
            Token::Symbol(sym) => format!("{sym:?}"),
            Token::LongSymbol(sym) => format!("{sym:?}"),
        }
    }
}
