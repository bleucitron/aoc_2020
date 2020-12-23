mod utils;

fn move_around(input: Vec<i32>, nb: usize) -> Vec<i32> {
  let mut cups = input.clone();

  for i in 0..nb {
    if i % 1_000_000 == 0 {
      println!("MOVE nb {}", i);
    }
    let pos = if i >= cups.len() { i % cups.len() } else { i };
    let current = cups[pos];

    // let removed = &cups[1..4];
    // // PREND BCP TROP DE TEMPS
    let mut removed: Vec<i32> = Vec::new();
    let mut k = 1;
    while k < 4 {
      let mut to_remove = pos + 1;

      if to_remove >= cups.len() {
        to_remove = 0;
      }

      let value = cups.remove(to_remove);
      removed.push(value);

      k += 1;
    }

    // OK
    let mut destination_value = current - 1;

    while removed.iter().any(|&c| c == destination_value) {
      destination_value = destination_value - 1;
    }
    if destination_value == 0 {
      destination_value = *cups.iter().max().unwrap() as i32;
    }

    // PREND BEAUCOUP TROP DE TEMPS
    let destination = cups.iter().position(|&c| c == destination_value).unwrap() + 1;
    // let destination = 1000000;

    // TIMING A VERIFIER
    for j in 0..removed.len() {
      let mut new_pos = destination + j;

      if new_pos > cups.len() {
        new_pos = new_pos % cups.len();
      }
      cups.insert(new_pos, removed[j]);
    }

    while cups[pos] != current {
      let moved = cups.remove(0);
      cups.push(moved);
    }

    // println!("NEW {:?}", new_cups);
  }

  return cups;
}

pub fn run() {
  println!("Day 23 !");
  let file_name = "./data/test.txt";

  let input = utils::read_input(file_name);

  let mut cups: Vec<i32> = input[0]
    .clone()
    .chars()
    .map(|x| x.to_string().parse::<i32>().unwrap())
    .collect();

  println!("CUPS {:?}", cups);

  let mut final_cups = move_around(cups.clone(), 100);
  while final_cups[0] != 1 {
    let moved = final_cups.remove(0);
    final_cups.push(moved);
  }

  final_cups.remove(0);

  // println!("Final CUPS {:?}", final_cups);

  let mut output = "".to_string();
  for i in 0..final_cups.len() {
    output.push_str(&final_cups[i].to_string());
  }
  println!("   Part 1: {}", output);

  return;

  let mut cups_p2 = cups.clone();

  let max = *cups_p2.iter().max().unwrap() as i32;

  let nb_cups = 1_000_000;
  let nb_moves = 10_000_000;

  for i in (max + 1)..(nb_cups + 1) {
    cups_p2.push(i);
  }

  println!("CUPS length {}", cups_p2.len());
  let final_cups_p2 = move_around(cups_p2.clone(), nb_moves);

  let one_pos = final_cups_p2.iter().position(|&c| c == 1).unwrap();

  let len = nb_cups as usize;

  let pos1 = if one_pos + 1 >= len {
    (one_pos + 1) % len
  } else {
    one_pos + 1
  };
  let pos2 = if one_pos + 2 >= len {
    (one_pos + 2) % len
  } else {
    one_pos + 2
  };
  let first = final_cups_p2[pos1];
  let second = final_cups_p2[pos2];

  // println!("FINAL {:?}", final_cups_p2);
  println!("FIRST {}", first);
  println!("SECOND {}", second);
}
