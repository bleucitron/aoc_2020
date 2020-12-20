mod utils;
use regex::Regex;
use std::cmp;
use std::collections::HashMap;

fn build_map(tiles: Vec<String>) -> HashMap<i64, HashMap<(usize, usize), char>> {
  let mut map: HashMap<i64, HashMap<(usize, usize), char>> = HashMap::new();

  for t in 0..tiles.len() {
    let tile = &tiles[t];
    let parts: Vec<_> = tile.split(':').collect();
    let title = parts[0];
    let image = parts[1];

    let re = Regex::new(r"Tile (\d*)").unwrap();

    let mut id = 0;

    for cap in re.captures_iter(title) {
      id = cap[1].parse::<i64>().unwrap();
    }

    let rows: Vec<&str> = image.trim().split('\n').collect();
    let mut image_map: HashMap<(usize, usize), char> = HashMap::new();

    for i in 0..rows.len() {
      let line = rows[i];

      for j in 0..line.len() {
        let c = line.chars().nth(j).unwrap();
        image_map.insert((i, j), c);
      }
    }

    map.insert(id, image_map);

    // println!("{}", image);
    // println!("");
  }

  return map;
}

fn print_map(map: &HashMap<(usize, usize), char>) {
  println!("");
  for i in 0..10 {
    let mut owned: String = "".to_owned();

    for j in 0..10 {
      owned.push_str(&map.get(&(i, j)).unwrap().to_string());
    }

    println!("{}", owned);
  }
  println!("");
}

fn rotate(map: &HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
  let mut new_map: HashMap<(usize, usize), char> = HashMap::new();

  for i in 0..10 {
    for j in 0..10 {
      let c = map.get(&(i, j)).unwrap();
      new_map.insert((j, 10 - 1 - i), *c);
    }
  }

  return new_map;
}

fn flip(map: &HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
  let mut new_map: HashMap<(usize, usize), char> = HashMap::new();

  for i in 0..10 {
    for j in 0..10 {
      let c = map.get(&(i, j)).unwrap();
      new_map.insert((10 - 1 - i, j), *c);
    }
  }

  return new_map;
}

fn get_border(image: &HashMap<(usize, usize), char>, side: usize) -> String {
  let mut owned: String = "".to_owned();

  if side == 0 {
    for j in 0..10 {
      owned.push_str(&image.get(&(0, j)).unwrap().to_string());
    }
  } else if side == 1 {
    for i in 0..10 {
      owned.push_str(&image.get(&(i, 9)).unwrap().to_string());
    }
  } else if side == 2 {
    for j in 0..10 {
      owned.push_str(&image.get(&(9, j)).unwrap().to_string());
    }
  } else {
    for i in 0..10 {
      owned.push_str(&image.get(&(i, 0)).unwrap().to_string());
    }
  }
  return owned;
}

// fn check(image1: &HashMap<(usize, usize), char>, image2: &HashMap<(usize, usize), char>) -> bool {
//   return get_top(image1) == get_top(image2);
// }

