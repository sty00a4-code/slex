#[cfg(test)]
mod tests;

pub mod error;
pub mod indent_lexer;
pub mod lexer;

pub fn lex(input: String) -> Result<Vec<error::Located<lexer::Token>>, error::Error> {
    lexer::Lexer::new(input).lex()
}
