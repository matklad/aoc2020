use std::collections::HashMap;

use aoc2020::{read_stdin, Interner};

fn main() {
  let input = read_stdin().unwrap();

  let mut g = Graph::default();
  for mut line in input.lines() {
    // light red
    let u = color(&mut line);
    let u = g.add_node(u);
    // bags contain
    word(&mut line);
    word(&mut line);

    loop {
      match word(&mut line) {
        "no" | "" => break,
        // 1
        n => {
          let n = n.parse::<u64>().unwrap();
          // bright white
          let v = color(&mut line);
          let v = g.add_node(v);

          g.add_edge(u, v, n);
          // bag, 2 muted yellow bags.
          word(&mut line);
        }
      }
    }
  }

  let start = g.add_node("shiny gold");
  let res = g.size(start) - 1;
  println!("{}", res);
}

fn color<'a>(s: &mut &'a str) -> &'a str {
  let orig = *s;
  word(s);
  word(s);
  orig[..orig.len() - s.len()].trim()
}

fn word<'a>(s: &mut &'a str) -> &'a str {
  let idx =
    s.char_indices().find(|&(_, c)| c == ' ').map_or(s.len(), |(idx, _c)| idx);
  let (res, rest) = s.split_at(idx);
  *s = rest.trim();
  res.trim()
}

#[derive(Default)]
struct Graph {
  labels: Interner,
  nbs: Vec<HashMap<u32, u64>>,
}

impl Graph {
  fn add_node(&mut self, label: &str) -> u32 {
    let (res, new) = self.labels.get_or_intern(label);
    if new {
      self.nbs.push(HashMap::new())
    }
    res
  }
  fn add_edge(&mut self, from: u32, to: u32, val: u64) {
    self.nbs[from as usize].insert(to, val);
  }

  fn size(&self, u: u32) -> u64 {
    1 + self.nbs[u as usize]
      .iter()
      .map(|(&v, &s)| s * self.size(v))
      .sum::<u64>()
  }
}
