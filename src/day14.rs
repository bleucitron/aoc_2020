use regex::Regex;
use std::collections::HashMap;
mod utils;

fn to_binary(d: usize) -> String {
  return format!("{:b}", d);
}

fn to_decimal(s: &str) -> isize {
  return isize::from_str_radix(s, 2).unwrap();
}

fn check_string(s: String) -> Vec<String> {
  let checked: Vec<char> = s.chars().collect();
  let mut output: Vec<String> = Vec::new();

  for i in 0..checked.len() {
    if checked[i] == 'X' {
      let mut v1 = checked.clone();
      v1[i] = '0';
      let s1: String = v1.into_iter().collect();
      let mut v2 = checked.clone();
      v2[i] = '1';
      let s2: String = v2.into_iter().collect();

      let o1 = check_string(s1);
      let o2 = check_string(s2);

      for j in 0..o1.len() {
        output.push(o1[j].clone());
      }
      for j in 0..o2.len() {
        output.push(o2[j].clone());
      }
      break;
    }
  }

  if !s.contains("X") {
    output.push(s.clone());
  }
  return output;
}

fn part1(d: Vec<String>) {
  let mut mask = "";

  let mut memory = HashMap::new();

  println!("{}", mask);

  for i in 0..d.len() {
    let line: Vec<&str> = d[i].split(" = ").collect();

    if line[0] == "mask" {
      mask = line[1];
      continue;
    }

    let r = Regex::new(r"mem\[(\d*)\]").unwrap();

    let input = line[1].parse::<usize>().unwrap();
    let mut pos = 0;

    for cap in r.captures_iter(line[0]) {
      pos = cap[1].parse::<i64>().unwrap();
    }

    let m: Vec<char> = mask.clone().chars().collect();
    let binary = format!("{:0>36}", to_binary(input));

    let mut new_chars: Vec<char> = binary.chars().collect();

    for i in 0..m.len() {
      if m[i] != 'X' {
        new_chars[i] = m[i];
      }
    }
    let output_string: String = new_chars.into_iter().collect();
    let output = to_decimal(&output_string);

    memory.insert(pos, output);
  }

  let mut result = 0;
  for value in memory.values() {
    result += value;
  }

  println!("   Part 1: {}", result);
}

fn part2(d: Vec<String>) {
  let mut mask = "";

  let mut memory = HashMap::new();

  for i in 0..d.len() {
    let line: Vec<&str> = d[i].split(" = ").collect();

    if line[0] == "mask" {
      mask = line[1];
      continue;
    }

    let r = Regex::new(r"mem\[(\d*)\]").unwrap();

    let input = line[1].parse::<usize>().unwrap();
    let mut pos = 0;

    for cap in r.captures_iter(line[0]) {
      pos = cap[1].parse::<usize>().unwrap();
    }

    let m: Vec<char> = mask.clone().chars().collect();
    let binary = format!("{:0>36}", to_binary(pos));

    let mut new_chars: Vec<char> = binary.chars().collect();

    for i in 0..m.len() {
      if m[i] == '1' {
        new_chars[i] = '1';
      } else if m[i] == 'X' {
        new_chars[i] = 'X';
      }
    }
    let output_string: String = new_chars.into_iter().collect();

    let mems = check_string(output_string);

    for i in 0..mems.len() {
      let s = mems[i].to_string();
      let pos = to_decimal(&s);

      memory.insert(pos, input);
    }
  }

  let mut result = 0;
  for value in memory.values() {
    result += value;
  }

  println!("   Part 2: {}", result);
}

pub fn run() {
  let file_name = "./data/14.txt";
  let data = utils::read_input(file_name);
  let data2 = data.clone();

  part1(data);
  part2(data2);
}
