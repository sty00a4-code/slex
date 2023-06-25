use crate::error::{Error, ErrorType, Located, Position};
use crate::tokens::Token;

#[derive(Debug, Clone, PartialEq)]
pub struct Line {
    pub ln: usize,
    pub tokens: Vec<Located<Token>>,
    pub indent: usize,
}
impl Line {
    pub fn len(&self) -> usize {
        self.tokens.len()
    }
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
    pub fn pop(&mut self) -> Option<Located<Token>> {
        self.tokens.pop()
    }
    pub fn remove(&mut self, idx: usize) -> Located<Token> {
        self.tokens.remove(idx)
    }
}

pub struct Lexer {
    pub lines: Vec<String>,
    pub symbols: Vec<String>,
    pub keywords: Vec<String>,
    pub idx: usize,
    pub ln: usize,
    pub col: usize,
}
impl Lexer {
    pub fn new(text: String) -> Self {
        Self {
            lines: text.split('\n').map(|s| s.to_string()).collect(),
            symbols: vec![],
            keywords: vec![],
            idx: 0,
            ln: 0,
            col: 0,
        }
    }
    pub fn pos(&self) -> Position {
        Position {
            idx: self.idx..self.idx + 1,
            ln: self.ln..self.ln + 1,
            col: self.col..self.col + 1,
        }
    }
    pub fn advance(&mut self) {
        self.idx += 1;
        self.col += 1;
    }
    pub fn advance_line(&mut self) {
        self.idx += 1;
        self.ln += 1;
        self.col = 0;
    }
    pub fn get(&self) -> Option<char> {
        self.lines.get(self.ln).and_then(|line| {
            line.get(self.idx..self.idx + 1)
                .and_then(|s| s.chars().next())
        })
    }
    pub fn next_char(&mut self) -> Option<char> {
        let c = self.get();
        self.advance();
        c
    }
    pub fn has_symbols(&self) -> bool {
        !self.symbols.is_empty()
    }
    pub fn symbols(mut self, symbols: &[&str]) -> Self {
        self.symbols = symbols.iter().map(|symbol| symbol.to_string()).collect();
        self
    }
    pub fn keywords(mut self, keywords: &[&str]) -> Self {
        self.keywords = keywords.iter().map(|symbol| symbol.to_string()).collect();
        self
    }
    pub fn lex(&mut self) -> Result<Vec<Line>, Error> {
        let mut lines = vec![];
        while self.ln < self.lines.len() {
            let mut indent = 0;
            while let Some(' ' | '\t') = self.get() {
                self.advance();
                indent += 1;
            }
            let mut tokens = vec![];
            while let Some(c) = self.get() {
                let mut pos = self.pos();
                match c {
                    ' ' | '\t' | '\r' => {
                        self.advance();
                    }
                    '0'..='9' => {
                        let mut num = self.next_char().unwrap().to_string();
                        while let Some('0'..='9') = self.get() {
                            pos.extend(&self.pos());
                            num.push(self.next_char().unwrap());
                        }
                        if let Some('.') = self.get() {
                            pos.extend(&self.pos());
                            num.push(self.next_char().unwrap());
                            while let Some('0'..='9') = self.get() {
                                num.push(self.next_char().unwrap());
                                pos.extend(&self.pos());
                            }
                            tokens.push(Located::new(Token::Float(num.parse().unwrap()), pos));
                        } else {
                            tokens.push(Located::new(Token::Int(num.parse().unwrap()), pos));
                        }
                    }
                    'a'..='z' | 'A'..='Z' | '_' => {
                        let mut ident = self.next_char().unwrap().to_string();
                        while let Some('a'..='z') | Some('A'..='Z') | Some('_') | Some('0'..='9') =
                            self.get()
                        {
                            pos.extend(&self.pos());
                            ident.push(self.next_char().unwrap());
                        }
                        tokens.push(Located::new(
                            if self.keywords.contains(&ident) {
                                Token::Keyword(ident)
                            } else {
                                Token::Ident(ident)
                            },
                            pos,
                        ));
                    }
                    '\'' => {
                        let mut c = self.next_char().unwrap();
                        if c == '\\' {
                            c = match self.next_char().unwrap() {
                                'n' => '\n',
                                't' => '\t',
                                'r' => '\r',
                                '\\' => '\\',
                                '\'' => '\'',
                                _ => return Err(Error::new(ErrorType::BadChar(c), pos)),
                            }
                        }
                        pos.extend(&self.pos());
                        if self.next_char().unwrap() != '\'' {
                            return Err(Error::new(ErrorType::BadChar(c), pos));
                        }
                        tokens.push(Located::new(Token::Char(c), pos));
                    }
                    '"' => {
                        let mut string = String::new();
                        while let Some(c) = self.next_char() {
                            if c == '\\' {
                                let c = match self.next_char().unwrap() {
                                    'n' => '\n',
                                    't' => '\t',
                                    'r' => '\r',
                                    '\\' => '\\',
                                    '"' => '"',
                                    _ => return Err(Error::new(ErrorType::BadChar(c), pos)),
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
                            return Err(Error::new(ErrorType::UnclosedString, pos));
                        }
                        tokens.push(Located::new(Token::String(string), pos));
                    }
                    _ => {
                        if self.has_symbols() {
                            let mut symbol = self.next_char().unwrap().to_string();
                            while self.get().is_some() {
                                let matches = self
                                    .symbols
                                    .iter()
                                    .filter(|s| *s == &symbol || s.starts_with(&symbol))
                                    .count();
                                if matches == 0 {
                                    symbol.pop();
                                    break;
                                } else if matches == 1 {
                                    break;
                                } else {
                                    symbol.push(self.next_char().unwrap());
                                    pos.extend(&self.pos());
                                }
                            }
                            if self.symbols.iter().any(|s| s == &symbol) {
                                if symbol.len() == 1 {
                                    tokens.push(Located::new(
                                        Token::Symbol(symbol.chars().next().unwrap()),
                                        pos,
                                    ));
                                } else {
                                    tokens.push(Located::new(Token::LongSymbol(symbol), pos));
                                }
                            } else {
                                return Err(Error::new(ErrorType::InvalidSymbol(symbol), pos));
                            }
                        } else {
                            let c = self.next_char().unwrap();
                            tokens.push(Located::new(Token::Symbol(c), pos));
                        }
                    }
                }
            }
            lines.push(Line {
                ln: self.ln,
                tokens,
                indent,
            });
            self.advance_line()
        }
        Ok(lines)
    }
}
