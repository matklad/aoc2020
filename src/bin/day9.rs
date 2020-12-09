use aoc2020::{read_stdin, sum2};

fn main() {
  let input = read_stdin().unwrap();
  let xs = input
    .split_ascii_whitespace()
    .map(|it| it.parse::<u64>().unwrap())
    .collect::<Vec<_>>();
  let tgt = find_violation(&xs, 25).unwrap();

  let mut i = 0;
  let mut j = 1;
  let mut acc = xs[i] + xs[j];

  let (i, j) = loop {
    if acc == tgt {
      break (i, j);
    }
    if acc + xs[j + 1] <= tgt {
      j += 1;
      acc += xs[j];
    } else {
      acc -= xs[i];
      i += 1;
    }
  };

  let range = &xs[i..=j];
  let min = *range.iter().min().unwrap();
  let max = *range.iter().max().unwrap();
  println!("{}", min + max);
}

fn find_violation(xs: &[u64], n: usize) -> Option<u64> {
  let mut buf = xs[..n].to_vec();
  buf.sort();

  for i in n..xs.len() {
    if sum2(&buf, xs[i]).is_none() {
      return Some(xs[i]);
    }
    remove_item(&mut buf, xs[i - n]);
    insert_sorted(&mut buf, xs[i]);
  }
  None
}

fn remove_item(xs: &mut Vec<u64>, item: u64) {
  let idx = xs.iter().position(|&it| it == item).unwrap();
  xs.remove(idx);
}

fn insert_sorted(xs: &mut Vec<u64>, item: u64) {
  let idx = match xs.binary_search(&item) {
    Ok(it) => it,
    Err(it) => it,
  };
  xs.insert(idx, item)
}
