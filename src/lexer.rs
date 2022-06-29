use logos::Logos;
use crate::syntax::TokenKind;

pub struct Lexer<'source> {
  inner: logos::Lexer<'source, TokenKind>,
}

impl<'source> Lexer<'source> {
  pub fn new(input: &'source str) -> Self {
    Self {
      inner: TokenKind::lexer(input),
    }
  }
}

// Adding this so it can easily be collected into source
impl<'a> Iterator for Lexer<'a> {
  type Item = Token<'a>;

  fn next(&mut self) -> Option<Self::Item> {
    let kind = self.inner.next()?;
    let text = self.inner.slice();

    Some(Self::Item { kind, text })
  }
}

#[derive(Debug, PartialEq)]
pub struct Token<'a> {
  pub kind: TokenKind,
  pub text: &'a str,
}
