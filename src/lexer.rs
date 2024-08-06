use std::collections::VecDeque;

use crate::token::Token;

pub struct TokenExp {
  tokens: Vec<Token>,
}

impl TokenExp {
  pub fn from_str(str: &str) -> Self {
    let mut token_exp: TokenExp = TokenExp {
      tokens: Vec::<Token>::new()
    };

    let mut expression = String::from(str);
    let mut last_token = Token::NullOps;

    while let Some(str) = TokenExp::shift_str_token(&mut expression, last_token) {
      match str.as_str() {
        "-" => {
          if last_token.is_operator() ||
              last_token == Token::ParenOpen ||
              last_token == Token::NullOps {

            token_exp.tokens.push(Token::Sub(1));
            last_token = Token::Sub(1);

            continue;
          }

          token_exp.tokens.push(Token::Sub(2));
          last_token = Token::Sub(2);
        },
        "+" => {
          if last_token.is_operator() ||
              last_token == Token::ParenOpen ||
              last_token == Token::NullOps {

            token_exp.tokens.push(Token::Sum(1));
            last_token = Token::Sum(1);

            continue;
          }

          token_exp.tokens.push(Token::Sum(2));
          last_token = Token::Sum(2);
        },
        "*" => {
          token_exp.tokens.push(Token::Mul);
          last_token = Token::Mul;
        },
        "/" => {
          token_exp.tokens.push(Token::Div);
          last_token = Token::Div;
        },
        "(" => {
          token_exp.tokens.push(Token::ParenOpen);
          last_token = Token::ParenOpen;
        },
        ")" => {
          token_exp.tokens.push(Token::ParenClose);
          last_token = Token::ParenClose;
        }
        _ => {
          let num = str.parse::<i64>()
            .expect(format!("UNKNOWN TOKEN: {}", str).as_str());

          token_exp.tokens.push(Token::Literal(num));
          last_token = Token::Literal(num);
        }
      }
    }

    token_exp
  }

  fn shift_str_token(expression: &mut String, last_token: Token) -> Option<String> {
    while expression.starts_with(' ') {
      expression.remove(0);
    }

    let mut trimmed_str: String = String::new();

    while let Some(c) = expression.chars().next() {
      match c {
        '(' | ')' | '+' | '-' | '/' | '*' => {
          if trimmed_str.len() > 0 {
            break;
          }
        },
        ' ' => { break; }
        '0' ..= '9' => {}
        _ => { panic!("INVALID SYNTAX: '{}'", c) },
      }

      expression.drain(..1);
      trimmed_str.push(c);

      if c == '+' || c == '-' || c == '(' || c == ')' { break; }

      if c == '*' || c == '/' {
        if last_token.is_operator() {
          panic!("INVALID SYNTAX: '{}'", c);
        }
        break;
      }
    }

    if trimmed_str.is_empty() {
      return None;
    }

    Some(trimmed_str)
  }

  pub fn display(&self) {
    for token in &self.tokens {
      print!("{} | ", token.to_string());
    }
    print!("\n");
  }
}

pub struct RPNExp {
  pub tokens: Vec<Token>,
}

impl RPNExp {
  pub fn from_exp(exp: &TokenExp) -> Self {
    let mut operator_stack = VecDeque::<Token>::new();
    let mut output_queue = VecDeque::<Token>::new();

    for token in exp.tokens.iter() {
      if token.is_literal() {
        output_queue.push_back(*token);
      } else if token.is_operator() {
        while let Some(op) = operator_stack.back() {
          if token.check_precedence(op) < 0 { break; }

          let greater_token: Token = operator_stack.pop_back().unwrap();

          output_queue.push_back(greater_token);
        }

        operator_stack.push_back(*token);
      } else if *token == Token::ParenOpen {
        operator_stack.push_back(*token);
      } else if *token == Token::ParenClose {
        Self::flush_stack_to_queue(&mut operator_stack, &mut output_queue, |t| {
          *t != Token::ParenOpen
        });

        operator_stack.pop_back();
      }
    }

    Self::flush_stack_to_queue(&mut operator_stack, &mut output_queue, |_| { true });

    let mut rpn = RPNExp {
      tokens: Vec::<Token>::new()
    };

    Self::flush_queue_to_rpn_exp(&mut output_queue, &mut rpn);

    rpn
  }

  fn flush_stack_to_queue<F>(stack: &mut VecDeque<Token>, queue: &mut VecDeque<Token>, cond: F)
  where
    F: Fn(&Token) -> bool
  {
    while let Some(top) = stack.back() {
      if !cond(top) { break; }

      let greater_token: Token = stack.pop_back().unwrap();

      queue.push_back(greater_token);
    }
  }

  fn flush_queue_to_rpn_exp(queue: &mut VecDeque<Token>, rpn: &mut RPNExp) {
    while let Some(token) = queue.pop_front() {
      rpn.tokens.push(token);
    }
  }

  pub fn display(&self) {
    for token in &self.tokens {
      print!("{} | ", token.to_string());
    }
    print!("\n");
  }
}