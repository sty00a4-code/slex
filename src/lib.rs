#[cfg(test)]
mod tests;

pub mod error;
pub mod indent_lexer;
pub mod lexer;
pub mod tokens;

pub fn lex(input: String) -> Result<Vec<error::Located<tokens::Token>>, error::Error> {
    lexer::Lexer::new(input).lex()
}
pub fn lex_indent(input: String) -> Result<Vec<indent_lexer::Line>, error::Error> {
    indent_lexer::Lexer::new(input).lex()
}
