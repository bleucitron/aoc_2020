use regex::Regex;
use std::collections::HashMap;
mod utils;

fn remove_bag(s: &str) -> String {
  return s.replace("bags", "").replace("bag", "").trim().to_string();
}

pub fn run() {
  let file_name = "./data/07.txt";
  let data = utils::read_input(file_name);

  let mut bag_rules = HashMap::new();

  for i in 0..data.len() {
    let d: Vec<&str> = data[i].split(" contain ").collect();

    let r = Regex::new(r"(\d) ([a-zA-Z ]*)").unwrap();
    let cleaned = &remove_bag(d[1]).replace(".", "");
    let captures = r.captures_iter(cleaned);

    let mut value_map = HashMap::new();

    for capture in captures {
      value_map.insert(
        capture[2].to_string().trim().to_string(),
        capture[1].parse::<i32>().unwrap(),
      );
    }

    bag_rules.insert(remove_bag(d[0]), value_map);
  }

  let mut nb = 0;

  fn check_rule(color: String, map: &HashMap<String, HashMap<String, i32>>) -> bool {
    match map.get(&color) {
      Some(rule) => {
        if rule.contains_key("shiny gold") {
          return true;
        } else {
          for c in rule.keys() {
            if check_rule(c.to_string(), map) {
              return true;
            }
          }
        }
      }
      None => return false,
    };
    return false;
  }

  for color in (&bag_rules).keys() {
    if check_rule(color.to_string(), &bag_rules) {
      nb += 1;
    }
  }

  println!("Part 1 {}", nb);

  fn count(color: String, map: &HashMap<String, HashMap<String, i32>>) -> i32 {
    match map.get(&color) {
      Some(rule) => {
        let mut nb = 1;

        for (r, c) in rule {
          nb += c * count(r.to_string(), map);
        }
        return nb;
      }
      None => return 0,
    };
  }

  println!("Part 2 {}", count("shiny gold".to_string(), &bag_rules) - 1);
}
