use regex::Regex;
use std::collections::HashMap;
mod utils;

#[derive(Clone, Debug)]
pub enum Rule {
  Literal(char),
  Seq(Vec<i32>),
  Or(Vec<i32>, Vec<i32>),
}

fn get_rule_map(s: String) -> HashMap<i32, Rule> {
  let rules: Vec<&str> = s.split('\n').collect();

  let mut map: HashMap<i32, Rule> = HashMap::new();

  let re = Regex::new(r"(\d*): (.*)").unwrap();
  for i in 0..rules.len() {
    let line = rules[i];
    for cap in re.captures_iter(line) {
      let pos = cap[1].parse::<i32>().unwrap();
      let rule = cap[2].to_string();

      if rule.chars().nth(0).unwrap() == '"' {
        let c = rule.chars().nth(1).unwrap();
        map.insert(pos, Rule::Literal(c));
      } else if rule.contains('|') {
        let strings: Vec<&str> = rule.split(" | ").collect();

        let left = split(strings[0]);
        let right = split(strings[1]);

        map.insert(pos, Rule::Or(left, right));
      } else {
        let seq = split(&rule);

        map.insert(pos, Rule::Seq(seq));
      }
    }
  }

  return map;
}

fn split(s: &str) -> Vec<i32> {
  return s.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
}

fn validate(s: String, mut next: Vec<Rule>, map: HashMap<i32, Rule>) -> bool {
  if next.len() == 0 {
    return s == "";
  }

  if s.len() == 0 {
    return false;
  }

  match next.pop() {
    Some(Rule::Literal(c)) => {
      let mut s_copy = s.clone();
      let first = s_copy.remove(0);

      return first == c && validate(s_copy, next, map);
    }
    Some(Rule::Seq(seq)) => {
      let n = seq.len();
      for i in 0..n {
        let rule = map.get(&seq[n - 1 - i]).unwrap().clone();

        next.push(rule);
      }

      return validate(s.clone(), next, map);
    }
    Some(Rule::Or(seq1, seq2)) => {
      let l1 = seq1.len();
      let mut n1 = next.clone();

      for i in 0..l1 {
        let rule = map.get(&seq1[l1 - 1 - i]).unwrap().clone();

        n1.push(rule);
      }

      let l2 = seq2.len();
      let mut n2 = next.clone();

      for i in 0..l2 {
        let rule = map.get(&seq2[l2 - 1 - i]).unwrap().clone();

        n2.push(rule);
      }

      return validate(s.clone(), n1, map.clone()) || validate(s.clone(), n2, map.clone());
    }
    None => {
      return true;
    }
  }
}

fn check(s: String, map: HashMap<i32, Rule>) -> bool {
  let mut rules: Vec<Rule> = Vec::new();

  rules.push(map.get(&0).unwrap().clone());

  let is_valid = validate(s.clone(), rules, map);

  // println!("IS VALID {} {}", s, is_valid);
  return is_valid;
}

pub fn run() {
  println!("Day 19 !");
  let file_name = "./data/19.txt";

  let input = utils::read_input_with_empty_lines(file_name);

  let lines: Vec<&str> = input[1].trim().split('\n').collect();

  let mut map = get_rule_map(input[0].clone());

  map.insert(
    8,
    Rule::Or(
      [42].iter().copied().collect(),
      [42, 8].iter().copied().collect(),
    ),
  );
  map.insert(
    11,
    Rule::Or(
      [42, 31].iter().copied().collect(),
      [42, 11, 31].iter().copied().collect(),
    ),
  );

  let mut nb = 0;
  for i in 0..lines.len() {
    let line = lines[i];

    let is_valid = check(line.to_string(), map.clone());

    if is_valid {
      nb += 1;
    }
  }
  println!("   Part 2: {}", nb)
}
