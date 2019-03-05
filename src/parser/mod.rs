mod utils;

use self::utils::TokenTests;
use std::iter::Peekable;
use std::str::Chars;

#[derive(Clone, Debug)]
pub enum Token {
  EOF,
  Illegal { value: String },
  Whitespace { value: String },
  Identifier { name: String },
  Comment { text: String },
  True,
  False,
  Match,
  In,
  Asc,
  Desc,
  LeftBrace,
  RightBrace,
  Asterisk,
  Equals,
  NEQ,
  Arrow,
  Rocket,
  Not,
  Pipe,
  QuestionMark,
  Dot,
  DotDot,
  DotDotDot,
  Or,
  And,
  LT,
  LTE,
  GT,
  GTE,
  Minus,
}

pub struct Scanner<'a> {
  input: Peekable<Chars<'a>>,
  pos: u64,
}

impl<'a> Scanner<'a> {
  pub fn new(source: &'a str) -> Scanner {
    Scanner {
      input: source.chars().peekable(),
      pos: 0,
    }
  }

  pub fn pos(&self) -> u64 {
    return self.pos;
  }

  pub fn next(&mut self) -> Token {
    if let Some(s) = self.scan_whitespace() {
      return Token::Whitespace { value: s };
    }
    if let Some(c) = self.peek() {
      if c.is_leading_identifier() {
        return self.scan_identifier();
      }
      if c.is_operator() {
        return self.scan_operator();
      }
      if c == '/' {
        return self.scan_comment();
      }
      if c == '.' {
        return self.scan_dots();
      }
      let t = match c {
        '*' => Token::Asterisk,
        '{' => Token::LeftBrace,
        '}' => Token::RightBrace,
        _ => Token::Illegal {
          value: c.to_string(),
        },
      };
      self.skip();
      return t;
    }
    Token::EOF
  }

  fn scan_dots(&mut self) -> Token {
    let mut count = 0;
    while let Some(c) = self.peek() {
      if c != '.' {
        break;
      }
      self.skip();
      count += 1;
    }
    return match count {
      0 => Token::EOF,
      1 => Token::Dot,
      2 => Token::DotDot,
      3 => Token::DotDotDot,
      _ => Token::Illegal {
        value: ".".repeat(count),
      },
    };
  }

  fn scan_operator(&mut self) -> Token {
    if let Some(c1) = self.read() {
      if c1.is_operator() {
        if let Some(c2) = self.peek() {
          if c2.is_operator() {
            self.skip();
            return match (c1, c2) {
              ('=', '=') => Token::Equals,
              ('|', '|') => Token::Or,
              ('&', '&') => Token::And,
              ('<', '=') => Token::LTE,
              ('>', '=') => Token::GTE,
              ('=', '>') => Token::Rocket,
              ('!', '=') => Token::NEQ,
              ('-', '>') => Token::Arrow,
              _ => Token::Illegal {
                value: format!("{}{}", c1, c2),
              },
            };
          }
        }
        return match c1 {
          '>' => Token::GT,
          '<' => Token::LT,
          '!' => Token::Not,
          '*' => Token::Asterisk,
          '|' => Token::Pipe,
          '?' => Token::QuestionMark,
          '-' => Token::Minus,
          _ => Token::Illegal {
            value: c1.to_string(),
          },
        };
      }
      return Token::Illegal {
        value: c1.to_string(),
      };
    }
    Token::EOF
  }

  fn scan_identifier(&mut self) -> Token {
    let mut buf = String::new();
    while let Some(c) = self.peek() {
      if c.is_identifier() {
        self.skip();
        buf.push(c);
      } else {
        break;
      }
    }
    match buf.as_str() {
      "match" => Token::Match,
      "in" => Token::In,
      "asc" => Token::Asc,
      "desc" => Token::Desc,
      "true" => Token::True,
      "false" => Token::False,
      _ => Token::Identifier { name: buf },
    }
  }

  fn scan_comment(&mut self) -> Token {
    if let Some(c) = self.read() {
      if c != '/' {
        return Token::Illegal {
          value: c.to_string(),
        };
      }
      let mut s = String::new();
      while let Some(c) = self.read() {
        if c == '\n' {
          break;
        }
        s.push(c);
      }
      return Token::Comment { text: s };
    }
    return Token::EOF;
  }

  fn scan_whitespace(&mut self) -> Option<String> {
    let mut s = String::new();
    while let Some(c) = self.peek() {
      if !c.is_whitespace() {
        break;
      }
      s.push(c);
      self.skip();
    }
    if s.len() == 0 {
      return None;
    }
    return Some(s);
  }

  fn skip(&mut self) {
    let _ = self.read();
  }

  fn read(&mut self) -> Option<char> {
    self.pos += 1;
    self.input.next()
  }

  fn peek(&mut self) -> Option<char> {
    if let Some(&c) = self.input.peek() {
      Some(c)
    } else {
      None
    }
  }
}
