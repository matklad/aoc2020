use std::{
  io::{self, BufRead},
  ops::RangeInclusive,
};

use aoc2020::split_once;

fn main() {
  let mut buf = String::new();
  let stdin = io::stdin();
  let mut stdin = stdin.lock();

  let mut roles: Vec<(String, [RangeInclusive<u32>; 2])> = Vec::new();
  loop {
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    if buf.trim().is_empty() {
      break;
    }
    let (role, restrictions) = split_once(&buf, ':').unwrap();
    let mut restrictions = restrictions
      .split("or")
      .map(|it| it.trim())
      .map(|it| split_once(it, '-').unwrap())
      .map(|(lo, hi)| lo.parse::<u32>().unwrap()..=hi.parse::<u32>().unwrap());
    roles.push((
      role.to_string(),
      [restrictions.next().unwrap(), restrictions.next().unwrap()],
    ));
    assert!(restrictions.next().is_none());
  }

  buf.clear();
  stdin.read_line(&mut buf).unwrap();
  assert!(buf.starts_with("your ticket"));

  buf.clear();
  stdin.read_line(&mut buf).unwrap();
  let my_ticket = parse_ticket(&buf);

  buf.clear();
  stdin.read_line(&mut buf).unwrap();
  assert!(buf.trim().is_empty());

  buf.clear();
  stdin.read_line(&mut buf).unwrap();
  assert!(buf.starts_with("nearby tickets"));

  let mut tickets = Vec::new();
  loop {
    buf.clear();
    stdin.read_line(&mut buf).unwrap();
    if buf.trim().is_empty() {
      break;
    }
    tickets.push(parse_ticket(&buf));
  }

  let all = (1u32 << my_ticket.len() as u32) - 1;
  let mut pos = vec![all; my_ticket.len()];

  for ticket in &tickets {
    let mut tpos = vec![0u32; my_ticket.len()];
    for (idx, &val) in ticket.iter().enumerate() {
      for (ridx, (_, [lo, hi])) in roles.iter().enumerate() {
        if lo.contains(&val) || hi.contains(&val) {
          tpos[idx] |= 1u32 << ridx as u32;
        }
      }
    }
    if tpos.iter().any(|&it| it == 0) {
      continue;
    }
    pos.iter_mut().zip(&tpos).for_each(|(p, tp)| *p &= *tp);
  }

  let mut classes = vec![""; my_ticket.len()];
  let mut worklist = (0..my_ticket.len()).collect::<Vec<_>>();

  while !worklist.is_empty() {
    let idx = {
      let i =
        worklist.iter().position(|&idx| pos[idx].count_ones() == 1).unwrap();
      worklist.swap_remove(i)
    };
    let m = pos[idx];
    for p in &mut pos {
      *p &= !m;
    }
    classes[idx] = roles[m.trailing_zeros() as usize].0.as_str();
  }

  let res = classes
    .iter()
    .copied()
    .enumerate()
    .filter(|(_, it)| it.starts_with("departure"))
    .map(|(idx, _)| u64::from(my_ticket[idx]))
    .inspect(|it| println!("{}", it))
    .product::<u64>();

  eprintln!("{}", res);
}

fn parse_ticket(ticket: &str) -> Vec<u32> {
  ticket.trim().split(',').map(|it| it.parse::<u32>().unwrap()).collect()
}
