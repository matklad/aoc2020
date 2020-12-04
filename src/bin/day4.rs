use core::panic;

use aoc2020::{read_stdin, split_once};

fn main() {
  let input = read_stdin().unwrap();

  let passports = input.split("\n\n");
  let res = passports.filter(|&it| is_valid(it)).count();
  println!("{}", res)
}

fn is_valid(passport: &str) -> bool {
  // byr (Birth Year) - four digits; at least 1920 and at most 2002.
  // iyr (Issue Year) - four digits; at least 2010 and at most 2020.
  // eyr (Expiration Year) - four digits; at least 2020 and at most 2030.
  // hgt (Height) - a number followed by either cm or in:
  //   If cm, the number must be at least 150 and at most 193.
  //   If in, the number must be at least 59 and at most 76.
  // hcl (Hair Color) - a # followed by exactly six characters 0-9 or a-f.
  // ecl (Eye Color) - exactly one of: amb blu brn gry grn hzl oth.
  // pid (Passport ID) - a nine-digit number, including leading zeroes.
  let mut byr_valid = false;
  let mut iyr_valid = false;
  let mut eyr_valid = false;
  let mut hgt_valid = false;
  let mut hcl_valid = false;
  let mut ecl_valid = false;
  let mut pid_valid = false;

  let fields = passport.split_ascii_whitespace();
  for field in fields {
    let (name, value) = split_once(field, ':').unwrap();
    match name {
      "byr" => byr_valid = is_in_range(value, 1920, 2002),
      "iyr" => iyr_valid = is_in_range(value, 2010, 2020),
      "eyr" => eyr_valid = is_in_range(value, 2020, 2030),
      "hgt" => {
        hgt_valid = match value.get(value.len() - 2..) {
          Some(u @ "cm") | Some(u @ "in") => {
            let number = &value[..value.len() - 2];
            match u {
              "cm" => is_in_range(number, 150, 193),
              "in" => is_in_range(number, 59, 76),
              _ => unreachable!(),
            }
          }
          Some(_) | None => false,
        }
      }
      "hcl" => {
        hcl_valid = value.starts_with('#')
          && value.len() == 7
          && value[1..].chars().all(|c| matches!(c, '0'..='9' | 'a'..='f'))
      }
      "ecl" => {
        ecl_valid =
          matches!(value, "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth")
      }
      "pid" => {
        pid_valid =
          value.len() == 9 && value.chars().all(|c| matches!(c, '0'..='9'))
      }
      "cid" => (),
      _ => panic!("unknown field: {}", name),
    }
  }
  byr_valid
    && iyr_valid
    && eyr_valid
    && hgt_valid
    && hcl_valid
    && ecl_valid
    && pid_valid
}

fn is_in_range(value: &str, lo: u32, hi: u32) -> bool {
  match value.parse::<u32>() {
    Ok(it) => lo <= it && it <= hi,
    Err(_) => false,
  }
}
