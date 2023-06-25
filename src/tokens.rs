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
