use std::collections::{HashMap};

use aoc2020::read_stdin;

fn main() {
  let input = read_stdin().unwrap();
  let input = input
    .trim()
    .split(",")
    .map(|it| it.parse::<usize>().unwrap())
    .collect::<Vec<_>>();

  let n = 30_000_000;
  let mut xs = Vec::with_capacity(n);
  let mut prev: HashMap<usize, (usize, Option<usize>)> = HashMap::new();
  let mut input = input.into_iter();

  while xs.len() < n {
    let x = match input.next() {
      Some(it) => it,
      None => {
        let last = *xs.last().unwrap();
        let (t1, t2) = prev[&last];
        match t2 {
          None => 0,
          Some(t2) => t1 - t2,
        }
      }
    };

    let t1 = xs.len();
    let t2 = prev.get(&x).map_or(None, |(t1, _t2)| Some(*t1));
    let val = (t1, t2);
    prev.insert(x, val);
    xs.push(x);
  }
  println!("{}", xs.last().unwrap());
}
