use std::collections::HashSet;
mod utils;

fn parse_input(input: &str) -> (String, i32) {
  let line: Vec<&str> = input.trim().split(" ").collect();

  let sign = line[1].chars().nth(0).unwrap().to_string();

  let length = line[1].len();
  let mut value = line[1][1..length].parse::<i32>().unwrap();

  if sign == "-" {
    value = -value;
  }

  return (line[0].to_string(), value);
}

pub fn test(data: Vec<String>) -> (bool, i32) {
  let mut acc = 0;
  let mut i: i32 = 0;
  let mut tested = HashSet::new();

  while !(tested.contains(&i)) && (i as usize) < data.len() {
    tested.insert(i);

    let input = parse_input(&data[i as usize]);
    let (op, value) = &input;

    if op == "acc" {
      acc += value;
      i += 1;
    } else if op == "jmp" {
      let v = *value;
      i += v;
    } else {
      i += 1;
    }
  }

  let has_terminated = (i as usize) == data.len();

  if has_terminated {
    println!("Terminated correctly");
  }

  return (has_terminated, acc);
}

pub fn run() {
  println!("Day 08 !");

  let file_name = "./data/08.txt";
  let data = utils::read_input(file_name);

  for i in 0..(data.len()) {
    let mut clone = data.clone();

    let (op, value) = &parse_input(&clone[i as usize]);

    if op == "nop" {
      println!("Line {}, NOP to JMP", i);
      clone[i] = clone[i].replace("nop", "jmp");
      let (has_terminated, acc) = test(clone);

      if has_terminated {
        println!("Result {}", acc);
        break;
      }
    } else if op == "jmp" {
      println!("Line {}, JMP to NOP", i);
      clone[i] = clone[i].replace("jmp", "nop");
      let (has_terminated, acc) = test(clone);

      if has_terminated {
        println!("Result {}", acc);
        break;
      }
    } else {
      println!("...");
    }
  }
}
