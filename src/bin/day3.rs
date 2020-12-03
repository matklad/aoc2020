use std::str::FromStr;

use aoc2020::read_stdin;

struct Grid {
  dim: (usize, usize),
  pat: Vec<bool>,
}

impl FromStr for Grid {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, ()> {
    let x_len = s.lines().next().ok_or(())?.len();
    let mut y_len = 0;
    let mut pat = Vec::new();

    for line in s.lines() {
      if line.len() != x_len {
        return Err(());
      }
      y_len += 1;
      for c in line.chars() {
        pat.push(match c {
          '.' => false,
          '#' => true,
          _ => return Err(()),
        })
      }
    }

    assert_eq!(pat.len(), x_len * y_len);
    Ok(Grid { dim: (x_len, y_len), pat })
  }
}

impl Grid {
  fn at(&self, (px, py): (usize, usize)) -> bool {
    let px = px % self.dim.0;
    assert!(py < self.dim.1);
    let idx = py * self.dim.0 + px;
    self.pat[idx]
  }
}

fn trees_on_slope(grid: &Grid, slope: (usize, usize)) -> usize {
  let mut n_trees = 0usize;
  let mut pos = (0, 0);
  loop {
    if grid.at(pos) {
      n_trees += 1;
    }
    if pos.1 == grid.dim.1 - 1 {
      break;
    }
    pos = (pos.0 + slope.0, pos.1 + slope.1);
  }
  n_trees
}

fn main() {
  let input = read_stdin().unwrap();
  let grid = input.parse::<Grid>().unwrap();

  let slopes = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

  let mut res = 1;
  for &slope in slopes.iter() {
    res *= trees_on_slope(&grid, slope);
  }
  println!("{}", res);
}
