mod utils;

fn move_around(input: Vec<i32>, nb: usize) -> Vec<i32> {
  let mut cups = input.clone();
  let mut cache = input.clone();
  let len = input.len();

  for i in 0..nb {
    if i > 0 && i % 1_000_00 == 0 {
      println!("MOVE nb {}", i);
    }
    let pos = if i >= cups.len() { i % cups.len() } else { i };
    let current = cups[pos];

    let r1 = cups[(pos + 1) % len];
    let r2 = cups[(pos + 2) % len];
    let r3 = cups[(pos + 3) % len];
    let mut to_replace = [r1, r2, r3].to_vec();
    let max = *cups.iter().max().unwrap() as i32;

    let mut destination = current;
    loop {
      destination = destination - 1;

      if destination == 0 {
        destination = *cups.iter().max().unwrap() as i32;
      }

      if destination != r1 && destination != r2 && destination != r3 {
        break;
      }
    }
    // let destination = cups.iter().position(|&c| c == destination_value).unwrap() + 1;
    // println!("  DESTINATION index {}", destination);

    // cups[pos +1] =

    let mut replace = false;
    for j in 0..len {
      if replace {
        if to_replace.len() == 0 {
          replace = false;
          break;
        } else {
          let r = to_replace.remove(0);
          let o = std::mem::replace(&mut cups[j], r);
        }
      } else if j > pos {
        let new_pos = (j + pos + 3) % len;
        cups[j] = cups[new_pos];
        if cups[j] == destination {
          replace = true;
        }
      }

      // println!("    {:?}", cups);
      // break;
    }
    // println!("");
    println!("NEW {:?}", cups);
    // break;
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

  println!("CUPS{:?}", cups);

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
