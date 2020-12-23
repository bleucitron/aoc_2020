mod utils;

fn move_around(input: Vec<i32>, nb: usize) -> Vec<i32> {
  let mut cups = input.clone();
  let len = cups.len();

  for i in 0..nb {
    if i > 0 && i % 1_0 == 0 {
      println!("MOVE nb {}", i);
    }
    let pos = if i >= cups.len() { i % cups.len() } else { i };
    let current = cups[pos];

    // println!("");
    // println!("CUrrent {}", current);

    let mut doubled = cups.clone();
    doubled.extend(doubled.clone());
    // println!("Doubled {:?}", doubled);

    let start = cups[0..pos].to_vec();
    let start_len = start.len();
    let mut changed = cups[pos..(pos + 1)].to_vec();
    let end = doubled[((len + pos + 4) % (2 * len))..(2 * len)].to_vec();

    let removed = doubled[(pos + 1)..(pos + 4)].to_vec();

    changed.extend(end);
    changed.extend(start);

    // println!("Changed {:?}", changed);
    // println!("Removed {:?}", removed);

    let mut destination_value = current;

    loop {
      destination_value = destination_value - 1;

      if destination_value == 0 {
        destination_value = *changed.iter().max().unwrap() as i32;
      }

      if !removed.iter().any(|&c| c == destination_value) {
        break;
      }
    }

    // println!("  DESTINATION {}", destination_value);
    let destination = changed
      .iter()
      .position(|&c| c == destination_value)
      .unwrap()
      + 1;
    // println!("  DESTINATION index {}", destination);

    let mut split = changed[0..destination].to_vec();
    let split_2 = changed[destination..changed.len()].to_vec();

    split.extend(removed);
    split.extend(split_2);

    // println!("SPLIT {:?}", split);
    let split_3 = split[0..(len - start_len)].to_vec();
    let mut final_cups = split[(len - start_len)..len].to_vec();
    final_cups.extend(split_3);

    cups = final_cups;

    // println!("NEW {:?}", cups);
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

  // return;

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
