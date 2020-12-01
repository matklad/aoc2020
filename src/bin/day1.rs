use std::io::Read;

fn main() {
  let input: String = {
    let mut buf = String::new();
    std::io::stdin().read_to_string(&mut buf).unwrap();
    buf
  };

  let numbers: Vec<u32> = {
    let mut numbers =
      input.lines().map(|it| it.parse::<u32>().unwrap()).collect::<Vec<_>>();
    numbers.sort();
    numbers
  };

  let (x, y, z) = sum3(&numbers, 2020).unwrap();
  println!("{}", x * y * z);
}

fn sum2(xs: &[u32], tgt: u32) -> Option<(u32, u32)> {
  if xs.len() < 2 {
    return None;
  }
  let mut i = 0;
  let mut j = xs.len() - 1;
  while i < j {
    if xs[i] + xs[j] == tgt {
      return Some((xs[i], xs[j]));
    }
    if xs[i] + xs[j - 1] >= tgt {
      j -= 1;
    } else {
      i += 1;
    }
  }
  None
}

fn sum3(xs: &[u32], tgt: u32) -> Option<(u32, u32, u32)> {
  for j in 0..xs.len() {
    for i in 0..j {
      if let Some(r) = tgt.checked_sub(xs[i] + xs[j]) {
        if xs.binary_search(&r).is_ok() {
          return Some((xs[i], xs[j], r));
        }
      }
    }
  }
  None
}
