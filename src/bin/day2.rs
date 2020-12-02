use aoc2020::{read_stdin, split_once};

fn main() {
  let input = read_stdin().unwrap();
  let res = input.lines().filter(|&it| is_valid(it)).count();
  println!("{}", res)
}

fn is_valid(line: &str) -> bool {
  // 1-3 b: cdefg
  let (rule, password) = split_once(line, ':').unwrap();
  let password = password.trim();

  let (range, c) = split_once(rule, ' ').unwrap();
  let (lo, hi) = split_once(range, '-').unwrap();
  let lo = lo.parse::<usize>().unwrap() - 1;
  let hi = hi.parse::<usize>().unwrap() - 1;

  assert!(lo <= hi && hi <= password.len());
  password[lo..].starts_with(c) ^ password[hi..].starts_with(c)
}
