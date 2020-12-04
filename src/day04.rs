use regex::Regex;
use std::fs::read_to_string;

fn parse_input(file_name: &str) -> Vec<String> {
  let input = read_to_string(file_name)
    .expect("file not found!")
    .to_string();

  let lines: Vec<String> = input
    .split(
      "

",
    )
    .map(|x| x.to_string())
    .collect();

  return lines;
}

pub fn check_byr(s: &str) -> bool {
  let re = Regex::new(r"\d{4}$").unwrap();

  let reg_exp_ok = re.is_match(s);
  let value = s.parse::<i32>().unwrap();
  let value_ok = value >= 1920 && value <= 2002;

  return reg_exp_ok && value_ok;
}

pub fn check_iyr(s: &str) -> bool {
  let re = Regex::new(r"\d{4}$").unwrap();

  let reg_exp_ok = re.is_match(s);
  let value = s.parse::<i32>().unwrap();
  let value_ok = value >= 2010 && value <= 2020;

  return reg_exp_ok && value_ok;
}

pub fn check_eyr(s: &str) -> bool {
  let re = Regex::new(r"\d{4}$").unwrap();

  let reg_exp_ok = re.is_match(s);
  let value = s.parse::<i32>().unwrap();
  let value_ok = value >= 2020 && value <= 2030;

  return reg_exp_ok && value_ok;
}

pub fn check_hgt(s: &str) -> bool {
  let re = Regex::new(r"([\d]*)(in|cm)").unwrap();

  for cap in re.captures_iter(s) {
    let unit = &cap[2];
    let value = (&cap[1]).parse::<i32>().unwrap();

    if unit == "in" {
      return value >= 59 && value <= 76;
    } else if unit == "cm" {
      return value >= 150 && value <= 193;
    }
  }
  return false;
}

pub fn check_hcl(s: &str) -> bool {
  let re = Regex::new(r"^#([0-9a-f]){6}$").unwrap();

  return re.is_match(s);
}

pub fn check_ecl(s: &str) -> bool {
  let re = Regex::new(r"^(amb|blu|brn|gry|grn|hzl|oth){1}$").unwrap();

  return re.is_match(s);
}

pub fn check_pid(s: &str) -> bool {
  let re = Regex::new(r"^([0-9]){9}$").unwrap();

  return re.is_match(s);
}

pub fn check_value(code: &str, value: &str) -> bool {
  if code == "byr" {
    return check_byr(value);
  } else if code == "iyr" {
    return check_iyr(value);
  } else if code == "eyr" {
    return check_eyr(value);
  } else if code == "hgt" {
    return check_hgt(value);
  } else if code == "hcl" {
    return check_hcl(value);
  } else if code == "ecl" {
    return check_ecl(value);
  } else if code == "pid" {
    return check_pid(value);
  } else if code == "cid" {
    return true;
  }

  return false;
}

fn validate(passport: String) -> bool {
  let temp = passport.replace("\n", " ");
  let cleaned = temp.trim_end();
  let pairs = cleaned.split(" ");

  let mut is_data_valid = true;

  for pair in pairs {
    let data: Vec<&str> = pair.split(":").collect();
    // println!("PAIR {}", pair);
    // println!("DATA {:?}", data);
    let code = data[0];
    let value = data[1];
    // println!("PAIR {0} {1}", data[0], data[1]);

    let is_value_valid = check_value(code, value);

    is_data_valid = is_value_valid && is_data_valid;
  }

  let is_valid = passport.contains("byr")
    && passport.contains("iyr")
    && passport.contains("eyr")
    && passport.contains("hgt")
    && passport.contains("hcl")
    && passport.contains("ecl")
    && passport.contains("pid");

  return is_valid && is_data_valid;
}

pub fn run() {
  println!("Day 04 !");

  let file_name = "./data/04.txt";
  let passports = parse_input(file_name);

  let mut result = 0;

  for i in 0..(passports.len()) {
    let is_valid = validate(passports[i].to_string());

    if is_valid {
      result += 1;
    }
  }

  println!("Nb valid {}", result);
}
