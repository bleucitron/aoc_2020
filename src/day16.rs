mod utils;

fn part1(d: Vec<String>) {
  let result = "";
  println!("   Part 1: {}", result);
}

fn part2(d: Vec<String>) {
  let result = "";
  println!("   Part 2: {}", result);
}

pub fn run() {
  let file_name = "./data/16.txt";
  let data = utils::read_input(file_name);

  println!("Data {:?}", data);

  part1(data.clone());
  part2(data.clone());
}
