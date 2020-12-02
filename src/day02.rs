mod utils;

pub fn run() {
  println!("Day 02 !");
  let file_name = "./data/02.txt";

  let pwds = utils::read_input(file_name);

  let mut valid = 0;

  for i in 0..pwds.len() {
    let pwd = &pwds[i];
    let v: Vec<&str> = pwd.split(':').collect();

    let rule: Vec<&str> = v[0].split(' ').collect();
    let pwd = v[1].trim();

    let numbers: Vec<&str> = rule[0].split('-').collect();
    let letter = rule[1];

    // PART 1
    // let min: usize = numbers[0].parse().unwrap();
    // let max: usize = numbers[1].parse().unwrap();
    // let count = pwd.matches(letter).count();
    // if (count >= min) && (count <= max) {
    //   valid += 1;
    // }

    // PART 2
    let index1: usize = numbers[0].parse().unwrap();
    let index2: usize = numbers[1].parse().unwrap();

    let letter1 = pwd.chars().nth(index1 - 1).unwrap().to_string();
    let letter2 = pwd.chars().nth(index2 - 1).unwrap().to_string();

    if (letter1 == letter) && (letter2 != letter) {
      valid += 1;
    } else if (letter1 != letter) && (letter2 == letter) {
      valid += 1;
    }
  }

  println!("VALID {}", valid);
}
