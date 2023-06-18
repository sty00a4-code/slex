use std::fmt::Debug;
use crate::error::{Error, ErrorType, Position, Located};

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Ident(String), Int(i64), Float(f64), Char(char), String(String),
    Symbol(char), LongSymbol(String)
}

pub struct Lexer {
    text: String,
    symbols: Vec<String>,
    idx: usize, ln: usize, col: usize
}
impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            text,
            symbols: vec![],
            idx: 0, ln: 0, col: 0
        }
    }
    pub fn pos(&self) -> Position {
        Position {
            idx: self.idx..self.idx+1,
            ln: self.ln..self.ln+1,
            col: self.col..self.col+1
        }
    }
    pub fn advance(&mut self) {
        self.idx += 1;
        if self.text.get(self.idx..self.idx+1) == Some("\n") {
            self.ln += 1;
            self.col = 0;
        } else {
            self.col += 1;
        }
    }
    pub fn get(&self) -> Option<char> {
        self.text.get(self.idx..self.idx+1).and_then(|s| s.chars().next())
    }
    pub fn next(&mut self) -> Option<char> {
        let c = self.get();
        self.advance();
        c
    }
    pub fn has_symbols(&self) -> bool {
        self.symbols.len() > 0
    }
    pub fn symbols(mut self, symbols: &[&str]) -> Self {
        self.symbols = symbols.iter().map(|symbol| symbol.to_string()).collect();
        self
    }
    pub fn lex(&mut self) -> Result<Vec<Located<Token>>, Error> {
        let mut tokens = vec![];
        while let Some(c) = self.get() {
            let mut pos = self.pos();
            match c {
                ' ' | '\t' | '\r' | '\n' => {
                    self.advance();
                },
                '0'..='9' => {
                    let mut num = self.next().unwrap().to_string();
                    while let Some('0'..='9') = self.get() {
                        pos.extend(&self.pos());
                        num.push(self.next().unwrap());
                    }
                    if let Some('.') = self.get() {
                        pos.extend(&self.pos());
                        num.push(self.next().unwrap());
                        while let Some('0'..='9') = self.get() {
                            num.push(self.next().unwrap());
                            pos.extend(&self.pos());
                        }
                        tokens.push(Located::new(Token::Float(num.parse().unwrap()), pos));
                    } else {
                        tokens.push(Located::new(Token::Int(num.parse().unwrap()), pos));
                    }
                },
                'a'..='z' | 'A'..='Z' | '_' => {
                    let mut ident = self.next().unwrap().to_string();
                    while let Some('a'..='z') | Some('A'..='Z') | Some('_') | Some('0'..='9') = self.get() {
                        pos.extend(&self.pos());
                        ident.push(self.next().unwrap());
                    }
                    tokens.push(Located::new(Token::Ident(ident), pos));
                },
                '\'' => {
                    let mut c = self.next().unwrap();
                    if c == '\\' {
                        c = match self.next().unwrap() {
                            'n' => '\n',
                            't' => '\t',
                            'r' => '\r',
                            '\\' => '\\',
                            '\'' => '\'',
                            _ => return Err(Error::new(ErrorType::BadChar(c), pos))
                        }
                    }
                    pos.extend(&self.pos());
                    if self.next().unwrap() != '\'' {
                        return Err(Error::new(ErrorType::BadChar(c), pos))
                    }
                    tokens.push(Located::new(Token::Char(c), pos));
                },
                '"' => {
                    let mut string = String::new();
                    while let Some(c) = self.next() {
                        if c == '\\' {
                            let c = match self.next().unwrap() {
                                'n' => '\n',
                                't' => '\t',
                                'r' => '\r',
                                '\\' => '\\',
                                '"' => '"',
                                _ => return Err(Error::new(ErrorType::BadChar(c), pos))
                            };
                            string.push(c);
                        } else if c == '"' {
                            pos.extend(&self.pos());
                            break;
                        } else {
                            string.push(c);
                        }
                    }
                    if self.get().is_none() {
                        return Err(Error::new(ErrorType::UnclosedString, pos))
                    }
                    tokens.push(Located::new(Token::String(string), pos));
                }
                _ => if self.has_symbols() {
                    let mut symbol = self.next().unwrap().to_string();
                    while let Some(_) = self.get() {
                        let matches = self.symbols.iter().filter(|s| *s == &symbol || s.starts_with(&symbol)).count();
                        if matches == 0 {
                            symbol.pop();
                            break;
                        } else if matches == 1 {
                            break;
                        } else {
                            symbol.push(self.next().unwrap());
                            pos.extend(&self.pos());
                        }
                    }
                    if self.symbols.iter().any(|s| s == &symbol) {
                        if symbol.len() == 1 {
                            tokens.push(Located::new(Token::Symbol(symbol.chars().next().unwrap()), pos));
                        } else {
                            tokens.push(Located::new(Token::LongSymbol(symbol), pos));
                        }
                    } else {
                        return Err(Error::new(ErrorType::InvalidSymbol(symbol), pos))
                    }
                } else {
                    let c = self.next().unwrap();
                    tokens.push(Located::new(Token::Symbol(c), pos));
                }
            }
        }
        Ok(tokens)
    }
}