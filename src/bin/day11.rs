use std::{mem, ops, str::FromStr};

use aoc2020::read_stdin;

fn main() {
  let input = read_stdin().unwrap();
  let mut grid = input.parse::<Grid>().unwrap();
  let mut next = grid.clone();

  loop {
    let mut n_different = grid.dim.0 * grid.dim.1;
    for p in grid.indices() {
      next[p] = {
        let n_occupied =
          grid.adjacent(p).filter(|&a| grid[a] == Seat::Occupied).count();
        match grid[p] {
          Seat::Empty if n_occupied == 0 => Seat::Occupied,
          Seat::Occupied if n_occupied >= 5 => Seat::Empty,
          s => {
            n_different -= 1;
            s
          }
        }
      };
    }
    if n_different == 0 {
      break;
    }
    mem::swap(&mut grid, &mut next);
  }

  let res = grid.indices().filter(|&p| grid[p] == Seat::Occupied).count();
  println!("{}", res)
}

type Point = (usize, usize);

#[derive(Clone)]
struct Grid {
  dim: Point,
  data: Vec<Seat>,
}

#[derive(Copy, Clone, Eq, PartialEq)]
enum Seat {
  Empty,
  Occupied,
  Blocked,
}

impl FromStr for Grid {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, ()> {
    let width = s.lines().next().ok_or(())?.len();
    let mut dim = (width, 0);

    let mut data = Vec::new();
    for line in s.lines() {
      dim.1 += 1;
      for c in line.chars() {
        data.push(match c {
          'L' => Seat::Empty,
          '#' => Seat::Occupied,
          '.' => Seat::Blocked,
          _ => return Err(()),
        });
      }
    }
    assert_eq!(data.len(), dim.0 * dim.1);
    Ok(Grid { dim, data })
  }
}

impl Grid {
  fn indices(&self) -> impl Iterator<Item = Point> {
    let dim = self.dim;
    (0..dim.0).flat_map(move |x| (0..dim.1).map(move |y| (x, y)))
  }

  fn adjacent(&self, p: Point) -> impl Iterator<Item = Point> + '_ {
    assert!(self.in_bounds(p));

    let directions = (-1isize..=1)
      .flat_map(|dx| (-1isize..=1).map(move |dy| (dx, dy)))
      .filter(|&(x, y)| !(x == 0 && y == 0));
    directions.filter_map(move |d| {
      let mut t = 0isize;
      loop {
        t += 1;
        let a = (
          checked_add_signed(p.0, d.0 * t)?,
          checked_add_signed(p.1, d.1 * t)?,
        );
        if !self.in_bounds(a) {
          return None;
        }
        if self[a] != Seat::Blocked {
          return Some(a);
        }
      }
    })
  }

  fn in_bounds(&self, p: Point) -> bool {
    p.0 < self.dim.0 && p.1 < self.dim.1
  }

  fn to_index(&self, p: Point) -> usize {
    assert!(self.in_bounds(p));
    self.dim.0 * p.1 + p.0
  }
}

impl ops::Index<Point> for Grid {
  type Output = Seat;

  fn index(&self, p: Point) -> &Seat {
    let idx = self.to_index(p);
    &self.data[idx]
  }
}

impl ops::IndexMut<Point> for Grid {
  fn index_mut(&mut self, p: Point) -> &mut Seat {
    let idx = self.to_index(p);
    &mut self.data[idx]
  }
}

fn checked_add_signed(x: usize, d: isize) -> Option<usize> {
  let mut x = x as isize;
  x += d;
  if x < 0 {
    return None;
  }
  Some(x as usize)
}
