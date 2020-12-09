mod interner;

use std::{
  io::{self, Read},
  ops,
};

pub use crate::interner::Interner;

pub fn read_stdin() -> io::Result<String> {
  let mut buf = String::new();
  std::io::stdin().read_to_string(&mut buf)?;
  Ok(buf)
}

pub fn split_once(haystack: &str, delim: char) -> Option<(&str, &str)> {
  let mut split = haystack.splitn(2, delim);
  let prefix = split.next()?;
  let suffix = split.next()?;
  Some((prefix, suffix))
}

pub fn sum2<T>(xs: &[T], tgt: T) -> Option<(T, T)>
where
  T: Ord + ops::Add<Output = T> + Copy,
{
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
