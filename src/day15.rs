mod utils;
use std::collections::HashMap;

fn count(numbers: Vec<usize>, limit: usize) -> usize {
  let mut nb_by_turn: HashMap<usize, usize> = HashMap::new();

  let mut spoken = 0;

  for i in 0..numbers.len() {
    let s = numbers[i];
    nb_by_turn.insert(s, i + 1);
  }

  let mut i = numbers.len() + 1;
  let mut new_nb;

  while i < limit {
    // if i % 1000000 == 0 {
    //   println!("i {} M", i / 1000000);
    // }

    match nb_by_turn.get(&spoken) {
      Some(turn) => new_nb = i - turn,
      None => new_nb = 0,
    };

    nb_by_turn.insert(spoken, i);
    spoken = new_nb;

    i += 1;
  }

  return spoken;
}

fn part1(d: Vec<usize>) {
  let result = count(d, 2020);

  println!("   Part 1: {}", result);
}

fn part2(d: Vec<usize>) {
  let result = count(d, 30000000);

  println!("   Part 2: {}", result);
}

pub fn run() {
  let file_name = "./data/15.txt";
  let data = utils::read_input(file_name);
  let input = &data[0];

  let data: Vec<usize> = input
    .split(',')
    .map(|x| x.parse::<usize>().unwrap())
    .collect();

  println!("Data {:?}", data);

  part1(data.clone());
  part2(data.clone());
}
