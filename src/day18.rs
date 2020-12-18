mod utils;

fn parse_p1(s: &String) -> i64 {
  let mut operation = '*';
  let mut v: Vec<i64> = Vec::new();

  let mut new_s = s.to_string();

  let mut removing = 0;

  while new_s.len() > 0 {
    let c = new_s.remove(0);

    if c == '(' {
      if removing == 0 {
        let res = parse_p1(&new_s);
        v.push(res);
      }
      removing += 1;
      continue;
    }
    if c == ')' {
      if removing > 0 {
        removing -= 1;
      } else {
        break;
      }
    }
    if removing > 0 {
      continue;
    }
    if c == ' ' {
      continue;
    }
    if c == '+' || c == '*' {
      operation = c;
      continue;
    }
    if c != '(' && c != ')' {
      v.push(c.to_digit(10).unwrap() as i64);
    }

    if removing == 0 && v.len() == 2 {
      let mut new_v: Vec<i64> = Vec::new();
      if operation == '*' {
        let a = v[0];
        let b = v[1];
        let result = (a * b) as i64;
        new_v.push(result);
      } else if operation == '+' {
        let a = v[0];
        let b = v[1];
        let result = (a + b) as i64;
        new_v.push(result);
      }
      v = new_v;
    }
  }

  return v[0];
}

fn parse_p2(s: &String) -> i64 {
  let mut operation = '*';
  let mut v: Vec<i64> = Vec::new();
  let mut v_mult: Vec<i64> = Vec::new();

  let mut new_s = s.to_string();

  let mut removing = 0;

  while new_s.len() > 0 {
    let c = new_s.remove(0);

    if c == '(' {
      if removing == 0 {
        let res = parse_p2(&new_s);
        v.push(res);
      }
      removing += 1;
      continue;
    }
    if c == ')' {
      if removing > 0 {
        removing -= 1;
      } else {
        break;
      }
    }
    if removing > 0 {
      continue;
    }
    if c == ' ' {
      continue;
    }
    if c == '+' || c == '*' {
      operation = c;
      continue;
    }
    if c != '(' && c != ')' {
      v.push(c.to_digit(10).unwrap() as i64);
    }

    if removing == 0 && v.len() == 2 {
      let mut new_v: Vec<i64> = Vec::new();
      if operation == '*' {
        v_mult.push(v[0]);
        new_v.push(v[1]);
      } else if operation == '+' {
        let a = v[0];
        let b = v[1];

        let result = (a + b) as i64;
        new_v.push(result);
      }
      v = new_v;
    }

    if (removing == 0) && v_mult.len() == 2 {
      let mut new_v: Vec<i64> = Vec::new();
      let a = v_mult[0];
      let b = v_mult[1];
      let result = (a * b) as i64;
      new_v.push(result);
      v_mult = new_v;
    }
  }
  if v_mult.len() > 0 {
    return v[0] * v_mult[0];
  } else {
    return v[0];
  }
}

fn part1(input: Vec<String>) {
  let mut sum = 0;

  for i in 0..input.len() {
    sum += parse_p1(&input[i]);
  }

  println!("   Part 1: {}", sum);
}

fn part2(input: Vec<String>) {
  let mut sum = 0;

  for i in 0..input.len() {
    let res = parse_p2(&input[i]);
    sum += res;
  }

  println!("   Part 2: {}", sum);
}

pub fn run() {
  println!("Day 18 !");
  let file_name = "./data/18.txt";

  let input = utils::read_input(file_name);

  part1(input.clone());
  part2(input.clone());
}
