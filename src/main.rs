use std::env;

use lexer::TokenExp;
use parser::ExpressionTree;

pub mod token;
pub mod lexer;
pub mod parser;

fn main() {
  let mut args = env::args().skip(1);
  let exp_arg = args.next().expect("Esperado um argumento contendo a expressão");

  let token_exp = TokenExp::from_str(exp_arg.as_str());
  let mut tree = ExpressionTree::from_exp(&token_exp);

  // tree.root.borrow().display(0);

  println!("{}", tree.eval());
}
