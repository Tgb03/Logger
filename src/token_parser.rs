use crate::{logs::tokenizer::Token, parse_files::file_parse::TokenParserResult, time::Time, timed_run_parser::TimedRunParser};

pub struct TokenParser {

  results: TokenParserResult,

  timed_run_parser: TimedRunParser,

}

impl TokenParser {

  fn new() -> TokenParser {
    TokenParser {
      results: Default::default(),
      timed_run_parser: TimedRunParser::new("".to_string(), Time::new())
    }
  }

  pub fn parse_tokens(tokens: Vec<(Time, Token)>) -> TokenParserResult  {
    let mut token_parser = TokenParser::new();
    let mut result = TokenParserResult::default();

    let mut token_iter = tokens.into_iter();
    while let Some((time, token)) = token_iter.next() {
      match token {
        Token::SelectExpedition(level_name) => {
          token_parser.timed_run_parser.set_name(level_name);
        },
        Token::GameStarted => {
          token_parser.timed_run_parser.set_start_time(time);
          result.extend_run(token_parser.timed_run_parser.get_run(&mut token_iter));
        },
        Token::GameEndAbort | Token::PlayerDroppedInLevel(_) => { /* Ignored in case instant reset. */}
        Token::LogFileEnd => {
          return result;
        },
        _ => panic!("{:?} token is not handled properly by Token Parser.", token)
      }
    }

    result
  }

}


