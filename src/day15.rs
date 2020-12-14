mod utils;

fn part1(d: Vec<String>) {
  println!("PART 1");
}

fn part2(d: Vec<String>) {
  println!("PART 2");
}

pub fn run() {
  let file_name = "./data/15.txt";
  let data = utils::read_input(file_name);

  println!("Data {:?}", data);

  let clone_data = data.clone();

  part1(data);
  part2(clone_data);
}
