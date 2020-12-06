use aoc2020::read_stdin;

fn main() {
  let input = read_stdin().unwrap();
  let groups = input.split("\n\n");

  let mut total = 0;
  let mut buf: Vec<u8> = Vec::new();
  for group in groups {
    buf.clear();
    let mut n_people = 0usize;
    for line in group.lines() {
      n_people += 1;
      buf.extend(line.as_bytes());
    }

    let mut common = 0usize;
    buf.sort();
    let mut slice = buf.as_slice();
    while !slice.is_empty() {
      let rep = slice.iter().take_while(|&&it| it == slice[0]).count();
      if rep == n_people {
        common += 1;
      }
      slice = &slice[rep..]
    }

    total += common;
  }

  println!("{}", total);
}
