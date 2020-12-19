use std::iter;

use aoc2020::read_stdin;
use iter::Peekable;

fn main() {
  let input = read_stdin().unwrap();
  let mut total = 0u64;
  for line in input.lines() {
    total += expr(line);
  }
  println!("{}", total)
}

fn expr(input: &str) -> u64 {
  let mut lexer = tokenize(input).peekable();
  expr_bp(&mut lexer, 0)
}

fn expr_bp(
  lexer: &mut Peekable<impl Iterator<Item = Token>>,
  min_bp: u8,
) -> u64 {
  let mut lhs = match lexer.next() {
    Some(Token::Num(it)) => it,
    Some(Token::LParen) => {
      let lhs = expr_bp(lexer, 0);
      assert_eq!(lexer.next(), Some(Token::RParen));
      lhs
    }
    t => panic!("bad token: {:?}", t),
  };

  loop {
    let op = match lexer.peek() {
      None => break,
      Some(op) => *op,
    };

    if let Some((l_bp, r_bp)) = infix_binding_power(op) {
      if l_bp < min_bp {
        break;
      }

      lexer.next();
      let rhs = expr_bp(lexer, r_bp);

      lhs = match op {
        Token::Op(Op::Add) => lhs + rhs,
        Token::Op(Op::Mul) => lhs * rhs,
        _ => panic!(),
      };
      continue;
    }

    break;
  }

  lhs
}

fn infix_binding_power(op: Token) -> Option<(u8, u8)> {
  let res = match op {
    Token::Op(Op::Add) => (3, 4),
    Token::Op(Op::Mul) => (1, 2),
    _ => return None,
  };
  Some(res)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Token {
  LParen,
  RParen,
  Op(Op),
  Num(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Op {
  Add,
  Mul,
}

fn tokenize(s: &str) -> impl Iterator<Item = Token> + '_ {
  let mut rest = s;
  iter::from_fn(move || {
    rest = rest.trim_start();
    let c = rest.chars().next()?;
    let tok = match c {
      '(' => Token::LParen,
      ')' => Token::RParen,
      '+' => Token::Op(Op::Add),
      '*' => Token::Op(Op::Mul),
      _ => {
        let l = rest.splitn(2, |it| !matches!(it, '0'..='9')).next().unwrap();
        rest = &rest[l.len()..];
        return Some(Token::Num(l.parse().unwrap()));
      }
    };
    rest = &rest[1..];
    Some(tok)
  })
}
