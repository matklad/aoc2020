use std::{collections::HashMap, mem, rc::Rc, str::FromStr};

use aoc2020::{read_stdin, split_once};

fn main() {
  let input = read_stdin().unwrap();
  let idx = input.find("\n\n").unwrap();
  let (rules, cases) = input.split_at(idx);
  let rules = rules.parse::<RuleSet>().unwrap();
  let cases = cases.trim();

  let res = cases.lines().filter(|&it| rules.matches(it)).count();
  println!("{}", res);
}

struct RuleSet {
  rules: Vec<Rule>,
}

#[derive(Clone, PartialEq, Eq)]
enum Rule {
  Char(char),
  Seq(Vec<usize>),
  Alt(Vec<usize>, Vec<usize>),
}

const DUMMY: Rule = Rule::Char('?');

impl FromStr for RuleSet {
  type Err = ();

  fn from_str(s: &str) -> Result<Self, ()> {
    let mut rules = Vec::new();
    for line in s.lines() {
      let (idx, rule) = split_once(line, ':').unwrap();
      let idx = idx.parse::<usize>().unwrap();
      let rule = rule.trim();
      let r = if rule.starts_with('"') {
        let ch = rule.chars().nth(1).unwrap();
        Rule::Char(ch)
      } else if let Some((lhs, rhs)) = split_once(rule, '|') {
        Rule::Alt(parse_seq(lhs)?, parse_seq(rhs)?)
      } else {
        Rule::Seq(parse_seq(rule)?)
      };
      let n = rules.len().max(idx + 1);
      rules.resize(n, DUMMY);
      let prev = mem::replace(&mut rules[idx], r);
      assert!(matches!(prev, DUMMY));
    }
    Ok(RuleSet { rules })
  }
}

fn parse_seq(seq: &str) -> Result<Vec<usize>, ()> {
  seq.split_ascii_whitespace().map(|it| it.parse().map_err(drop)).collect()
}

impl RuleSet {
  fn matches(&self, text: &str) -> bool {
    let mut ctx = Ctx { rules: self, text, cache: HashMap::new() };
    ctx.m(0, 0).contains(&text.len())
  }
}

struct Ctx<'a> {
  rules: &'a RuleSet,
  text: &'a str,
  cache: HashMap<(usize, usize), Rc<Vec<usize>>>,
}

impl<'a> Ctx<'a> {
  fn m(&mut self, rule: usize, pos: usize) -> Rc<Vec<usize>> {
    let key = (rule, pos);
    if let Some(res) = self.cache.get(&key) {
      return Rc::clone(res);
    }
    let res = match &self.rules.rules[rule] {
      Rule::Char(c) => {
        if self.text[pos..].starts_with(*c) {
          vec![c.len_utf8()]
        } else {
          vec![]
        }
      }
      Rule::Seq(rules) => self.m_seq(rules, pos),
      Rule::Alt(rules1, rules2) => {
        let mut res = self.m_seq(rules1, pos);
        res.extend(self.m_seq(rules2, pos));
        res.sort();
        res.dedup();
        res
      }
    };
    let res = Rc::new(res);
    self.cache.insert(key, Rc::clone(&res));
    res
  }
  fn m_seq(&mut self, seq: &[usize], pos: usize) -> Vec<usize> {
    let mut res = vec![0];
    let mut next = Vec::new();
    for &rule in seq {
      next.clear();
      for &l1 in &res {
        for &l2 in self.m(rule, pos + l1).iter() {
          next.push(l1 + l2);
        }
      }
      next.sort();
      next.dedup();
      mem::swap(&mut res, &mut next);
    }
    res
  }
}
