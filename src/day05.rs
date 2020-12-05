use std::collections::HashSet;
mod utils;

fn get_row(s: &str) -> usize {
  let mut min = 0;
  let mut max = 127;

  for letter in s.chars() {
    if letter == 'F' {
      max = max - ((max - min) / 2) - 1;
    } else if letter == 'B' {
      min = min + ((max - min) / 2) + 1;
    }
  }

  if min != max {
    panic!("{} and {} should be equal", min, max);
  }
  return min;
}

fn get_column(s: &str) -> usize {
  let mut min = 0;
  let mut max = 7;

  for letter in s.chars() {
    if letter == 'L' {
      max = max - ((max - min) / 2) - 1;
    } else if letter == 'R' {
      min = min + ((max - min) / 2) + 1;
    }
  }

  if min != max {
    panic!("{} and {} should be equal", min, max);
  }
  return min;
}

fn decode(code: &str) -> usize {
  let rows = &code[0..7];
  let row = get_row(rows);

  let columns = &code[7..10];
  let column = get_column(columns);

  return row * 8 + column;
}

pub fn run() {
  println!("Day 05 !");

  let file_name = "./data/05.txt";
  let passes = utils::read_input(file_name);

  let mut max = 0;

  let mut ids = HashSet::new();

  for i in 0..(passes.len()) {
    let id = decode(&passes[i]);
    ids.insert(id);

    if id > max {
      max = id;
    }
  }

  let mut available_ids = HashSet::new();

  for i in 0..(max + 1) {
    available_ids.insert(i);
  }

  for id in ids {
    if available_ids.contains(&id) {
      available_ids.remove(&id);
    }
  }

  let mut _ids: Vec<usize> = available_ids.into_iter().collect();
  _ids.sort_by(|a, b| b.cmp(a));

  println!("MY SEAT {}", _ids[0]);
}
