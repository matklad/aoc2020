use std::{collections::HashSet, mem};

use aoc2020::read_stdin;

const DIM: usize = 4;
type Coord = [i32; DIM];

fn main() {
  let input = read_stdin().unwrap();

  let mut map: HashSet<Coord> = HashSet::new();
  for (y, line) in input.lines().enumerate() {
    for (x, c) in line.chars().enumerate() {
      match c {
        '.' => (),
        '#' => {
          map.insert([y as i32, x as i32, 0, 0]);
        }
        _ => panic!("{:?}", c),
      }
    }
  }

  let mut next: HashSet<Coord> = HashSet::new();
  let mut workset: HashSet<Coord> = HashSet::new();
  for _ in 0..6 {
    workset.extend(map.iter().copied().flat_map(adj));
    next.extend(workset.drain().filter(|&c| {
      let anb =
        adj(c).filter(|&it| it != c).filter(|it| map.contains(it)).count();
      let active = map.contains(&c);
      active && matches!(anb, 2 | 3) || !active && anb == 3
    }));
    mem::swap(&mut next, &mut map);
    next.clear()
  }
  println!("{}", map.len())
}

fn adj(c: Coord) -> impl Iterator<Item = Coord> {
  DELTAS.iter().copied().map(move |d| {
    let mut res = c;
    for i in 0..DIM {
      res[i] = c[i] + d[i];
    }
    res
  })
}

const DELTAS: [Coord; 81] = {
  let mut res = [[0; DIM]; 81];
  let mut i = 0;
  while i < 81 {
    let mut x = i;
    let mut c = 0;
    while c < DIM {
      res[i][c] = match x % 3 {
        0 => -1,
        1 => 0,
        2 => 1,
        _ => 0 / 0,
      };
      x /= 3;
      c += 1;
    }
    i += 1;
  }
  res
};
