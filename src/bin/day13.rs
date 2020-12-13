use std::io;

use num_bigint::BigInt;

fn main() {
  let mut buf = String::new();
  let _timestamp = {
    io::stdin().read_line(&mut buf).unwrap();
    buf.trim().parse::<u32>().unwrap()
  };
  let schedule = {
    buf.clear();
    io::stdin().read_line(&mut buf).unwrap();
    buf
      .trim()
      .split(',')
      .enumerate()
      .filter(|&(_idx, n)| n != "x")
      .map(|(idx, n)| {
        let n = n.parse::<BigInt>().unwrap();
        let idx: BigInt = idx.into();
        (-idx, n)
      })
      .collect::<Vec<_>>()
  };

  let mut g = (0.into(), 1.into());
  for gn in schedule.into_iter() {
    g = u(g, gn);
  }
  // x1t + x0 > 0
  // x1t > -x0
  let (x0, x1) = g;
  let t = -&x0 / &x1;

  println!("{}", t * x1 + x0);
}

type G = (BigInt, BigInt);

fn u((x0, x1): G, (y0, y1): G) -> G {
  // x1 * u' + x0 = y1 * v' + y0
  let (a, b, g) = egcd(x1.clone(), -&y1);
  assert!(&a * &x1 - b * &y1 == g);

  let k = div(&y0 - &x0, g.clone());

  // x1 * ak + x0 = y1 * bk + y0
  // u' = ak + (y1 / g) * t'
  (x0 + &x1 * a * k, div(y1, g) * &x1)
}

fn div(x: BigInt, y: BigInt) -> BigInt {
  let (d, m) = div_mod(x, y);
  assert!(m == 0.into());
  d
}

fn div_mod(x: BigInt, y: BigInt) -> (BigInt, BigInt) {
  (&x / &y, x % y)
}

fn egcd(x: BigInt, y: BigInt) -> (BigInt, BigInt, BigInt) {
  if y == 0.into() {
    return (1.into(), 0.into(), x);
  }
  let (d, m) = div_mod(x.clone(), y.clone());
  assert!(&d * &y + &m == x);
  let (a, b, g) = egcd(y, m);
  // a y + b(x - d y) = g
  (b.clone(), a - b * d, g)
}
