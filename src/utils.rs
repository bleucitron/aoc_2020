use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_input(file_name: &str) -> Vec<String> {
  let file = File::open(file_name).expect("file not found!");
  let reader = BufReader::new(file);

  let mut numbers = Vec::new();

  for line in reader.lines() {
    let my_string = String::from(&line.unwrap());

    numbers.push(my_string);
  }

  return numbers;
}
