use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
mod utils;

fn part1(tickets: Vec<String>, rules: &HashMap<usize, impl Fn(&i32) -> bool>) {
  let nbs = nb_from_tickets(tickets);
  let mut to_remove: Vec<i32> = Vec::new();

  for i in 0..nbs.len() {
    let nb = nbs[i];

    let mut is_valid = false;

    for check in rules.values() {
      is_valid = check(&nb);

      if is_valid {
        break;
      }
    }

    if !is_valid {
      to_remove.push(nb);
    }
  }

  println!("   Part 1: {}", sum(to_remove));
}

fn part2(
  tickets: Vec<String>,
  rules: &HashMap<usize, impl Fn(&i32) -> bool>,
  my_ticket_values: Vec<usize>,
) {
  let mut valid_tickets: Vec<Vec<i32>> = Vec::new();

  println!("STEP 1: valid tickets");
  for i in 0..tickets.len() {
    let ticket = &tickets[i];

    let nbs = nb_from_ticket(&ticket);

    let mut is_ticket_valid = true;

    for j in 0..nbs.len() {
      let nb = &nbs[j];
      let mut is_valid = false;

      for check in rules.values() {
        is_valid = is_valid || check(&nb);
      }
      // for k in 0..rules.len() {
      //   let rule = &rules[k];
      //   let check = get_rule(&rule);

      //   is_valid = is_valid || check(&nb);
      // }

      is_ticket_valid = is_ticket_valid && is_valid;
      if !is_ticket_valid {
        break;
      }
    }
    if is_ticket_valid {
      valid_tickets.push(nbs.clone());
    }
  }

  println!("STEP 2: possible rules for each position");

  // println!("VALID Tickets {:?}", valid_tickets);

  let mut map: HashMap<usize, HashSet<usize>> = HashMap::new();

  let mut possible = HashSet::new();

  for i in 0..rules.len() {
    possible.insert(i);
  }

  let ticket_length = valid_tickets[0].len();

  for position in 0..ticket_length {
    let mut possible_rules: HashSet<usize> = HashSet::new();

    match map.get(&position) {
      Some(v) => possible_rules = v.clone(),
      None => possible_rules = possible.clone(),
    }

    for i in 0..valid_tickets.len() {
      let ticket = &valid_tickets[i];

      let nb = &ticket[position];

      for (k, check) in rules.iter() {
        if !check(nb) {
          possible_rules.remove(&k);
        }
      }
    }

    if possible_rules.len() == 1 {
      let to_remove: Vec<usize> = Vec::from_iter(possible_rules.clone());
      possible.remove(&to_remove[0]);
    }
    map.insert(position, possible_rules.clone());
  }

  println!("STEP 3: clean rules");

  let cleaned = clean_map(map);

  let mut answer = 1;

  for i in 0..my_ticket_values.len() {
    let value = my_ticket_values[i];

    match cleaned.get(&i) {
      Some(v) => {
        if v < &6 {
          answer *= value;
        }
      }
      None => {}
    }
  }

  println!("   Part 2: {}", answer);
}

fn get_rule(s: &String) -> impl Fn(&i32) -> bool {
  let re = Regex::new(r"([a-z]*): (\d*)-(\d*) or (\d*)-(\d*)").unwrap();
  let captures = re.captures_iter(&s);

  let mut min1: i32 = 0;
  let mut max1: i32 = 0;
  let mut min2: i32 = 0;
  let mut max2: i32 = 0;

  for cap in captures {
    min1 = cap[2].parse::<i32>().unwrap();
    max1 = cap[3].parse::<i32>().unwrap();
    min2 = cap[4].parse::<i32>().unwrap();
    max2 = cap[5].parse::<i32>().unwrap();
  }

  move |n| n >= &min1 && n <= &max1 || n >= &min2 && n <= &max2
}

fn nb_from_tickets(tickets: Vec<String>) -> Vec<i32> {
  let mut numbers: Vec<i32> = Vec::new();

  for i in 0..tickets.len() {
    let ticket = &tickets[i];

    let _numbers: Vec<i32> = ticket
      .split(',')
      .map(|n| n.parse::<i32>().unwrap())
      .collect();

    for j in 0.._numbers.len() {
      numbers.push(_numbers[j]);
    }
  }

  return numbers;
}

fn nb_from_ticket(ticket: &String) -> Vec<i32> {
  let mut numbers: Vec<i32> = Vec::new();

  let _numbers: Vec<i32> = ticket
    .split(',')
    .map(|n| n.parse::<i32>().unwrap())
    .collect();

  for j in 0.._numbers.len() {
    numbers.push(_numbers[j]);
  }

  return numbers;
}

fn sum(nbs: Vec<i32>) -> i32 {
  let mut sum = 0;

  for i in 0..nbs.len() {
    sum += nbs[i];
  }

  return sum;
}

fn clean_map(map: HashMap<usize, HashSet<usize>>) -> HashMap<usize, usize> {
  let mut current_map = map.clone();
  let mut new_map: HashMap<usize, usize> = HashMap::new();

  while current_map.len() > 0 {
    for (key, val) in current_map.clone().iter() {
      if val.len() == 1 {
        let mut clone_map = current_map.clone();
        clone_map.remove(key).unwrap();

        let to_remove = Vec::from_iter(val.clone())[0];
        new_map.insert(*key, to_remove);

        for (k, v) in clone_map.clone().iter() {
          if v.len() > 0 && v.contains(&to_remove) {
            let mut clone = v.clone();
            clone.remove(&to_remove);
            clone_map.insert(*k, clone);
          }
        }

        current_map = clone_map;
      }
    }
  }

  return new_map;
}

pub fn run() {
  let file_name = "./data/16.txt";
  let data = utils::read_input_with_empty_lines(file_name);

  let rules: Vec<String> = data[0]
    .trim()
    .split('\n')
    .map(|x| x.trim().to_string())
    .collect();
  let mut tickets: Vec<String> = data[2]
    .trim()
    .split('\n')
    .map(|x| x.trim().to_string())
    .collect();
  tickets.remove(0);

  let my_ticket: Vec<String> = data[1]
    .trim()
    .split('\n')
    .map(|x| x.trim().to_string())
    .collect();

  let my_values: Vec<usize> = my_ticket[1]
    .split(',')
    .map(|x| x.trim().parse::<usize>().unwrap())
    .collect();

  let mut rule_map: HashMap<usize, _> = HashMap::new();

  for j in 0..rules.len() {
    let rule = &rules[j];
    let check = get_rule(&rule);

    rule_map.insert(j, check);
  }

  part1(tickets.clone(), &rule_map);
  part2(tickets.clone(), &rule_map, my_values.clone());
}
