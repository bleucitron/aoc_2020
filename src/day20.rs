mod utils;
use regex::Regex;
// use std::cmp;
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

  let size = (map.clone().len() as f64).sqrt() as usize;
  for i in 0..size {
    let mut owned: String = "".to_owned();

    for j in 0..size {
      owned.push_str(&map.get(&(i, j)).unwrap().to_string());
    }

    println!("{}", owned);
  }
  println!("");
}

fn rotate(map: &HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
  let size = (map.clone().len() as f64).sqrt() as usize;
  let mut new_map: HashMap<(usize, usize), char> = HashMap::new();

  for i in 0..size {
    for j in 0..size {
      let c = map.get(&(i, j)).unwrap();
      new_map.insert((j, size - 1 - i), *c);
    }
  }

  return new_map;
}

fn flip(map: &HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
  let size = (map.clone().len() as f64).sqrt() as usize;
  let mut new_map: HashMap<(usize, usize), char> = HashMap::new();

  for i in 0..size {
    for j in 0..size {
      let c = map.get(&(i, j)).unwrap();
      new_map.insert((size - 1 - i, j), *c);
    }
  }

  return new_map;
}

fn get_border(image: &HashMap<(usize, usize), char>, side: usize) -> String {
  let size = (image.clone().len() as f64).sqrt() as usize;
  let mut owned: String = "".to_owned();

  if side == 0 {
    for j in 0..size {
      owned.push_str(&image.get(&(0, j)).unwrap().to_string());
    }
  } else if side == 1 {
    for i in 0..size {
      owned.push_str(&image.get(&(i, size - 1)).unwrap().to_string());
    }
  } else if side == 2 {
    for j in 0..size {
      owned.push_str(&image.get(&(size - 1, j)).unwrap().to_string());
    }
  } else {
    for i in 0..size {
      owned.push_str(&image.get(&(i, 0)).unwrap().to_string());
    }
  }
  return owned;
}

fn remove_borders(image: &HashMap<(usize, usize), char>) -> HashMap<(usize, usize), char> {
  let size = (image.clone().len() as f64).sqrt() as usize;
  let mut photo = image.clone();

  for j in 0..size {
    photo.remove(&(0, j));
  }
  for i in 0..size {
    photo.remove(&(i, 0));
  }
  for j in 0..size {
    photo.remove(&(size - 1, j));
  }
  for i in 0..size {
    photo.remove(&(i, size - 1));
  }

  return photo.clone();
}

