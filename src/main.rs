use std::{env, time::Instant, u32};

use lexer::TokenExp;
use parser::ExpressionTree;

pub mod token;
pub mod lexer;
pub mod parser;

fn main() {
  let mut args = env::args().skip(1);
  let exp_arg = args.next()
    .expect("Esperado um argumento contendo a expressão\n./main <expressão> [-b <numero>]");

  let is_benchmark = args.next().and_then(|s| {
    Some(s == String::from("-b"))
  }).unwrap_or(false);

  let bench_size = if is_benchmark {
    args.next()
      .expect("Esperado número de execuçẽos")
      .parse::<u32>()
      .expect("Número de execuções deve ser um número natural")
  } else {
    1
  };

  if is_benchmark {
    println!("running {} tests:", bench_size);
  }

  let before = Instant::now();

  for _ in 0..bench_size {
    let token_exp = TokenExp::from_str(exp_arg.as_str());
    let mut tree = ExpressionTree::from_exp(&token_exp);

    if !is_benchmark {
      println!("{}", tree.eval(is_benchmark));
    }
  }

  // tree.root.borrow().display(0);

  println!("Elapsed time: {:.4?}", before.elapsed());
}
