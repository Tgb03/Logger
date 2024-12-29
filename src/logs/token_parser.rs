use crate::time::Time;

use super::tokenizer::Token;


pub trait TokenParserT<R> : Into<R> {

  /// get the current result from the parser
  fn into_result(&self) -> &R;

  /// parse on token and return whether or not the parser finished
  fn parse_one_token(&mut self, token_pair: (Time, Token)) -> bool;

  fn parse_continously<I>(&mut self, tokens: I)
  where 
    I: Iterator<Item = (Time, Token)> {
    
    for token in tokens {
      let _ = self.parse_one_token(token);
    }
  }

  fn parse_all_tokens<I, T>(tokens: I, constructor: T) -> R
  where
    I: Iterator<Item = (Time, Token)>,
    T: Into<Self> {

    let mut parser: Self = constructor.into();
    for token in tokens {
      parser.parse_one_token(token);
    }

    parser.into()
  }

  fn parse_all_tokens_default<I>(tokens: I) -> R
  where
    I: Iterator<Item = (Time, Token)>,
    Self: Default {

    Self::parse_all_tokens(tokens, Self::default())
    
  }

}


