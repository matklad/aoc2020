use aoc2020::read_stdin;

fn main() {
  let input = read_stdin().unwrap();
  let mut xs = input
    .split_ascii_whitespace()
    .map(|it| it.parse::<u32>().unwrap())
    .collect::<Vec<_>>();
  xs.push(0);
  xs.sort();

  xs.push(*xs.last().unwrap() + 3);

  let mut res = vec![!0u64; xs.len()];
  res[0] = 1;
  for i in 1..res.len() {
    res[i] =
      (0..i).rev().take_while(|&j| xs[i] - xs[j] <= 3).map(|j| res[j]).sum();
  }
  println!("{}", res.last().unwrap());
}
