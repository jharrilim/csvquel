use crate::{syntax::TokenKind, lexer::Token};

pub struct Source<'input, 'tokens> {
    pub tokens: &'tokens [Token<'input>],
    pub cursor: usize,
}

// We're basically adding this to get peek functionality for the lexer

impl <'input, 'tokens> Source<'input, 'tokens> {
    pub fn new(tokens: &'tokens [Token<'input>]) -> Self {
        Self {
            tokens,
            cursor: 0,
        }
    }

    pub(super) fn next_token(&mut self) -> Option<&'tokens Token<'input>> {
      self.eat_trivia();
  
      let token = self.tokens.get(self.cursor)?;
      self.cursor += 1;
  
      Some(token)
    }
  
    pub(super) fn peek_kind(&mut self) -> Option<TokenKind> {
      self.eat_trivia();
      self.peek_kind_raw()
    }
  
    pub fn is_end(&self) -> bool {
      self.peek_token().is_none()
    }

    fn peek_kind_raw(&self) -> Option<TokenKind> {
      self.peek_token().map(|Token { kind, .. }| *kind)
    }
  
    fn eat_trivia(&mut self) {
      while self.at_trivia() {
        self.cursor += 1;
      }
    }
  
    fn at_trivia(&self) -> bool {
      self.peek_kind_raw().map_or(false, TokenKind::is_trivia)
    }
  
    pub fn peek_token(&self) -> Option<&Token> {
      self.tokens.get(self.cursor)
    }
}
