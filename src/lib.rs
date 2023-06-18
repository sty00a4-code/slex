#[cfg(test)]
mod tests;

pub mod error;
pub mod lexer;
pub mod indent_lexer;

pub fn lex(input: String) -> Result<Vec<error::Located<lexer::Token>>, error::Error> {
    lexer::Lexer::new(input).lex()
}