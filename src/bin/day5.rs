use aoc2020::read_stdin;

fn main() {
  let input = read_stdin().unwrap();
  let mut seats = input.lines().map(decode).collect::<Vec<_>>();
  seats.sort();
  for (&a, &b) in seats.iter().zip(seats.iter().skip(1)) {
    if a + 1 != b {
      println!("{} {} {}", a, a + 1, b);
    }
  }
}

fn decode(bsp: &str) -> u32 {
  let (rbsp, cbsp) = bsp.split_at(7);
  let row = acc(rbsp.chars().rev().map(|it| it == 'B'));
  let col = acc(cbsp.chars().rev().map(|it| it == 'R'));
  row * 8 + col
}

fn acc(number: impl Iterator<Item = bool>) -> u32 {
  let mut res = 0u32;
  let mut mul = 1u32;
  for d in number {
    if d {
      res += mul
    }
    mul *= 2
  }
  res
}

#[test]
fn smoke_test() {
  assert_eq!(decode("FFFBBBFRRR"), 119);
}
