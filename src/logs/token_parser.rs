
use crate::run::time::Time;

use super::token::Token;


/// generic trait for a parser
pub trait TokenParserT<R> : Into<R> {

  /// get the current result from the parser
  fn into_result(&self) -> &R;

  /// get the current result mutable
  fn into_result_mut(&mut self) -> &mut R;

  /// parse on token and return whether or not the parser finished
  fn parse_one_token(&mut self, token_pair: (Time, Token)) -> bool;

  /// parse tokens without consuming the self
  fn parse_continously<I>(&mut self, tokens: I)
  where 
    I: Iterator<Item = (Time, Token)> {
    
    for token in tokens {
      let _ = self.parse_one_token(token);
    }
  }

  /// parse all tokens in one go and get the result
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

  /// parse all tokens in one go and get the result
  /// 
  /// uses a default constructor for the parser.
  fn parse_all_tokens_default<I>(tokens: I) -> R
  where
    I: Iterator<Item = (Time, Token)>,
    Self: Default {

    Self::parse_all_tokens(tokens, Self::default())
    
  }

}


