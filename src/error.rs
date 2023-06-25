use std::{
    error,
    fmt::{Debug, Display},
    ops::Range,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Position {
    pub idx: Range<usize>,
    pub ln: Range<usize>,
    pub col: Range<usize>,
}
impl Position {
    pub fn new(idx: Range<usize>, ln: Range<usize>, col: Range<usize>) -> Self {
        Self { idx, ln, col }
    }
    pub fn extend(&mut self, other: &Self) {
        self.idx.end = other.idx.end;
        self.ln.end = other.ln.end;
        self.col.end = other.col.end;
    }
}
pub struct Located<T> {
    pub value: T,
    pub pos: Position,
}
impl<T> Located<T> {
    pub fn new(value: T, pos: Position) -> Self {
        Self { value, pos }
    }
}
impl<T: Clone> Clone for Located<T> {
    fn clone(&self) -> Self {
        Self {
            value: self.value.clone(),
            pos: self.pos.clone(),
        }
    }
}
impl<T: Display> Display for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.value)
    }
}
impl<T: Debug> Debug for Located<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self.value)
    }
}
impl<T: PartialEq> PartialEq for Located<T> {
    fn eq(&self, other: &Self) -> bool {
        self.value == other.value
    }
}
impl<T: Eq> Eq for Located<T> {}
impl<T> std::ops::Deref for Located<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    BadChar(char),
    UnclosedString,
    InvalidSymbol(String),
}
#[derive(Debug, Clone, PartialEq)]
pub struct Error {
    pub error_type: ErrorType,
    pub pos: Position,
}
impl Error {
    pub fn new(error_type: ErrorType, pos: Position) -> Self {
        Self { error_type, pos }
    }
}
impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.error_type {
            ErrorType::BadChar(c) => write!(
                f,
                "bad character '{}' at {}, (ln: {}, col: {})",
                c, self.pos.idx.start, self.pos.ln.start, self.pos.col.start
            ),
            ErrorType::UnclosedString => write!(
                f,
                "unclosed string at {}, (ln: {}, col: {})",
                self.pos.idx.start, self.pos.ln.start, self.pos.col.start
            ),
            ErrorType::InvalidSymbol(s) => write!(
                f,
                "invalid symbol '{}' at {}, (ln: {}, col: {})",
                s, self.pos.idx.start, self.pos.ln.start, self.pos.col.start
            ),
        }
    }
}
impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        None
    }
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        None
    }
    fn description(&self) -> &str {
        match &self.error_type {
            ErrorType::BadChar(_) => "bad character",
            ErrorType::UnclosedString => "unclosed string",
            ErrorType::InvalidSymbol(_) => "invalid symbol",
        }
    }
}
