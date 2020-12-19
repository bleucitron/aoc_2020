use regex::Regex;
use std::collections::HashMap;
mod utils;

fn get_rule_map(s: String) -> HashMap<i32, String> {
  let rules: Vec<&str> = s.split('\n').collect();

  let mut map: HashMap<i32, String> = HashMap::new();

  let re = Regex::new(r"(\d*): (.*)").unwrap();
  for i in 0..rules.len() {
    let line = rules[i];
    for cap in re.captures_iter(line) {
      let pos = cap[1].parse::<i32>().unwrap();
      let rule = cap[2].to_string();

      // PART 1
      // map.insert(pos, rule);
      // PART 2
      if pos == 8 {
        map.insert(pos, "42 | 42 8".to_string());
      } else if pos == 11 {
        map.insert(pos, "42 31 | 42 11 31".to_string());
      } else {
        map.insert(pos, rule);
      }
    }
  }
  // println!("MAP {:?}", map);

  return map;
}

fn split(s: &str) -> Vec<i32> {
  // println!("S {}", s);
  return s.split(' ').map(|s| s.parse::<i32>().unwrap()).collect();
}

fn read_rule(s: &String, rule_string: &String, map: HashMap<i32, String>) -> (bool, String) {
  if s.len() == 0 {
    println!("    XXXX");
    return (false, "".to_string());
  }

  if rule_string.chars().nth(0).unwrap() == '"' {
    // println!("String {} Rule {}", s, rule_string);
    let pos = 0;

    let current_char = s.chars().nth(pos).unwrap();
    let valid_char = rule_string.chars().nth(1).unwrap();

    let mut next = s.clone();

    let is_valid = current_char == valid_char;
    next.remove(pos);

    return (is_valid, next);
  } else if rule_string.contains('|') {
    let strings: Vec<&str> = rule_string.split(" | ").collect();

    let rule1 = strings[0];
    let rule2 = strings[1];

    let (is_valid_1, n1) = read_rule(s, &rule1.to_string(), map.clone());

    if is_valid_1 {
      return (true, n1);
    } else {
      // println!("LEFT SIDE {} {}", rule1, false);
      let (is_valid_2, n2) = read_rule(s, &rule2.to_string(), map.clone());
      // println!("RIGHT SIDE {} {}", rule2, false);
      return (is_valid_2, n2);
    }
  } else {
    let rules: Vec<i32> = split(rule_string);

    // println!("COMBINED RULE {:?} {}", rules, rule_string);

    let mut is_valid = true;
    let mut next = s.clone();

    let mut vec: Vec<bool> = Vec::new();

    // if rule_string == "42 11 31" {
    if false {
      // TENTATIVE DE CHANGEMENT D'ORDRE, mais même en essayant d'inverser les caractères, ca ne change rien
      println!("BOUYA {}", s);

      let (valid_42, n_42) = read_rule(&next, map.get(&42).unwrap(), map.clone());
      println!("next 42 {}", n_42);

      is_valid = is_valid && valid_42;

      vec.push(valid_42);
      next = n_42.chars().rev().collect::<String>();

      let (valid_31, n_31) = read_rule(&next, map.get(&31).unwrap(), map.clone());

      is_valid = is_valid && valid_31;

      vec.push(valid_31);

      next = n_31;
      println!("after 31 {}", next);

      let (valid_11, n_11) = read_rule(&next, map.get(&11).unwrap(), map.clone());
      is_valid = is_valid && valid_11;
      vec.push(valid_11);
      next = n_11;
    } else {
      for i in 0..rules.len() {
        let rule = rules[i];

        let (_is_valid, n) = read_rule(&next, map.get(&rule).unwrap(), map.clone());
        is_valid = is_valid && _is_valid;

        next = n;
        vec.push(is_valid);
      }
    }

    // if !is_valid {
    //   println!("S {} {:?} {}", rule_string, v, s);
    //   println!("COMBO {} {}", is_valid, _next);
    // }
    // if rule_string == "42 11 31" {
    println!("RULES {:?}", rules);
    println!("VALIDS {:?}", vec);
    // }
    return (is_valid, next);
  }
}

fn check(s: String, map: HashMap<i32, String>) -> bool {
  let (is_valid, _next) = read_rule(&s, &map.get(&0).unwrap(), map.clone());

  println!("IS VALID {} {}", s, is_valid);
  return is_valid;
}

pub fn run() {
  println!("Day 19 !");
  let file_name = "./data/test.txt";

  let input = utils::read_input_with_empty_lines(file_name);

  let lines: Vec<&str> = input[1].trim().split('\n').collect();

  let map = get_rule_map(input[0].clone());

  check(lines[2].to_string(), map.clone());

  // let mut nb = 0;
  // for i in 0..lines.len() {
  //   let line = lines[i];

  //   let is_valid = check(line.to_string(), map.clone());

  //   if is_valid {
  //     nb += 1;
  //   }
  // }
  // println!("   Part 1: {}", nb)
}