pub fn run() {
  println!("Day 20 !");
  let file_name = "./data/test.txt";

  let tiles = utils::read_input_with_empty_lines(file_name);
  let size = (tiles.clone().len() as f64).sqrt();

  let mut map = build_map(tiles);

  let mut images: HashMap<i64, HashMap<(usize, usize), char>> = HashMap::new();
  let mut full_image: HashMap<(i32, i32), i64> = HashMap::new();
  let mut to_check: Vec<(i32, i32, usize)> = Vec::new();

  let mut corners: Vec<i64> = Vec::new();

  let mut current = 0;
  let mut first_corner_id = 0;
  let mut first_matches = Vec::new();

  for (id, image) in map.clone().iter() {
    current += 1;
    println!("NB {}", current);
    let mut nb = 0;

    let mut matches: Vec<(i64, usize)> = Vec::new();

    for i in 0..4 {
      let border_ref = get_border(image, i);

      for (_id, _image) in map.clone().iter() {
        if id == _id {
          continue;
        }

        let mut rot = 0;
        let mut tested = _image.clone();

        while rot < 8 {
          let border_test = get_border(&tested, 0);

          if border_test == border_ref {
            nb += 1;
            matches.push((*_id, i));
            break;
          }

          tested = rotate(&tested);
          if rot == 3 {
            tested = flip(&tested);
          }
          rot += 1;
        }
      }
    }

    if nb == 2 {
      first_corner_id = *id;
      first_matches = matches;
      break;
      // corners.push(*id);
    }
  }

  println!("First corner {}", first_corner_id);
  println!("First matches {:?}", first_matches);

  let (_, mut side) = first_matches[0];

  for i in 0..(size as i32) {
    for j in 0..(size as i32) {}

    side = (side + 1) % 4;

    side = (side - 1) % 4;
  }

  // let mut starting_images: Vec<HashMap<(usize, usize), char>> = Vec::new();

  // let mut rot = 0;
  // let mut tested_image = first_image.clone();

  // // rotate and flip
  // while rot < 8 {
  //   let new_image = tested_image.clone();
  //   starting_images.push(new_image.clone());
  //   tested_image = rotate(&new_image);

  //   if rot == 3 {
  //     tested_image = flip(&tested_image);
  //   }
  //   rot += 1;
  // }

  // for i in 0..starting_images.len() {
  //   let mut map = map.clone();
  //   let mut images: HashMap<i64, HashMap<(usize, usize), char>> = HashMap::new();
  //   let mut full_image: HashMap<(i32, i32), i64> = HashMap::new();
  //   let mut to_check: Vec<(i32, i32, usize)> = Vec::new();

  //   let mut min_i = 0;
  //   let mut max_i = 0;
  //   let mut min_j = 0;
  //   let mut max_j = 0;

  //   let image = &starting_images[i];

  //   // Initializations
  //   println!("ID {}", first_id);

  //   to_check.push((-1, 0, 2));
  //   to_check.push((0, 1, 3));
  //   to_check.push((1, 0, 0));
  //   to_check.push((0, -1, 1));

  //   map.remove(first_id);

  //   // TODO: remove borders
  //   images.insert(*first_id, image.clone());

  //   full_image.insert((0, 0), *first_id);

  //   while to_check.len() > 0 {
  //     // println!("TO_CHECK {:?}", to_check);
  //     let (i, j, side) = to_check.remove(0);
  //     // println!("to_check {}", to_check.len());
  //     // println!("Checking {} {} {}", i, j, side);

  //     let (di, dj) = match side {
  //       0 => (-1, 0),
  //       1 => (0, 1),
  //       2 => (1, 0),
  //       3 => (0, -1),
  //       _ => (0, 0),
  //     };

  //     if !full_image.contains_key(&(i, j)) {
  //       // check if side border matches

  //       let ref_id = full_image.get(&(i + di, j + dj)).unwrap();
  //       let reference = images.get(ref_id).unwrap();

  //       let mut found = (&0, &HashMap::new());
  //       let mut is_found = false;
  //       // println!("REFERENCE {} side {}", ref_id, (side + 2) % 4);
  //       // print_map(reference);

  //       let clone_map = map.clone();
  //       // for all images left
  //       for (id, image) in clone_map.iter() {
  //         let mut rot = 0;
  //         let mut tested_image = image.clone();

  //         // if *id == 2473 {
  //         //   println!("Testing against {} side {}", id, side);
  //         //   print_map(&tested_image);
  //         // }

  //         // rotate and flip
  //         while rot < 8 {
  //           let border_ref = get_border(reference, (side + 2) % 4);
  //           let border_test = get_border(&tested_image, side);

  //           // if *id == 2473 {
  //           //   println!("");
  //           //   println!("TESTED");
  //           //   println!("");
  //           //   println!("Side {} {}", side, (side + 2) % 4);
  //           //   println!("");
  //           //   println!("Border ref");
  //           //   println!("{}", border_ref);
  //           //   println!("Border test");
  //           //   println!("{}", border_test);
  //           //   println!("");
  //           // }

  //           if border_test == border_ref {
  //             found = (id, image);
  //             is_found = true;
  //             break;
  //           }

  //           tested_image = rotate(&tested_image);
  //           if rot == 3 {
  //             tested_image = flip(&tested_image);
  //           }
  //           rot += 1;
  //         }
  //       }

  //       if is_found {
  //         let (id, image) = found;
  //         // println!("FOUND i {} j {} id {}", i, j, id);
  //         if (max_i - min_i) as f64 > size {
  //           // println!("{}", max_i - min_i);
  //           continue;
  //         }
  //         if (max_j - min_j) as f64 > size {
  //           // println!("{}", max_j - min_j);
  //           continue;
  //         }
  //         to_check.push((i - 1, j, 2));
  //         to_check.push((i, j + 1, 3));
  //         to_check.push((i + 1, j, 0));
  //         to_check.push((i, j - 1, 1));

  //         min_i = cmp::min(min_i, i - 1);
  //         max_i = cmp::max(max_i, i + 1);
  //         min_j = cmp::min(min_j, j - 1);
  //         max_j = cmp::max(max_j, j + 1);

  //         map.remove(id);

  //         // TODO: remove borders
  //         images.insert(*id, image.clone());

  //         full_image.insert((i, j), *id);
  //         // break;
  //       }
  //     }

  //     // break;
  //   }

  //   println!("FULL IMAGE {:?}", full_image);
  //   println!("FULL IMAGE {}", full_image.len());
  // }
  // // let mut neighbor_by_id: HashMap<i64, i32> = HashMap::new();
  // // println!("NEIGHBORS {:?}", neighbor_by_id);

  // // let mut result = 1;
  // // for (id, value) in neighbor_by_id.iter() {
  // //   if *value == 2 {
  // //     println!("CORNER");
  // //     result *= id;
  // //   }
  // // }
  // // println!("NEIGHBORS {}", result);
  // // let mut ids: Vec<i64> = map.keys().map(|x| *x).collect();
  // // let n = 1951 as i64;

  // // let first = ids.pop().unwrap();
  // // let second = ids.pop().unwrap();

  // // check(map.get(&first).unwrap(), map.get(&second).unwrap());

  // // let to_print = map.get(&n).unwrap();
}
