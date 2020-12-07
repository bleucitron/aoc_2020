// use std::fs::read_to_string;
use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

pub fn read_input(file_name: &str) -> Vec<String> {
  let file = File::open(file_name).expect("file not found!");
  let reader = BufReader::new(file);

  let mut strings = Vec::new();

  for line in reader.lines() {
    let my_string = String::from(&line.unwrap());

    strings.push(my_string);
  }

  return strings;
}

// pub fn read_input_with_empty_lines(file_name: &str) -> Vec<String> {
//   let input = read_to_string(file_name)
//     .expect("file not found!")
//     .to_string();

//   let lines = input
//     .split(
//       "

// ",
//     )
//     .map(|x| x.to_string())
//     .collect();

//   return lines;
// }
