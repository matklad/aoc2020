use std::ops;

use aoc2020::read_stdin;

fn main() {
  let input = read_stdin().unwrap();

  let mut pos = (0, 0);
  let mut wp = Rot(10, 1);

  for line in input.lines() {
    let command = line.chars().next().unwrap();
    let arg = line[1..].parse::<i32>().unwrap();
    let r = match command {
      'E' => Rot(1, 0),
      'N' => Rot(0, 1),
      'W' => Rot(-1, 0),
      'S' => Rot(0, -1),
      'L' => Rot(0, 1),
      'R' => Rot(0, -1),
      'F' => {
        pos.0 += wp.0 * arg;
        pos.1 += wp.1 * arg;
        continue;
      }
      _ => unreachable!(),
    };
    if "ENSW".contains(command) {
      wp.0 += r.0 * arg;
      wp.1 += r.1 * arg;
    } else {
      assert!("LR".contains(command) && arg % 90 == 0);
      for _ in 0..arg / 90 {
        wp = wp * r;
      }
    }
  }
  println!("{}", pos.0.abs() + pos.1.abs())
}

#[derive(Clone, Copy)]
struct Rot(i32, i32);

impl ops::Mul<Rot> for Rot {
  type Output = Rot;

  fn mul(self, rhs: Rot) -> Rot {
    Rot(self.0 * rhs.0 - self.1 * rhs.1, self.0 * rhs.1 + self.1 * rhs.0)
  }
}
