use std::{cell::RefCell, rc::Rc};

use crate::{lexer::{RPNExp, TokenExp}, token::Token};

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
pub struct TreeNode {
  pub token: Token,
  pub left: Option<TreeNodeRef>,
  pub right: Option<TreeNodeRef>
}

pub struct ExpressionTree {
  pub root: TreeNodeRef
}

impl TreeNode {
  pub fn from_exp(exp: &TokenExp) -> TreeNodeRef {
    // println!("Expression: ");
    // exp.display();

    let tokens = RPNExp::from_exp(exp);
    // println!("RPN: ");
    // tokens.display();

    let root: TreeNodeRef = Self::from_rpn_exp(&tokens);

    root
  }

  pub fn from_rpn_exp(exp: &RPNExp) -> TreeNodeRef {
    let mut i: i32 = exp.tokens.len() as i32 - 1;
    let root = tree_build(exp, &mut i)
      .expect("EMPTY EXPRESSION");

    root
  }

  pub fn display(&self, level: u32) {
    let pad_fn = |l: u32| {
      for _ in 0..l as i32 - 1 {
        print!(" | ");
      }

      if l > 0 { print!(" |-"); }
    };

    let value = self.token.to_string();

    pad_fn(level);
    print!("({})\n", value);

    let left_ref = self.left.clone();
    let right_ref = self.right.clone();

    if let Some(left) = left_ref {
      left.borrow().display(level + 1);
    } else {
      pad_fn(level + 1);
      print!("(NULL)\n");
    }

    if let Some(right) = right_ref {
      right.borrow().display(level + 1);
    } else {
      pad_fn(level + 1);
      print!("(NULL)\n");
    }
  }

  pub fn eval(&self) -> i64 {
    let left_ref = self.left.clone();
    let right_ref = self.right.clone();

    let right_token: Option<Token> = right_ref.clone().and_then(|rr| {
      Some(rr.borrow().token)
    });

    match self.token {
      Token::NullOps => return 0,
      Token::Literal(num) => return num,
      Token::Sum(1) => {
        match right_token {
          Some(Token::Literal(num)) => return num,
          Some(_) => {
            return right_ref.unwrap().borrow().eval();
          }
          None => { panic!("RIGHT OPERAND DOES NOT EXIST") }
        }
      },
      Token::Sub(1) => {
        match right_token {
          Some(Token::Literal(num)) => return -num,
          Some(_) => {
            return -right_ref.unwrap().borrow().eval();
          }
          None => { panic!("RIGHT OPERAND DOES NOT EXIST") }
        }
      },
      _ => {}
    }

    let left_token: Option<Token> = left_ref.clone().and_then(|rr| {
      Some(rr.borrow().token)
    });

    let left_value = match left_token {
      Some(Token::Literal(num)) => num,
      Some(_) => left_ref.unwrap().borrow().eval(),
      _ => panic!("LEFT VALUE IS NONE")
    };

    let right_value = match right_token {
      Some(Token::Literal(num)) => num,
      Some(_) => right_ref.unwrap().borrow().eval(),
      _ => panic!("RIGHT VALUE IS NONE")
    };

    match self.token {
      Token::Sum(2) => left_value + right_value,
      Token::Sub(2) => left_value - right_value,
      Token::Mul    => left_value * right_value,
      Token::Div    => left_value / right_value,
      _ => panic!("INVALID TOKEN TYPE")
    }
  }

  pub fn print_exp(&self) {
    if let Some(left_ref) = self.left.clone() {
      left_ref.borrow().print_exp();
    }

    print!("{} ", self.token.to_string());

    if let Some(right_ref) = self.right.clone() {
      right_ref.borrow().print_exp();
    }
  }
}

impl ExpressionTree {
  pub fn from_exp(exp: &TokenExp) -> Self {
    ExpressionTree {
        root: TreeNode::from_exp(exp),
    }
  }

  pub fn display(&self) {
    let self_ref = self.root.clone();
    self_ref.borrow().display(0);
  }

  pub fn eval(&mut self) -> i64 {
    let root = self.root.clone();
    let mut token = root.borrow().token;

    while !token.is_literal() {
      eval_step(root.clone());

      root.borrow().print_exp();
      token = root.borrow().token;
      print!("\n");
    }

    match token {
      Token::Literal(num) => { return num; },
      _ => { panic!("INVALID FINAL TOKEN!"); }
    }
  }
}

fn tree_build(exp: &RPNExp, i: &mut i32) -> Option<TreeNodeRef> {
  if *i < 0 { return None; }

  let actual = exp.tokens[*i as usize];

  let mut left_node: Option<TreeNodeRef> = None;
  let mut right_node: Option<TreeNodeRef> = None;

  if !actual.is_literal() {
    let mut right = *i - 1;
    right_node = tree_build(exp, &mut right);

    match actual {
      Token::Sum(1) | Token::Sub(1) => {
        *i = right;
      }
      _ => {
        let mut left = right - 1;
        left_node = tree_build(exp, &mut left);

        *i = left;

        if left_node.is_none() && right_node.is_none() {
          panic!("MALFORMED EQUATION");
        }
      }
    }
  }

  Some(Rc::new(RefCell::new(TreeNode {
    token: actual,
    left: left_node,
    right: right_node,
  })))
}

fn eval_step(tree_ref: TreeNodeRef) {
  let mut tree = tree_ref.borrow_mut();
  let left_ref = tree.left.clone();
  let right_ref = tree.right.clone();

  if tree.token.is_literal() {
    return;
  }

  let right_token = right_ref.clone().expect(
    "RIGHT OPERAND DOES NOT EXIST"
  ).borrow().token;

  match tree.token {
    Token::Sum(1) | Token::Sub(1) => {
      if right_token.is_literal() {
        let result: i64 = tree.eval();

        tree.token = Token::Literal(result);
        tree.right = None;
        return;
      } else {
        eval_step(right_ref.clone().unwrap());
        return;
      }
    },
    _ => {}
  }

  let left_token = left_ref.clone().expect(
    "LEFT OPERAND DOES NOT EXIST"
  ).borrow().token;

  if left_token.is_literal() && right_token.is_literal() {
    let result: i64 = tree.eval();

    tree.token = Token::Literal(result);
    tree.left = None;
    tree.right = None;
  } else if !left_token.is_literal() {
    eval_step(left_ref.clone().unwrap());
  } else if !right_token.is_literal() {
    eval_step(right_ref.clone().unwrap());
  }
}