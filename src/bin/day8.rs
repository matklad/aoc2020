use std::mem;

use aoc2020::{read_stdin, split_once};

fn main() {
  let input = read_stdin().unwrap();
  let mut code: Vec<(OpCode, i32)> = input
    .lines()
    .map(|line| {
      let (op, arg) = split_once(line, ' ').unwrap();
      let op = match op {
        "acc" => OpCode::Acc,
        "jmp" => OpCode::Jmp,
        "nop" => OpCode::Nop,
        _ => panic!("bad opcode: {:?}", op),
      };
      let arg: i32 = arg.parse().unwrap();
      (op, arg)
    })
    .collect();

  let mut visited = vec![false; code.len()];
  for i in 0..code.len() {
    let rep = match code[i].0 {
      OpCode::Acc => continue,
      OpCode::Jmp => OpCode::Nop,
      OpCode::Nop => OpCode::Jmp,
    };
    let prev = mem::replace(&mut code[i].0, rep);
    if let Ok(acc) = eval(&code, &mut visited) {
      println!("{}", acc);
      return;
    }
    code[i].0 = prev;
  }
  panic!()
}

#[derive(Copy, Clone)]
enum OpCode {
  Acc,
  Jmp,
  Nop,
}

fn eval(code: &[(OpCode, i32)], visited: &mut [bool]) -> Result<i32, ()> {
  visited.iter_mut().for_each(|it| *it = false);
  let mut ip: i32 = 0;
  let mut acc: i32 = 0;
  loop {
    let idx = ip as usize;
    if idx == code.len() {
      return Ok(acc);
    }
    if mem::replace(&mut visited[idx], true) {
      return Err(());
    }
    let (op, arg) = code[idx];
    let mut d = 1;
    match op {
      OpCode::Acc => acc += arg,
      OpCode::Nop => (),
      OpCode::Jmp => d = arg,
    }
    ip += d;
  }
}
