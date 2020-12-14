use std::{collections::HashMap, io};

use aoc2020::split_once;

fn main() {
  let mut buf = String::new();
  let stdin = io::stdin();

  let mut sm = 0u64;
  let mut fm = 0u64;
  let mut mem: HashMap<u64, u64> = HashMap::new();

  loop {
    let buf = {
      buf.clear();
      let n_bytes = stdin.read_line(&mut buf).unwrap();
      if n_bytes == 0 {
        break;
      }
      let buf = buf.trim();
      if buf.is_empty() {
        break;
      }
      buf
    };

    if let Some(mask) = buf.strip_prefix("mask = ") {
      sm = 0u64;
      fm = 0u64;
      for c in mask.chars() {
        sm <<= 1;
        fm <<= 1;
        match c {
          '1' => sm |= 1,
          '0' => (),
          'X' => fm |= 1,
          _ => panic!(),
        }
      }
    } else {
      let (addr, val) = split_once(buf, '=').unwrap();
      let addr = addr
        .strip_prefix("mem[")
        .unwrap()
        .strip_suffix("] ")
        .unwrap()
        .parse::<u64>()
        .unwrap();

      let val = val.trim().parse::<u64>().unwrap();

      let addr = (addr | sm) & !fm;
      let mut m = fm;
      loop {
        mem.insert(addr | m, val);
        if m == 0 {
          break;
        }
        m = (m - 1) & fm;
      }
    }
  }

  let res = mem.values().sum::<u64>();
  println!("{}", res);
}
