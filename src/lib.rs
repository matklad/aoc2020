mod interner;

use std::io::{self, Read};

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