pub fn run() {
  println!("Day 20 !");
  let file_name = "./data/20.txt";

  let tiles = utils::read_input_with_empty_lines(file_name);

  let _map = build_map(tiles);
  let mut map = _map.clone();

  let mut images: HashMap<i64, HashMap<(usize, usize), char>> = HashMap::new();
  let mut id_by_pos: HashMap<(i32, i32), i64> = HashMap::new();
  let mut to_check: Vec<(i32, i32, usize)> = Vec::new();

  let ids: Vec<&i64> = _map.keys().collect();
  let first_id = ids[0];

  to_check.push((-1, 0, 2));
  to_check.push((0, 1, 3));
  to_check.push((1, 0, 0));
  to_check.push((0, -1, 1));

  let first_image = map.remove(first_id).unwrap();

  images.insert(*first_id, first_image.clone());
  id_by_pos.insert((0, 0), *first_id);

  while to_check.len() > 0 {
    let (i, j, side) = to_check.remove(0);

    let (di, dj) = match side {
      0 => (-1, 0),
      1 => (0, 1),
      2 => (1, 0),
      3 => (0, -1),
      _ => (0, 0),
    };

    if !id_by_pos.contains_key(&(i, j)) {
      // check if side border matches

      let ref_id = id_by_pos.get(&(i + di, j + dj)).unwrap();
      let reference = images.get(ref_id).unwrap();

      let mut found = (&0, HashMap::new());
      let mut is_found = false;

      let clone_map = map.clone();
      // for all images left
      for (id, image) in clone_map.iter() {
        let mut rot = 0;
        let mut tested_image = image.clone();

        // rotate and flip
        while rot < 8 {
          let border_ref = get_border(reference, (side + 2) % 4);
          let border_test = get_border(&tested_image, side);

          if border_test == border_ref {
            found = (id, tested_image);
            is_found = true;
            break;
          }

          tested_image = rotate(&tested_image);
          if rot == 3 {
            tested_image = flip(&tested_image);
          }
          rot += 1;
        }
      }

      if is_found {
        let (id, image) = found;

        to_check.push((i - 1, j, 2));
        to_check.push((i, j + 1, 3));
        to_check.push((i + 1, j, 0));
        to_check.push((i, j - 1, 1));

        map.remove(id);

        images.insert(*id, image.clone());

        id_by_pos.insert((i, j), *id);
      }
    }
  }

  let coords = id_by_pos.keys();

  let mut min_i = 0;
  let mut min_j = 0;
  let mut max_i = 0;
  let mut max_j = 0;

  for (i, j) in coords {
    if *i > max_i {
      max_i = *i;
    }
    if *j > max_j {
      max_j = *j;
    }
    if *i < min_i {
      min_i = *i;
    }
    if *j < min_j {
      min_j = *j;
    }
  }

  let mut nb = 1;

  for (i, j) in &[
    (min_i, min_j),
    (min_i, max_j),
    (max_i, min_j),
    (max_i, max_j),
  ] {
    let id = id_by_pos.get(&(*i, *j)).unwrap();

    nb *= id;
  }
  println!("   Part 1: {}", nb);

  let mut full_image: HashMap<(usize, usize), char> = HashMap::new();

  for x in min_i..(max_i + 1) {
    for y in min_j..(max_j + 1) {
      let coords = &(x, y);

      let id = id_by_pos.get(coords).unwrap();
      let image = images.get(id).unwrap();

      let photo = remove_borders(image);

      let size = (photo.clone().len() as f64).sqrt() as i32;

      for i in 0..size {
        for j in 0..size {
          let ij = ((i + 1) as usize, (j + 1) as usize);

          let ch = photo.get(&ij).unwrap();

          let g_i = x - min_i;
          let g_j = y - min_j;

          let global_coord = ((g_i * size + i) as usize, (g_j * size + j) as usize);

          full_image.insert(global_coord, *ch);
        }
      }
    }
  }

  let full_size = (full_image.clone().len() as f64).sqrt() as usize;

  let mut rot = 0;
  let mut nb_x = 0;
  let mut nb_monsters = 0;
  let mut tested_image = full_image.clone();

  // rotate and flip
  while rot < 8 {
    for i in 0..full_size {
      for j in 0..full_size {
        let ch = *tested_image.get(&(i, j)).unwrap();

        if ch == '#' && rot == 0 {
          nb_x += 1;
        }

        if i + 3 < full_size && j + 20 < full_size {
          let block = extract_block(i, j, tested_image.clone());
          if is_monster(block) {
            nb_monsters += 1;
          }
        }
      }
    }

    if nb_monsters > 0 {
      break;
    }

    tested_image = rotate(&tested_image);
    if rot == 3 {
      tested_image = flip(&tested_image);
    }
    rot += 1;
  }

  println!("   Part 2: {}", nb_x - nb_monsters * 15);
}

fn is_monster(s: String) -> bool {
  let positions = &[18, 20, 25, 26, 31, 37, 38, 39, 41, 44, 47, 50, 53, 56];
  let chars: Vec<char> = s.chars().collect();

  let mut answer = 1;

  for position in positions {
    if chars[*position] == '#' {
      answer *= 1;
    } else {
      answer = 0;
      break;
    }
  }
  return answer == 1;
}

fn extract_block(i: usize, j: usize, image: HashMap<(usize, usize), char>) -> String {
  let mut s: String = "".to_owned();

  for di in 0..3 {
    for dj in 0..20 {
      s.push_str(&image.get(&(i + di, j + dj)).unwrap().to_string());
    }
  }

  return s;
}
