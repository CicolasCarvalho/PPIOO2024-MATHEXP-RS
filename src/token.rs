type ArgsNum = u8;

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Token {
  NullOps,
  Sum(ArgsNum),
  Sub(ArgsNum),
  Mul,
  Div,
  ParenOpen,
  ParenClose,
  Literal(i64),
}

impl Token {
  pub fn is_operator(&self) -> bool {
    match self {
      Token::Sum(_) |
      Token::Sub(_) |
      Token::Mul    |
      Token::Div => true,
      _ => false
    }
  }

  pub fn is_literal(&self) -> bool {
    match self {
      Token::Literal(_) => true,
      _ => false
    }
  }

  pub fn check_precedence(&self, op: &Token) -> i8 {
    let self_priority: i8 = match self {
      Token::Sum(2) | Token::Sub(2) => 0,
      Token::Mul    | Token::Div    => 1,
      Token::Sum(1) | Token::Sub(1) => 2,
      _                             => -1,
    };

    let token_priority: i8 = match op {
      Token::Sum(2) | Token::Sub(2) => 0,
      Token::Mul    | Token::Div    => 1,
      Token::Sum(1) | Token::Sub(1) => 2,
      _                             => -1,
    };

    token_priority - self_priority
  }

  pub fn to_string(&self) -> String {
    match self {
      Token::NullOps => String::from("null"),
      Token::Sum(2) => String::from("+"),
      Token::Sub(2) => String::from("-"),
      Token::Sum(1) => String::from("+"),
      Token::Sub(1) => String::from("-"),
      Token::Mul => String::from("*"),
      Token::Div => String::from("/"),
      Token::ParenOpen => String::from("("),
      Token::ParenClose => String::from(")"),
      Token::Literal(num) => num.to_string(),
      _ => String::from("INVALID")
    }
  }
}
