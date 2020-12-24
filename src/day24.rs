use std::collections::HashMap;
mod utils;

pub fn run() {
  println!("Day 24 !");
  let file_name = "./data/24.txt";

  let input = utils::read_input(file_name);

  let mut grid = HashMap::new();

  for i in 0..input.len() {
    let mut steps = input[i].to_string();

    let mut coords = (0, 0, 0);

    while steps.len() > 0 {
      let mut step: String = steps.remove(0).to_string().to_owned();

      if step == "s" || step == "n" {
        step.push_str(&steps.remove(0).to_string());
      }

      let (a, b, c) = coords;

      if step == "e" {
        coords = (a + 1, b, c - 1);
      }
      if step == "ne" {
        coords = (a + 1, b - 1, c);
      }
      if step == "nw" {
        coords = (a, b - 1, c + 1);
      }
      if step == "w" {
        coords = (a - 1, b, c + 1);
      }
      if step == "sw" {
        coords = (a - 1, b + 1, c);
      }
      if step == "se" {
        coords = (a, b + 1, c - 1);
      }
    }

    match grid.get(&coords) {
      Some(v) => {
        grid.insert(coords, !v);
      }
      None => {
        grid.insert(coords, true);
      }
    }
  }

  let mut nb = 0;

  for value in grid.values() {
    if *value {
      nb += 1;
    }
  }

  println!("   Part 1: {}", nb);

  let mut grid_copy = grid.clone();

  for i in 0..100 {
    for (coords, value) in grid_copy.iter() {
      let (a, b, c) = coords;

      let mut neighbors = 0;

      // check all neighbors
      for (da, db, dc) in &[
        (1, 0, -1),
        (1, -1, 0),
        (0, -1, 1),
        (-1, 0, 1),
        (-1, 1, 0),
        (0, 1, -1),
      ] {
        match grid_copy.get(&(a + da, b + db, c + dc)) {
          Some(v) => {
            if *v {
              neighbors += 1;
            }
          }
          None => {
            // new tile, check if needs to be flipped to black
            let mut _neighbors = 0;

            for (_da, _db, _dc) in &[
              (1, 0, -1),
              (1, -1, 0),
              (0, -1, 1),
              (-1, 0, 1),
              (-1, 1, 0),
              (0, 1, -1),
            ] {
              match grid_copy.get(&(a + da + _da, b + db + _db, c + dc + _dc)) {
                Some(v) => {
                  if *v {
                    _neighbors += 1;
                  }
                }
                None => {}
              }
            }

            if _neighbors == 2 {
              grid.insert((a + da, b + db, c + dc), true);
            } else {
              grid.insert((a + da, b + db, c + dc), false);
            }
          }
        }
      }

      if *value {
        if neighbors == 0 || neighbors > 2 {
          grid.insert(*coords, false);
        }
      } else {
        if neighbors == 2 {
          grid.insert(*coords, true);
        }
      }
    }

    grid_copy = grid.clone();
  }

  let mut total = 0;
  for v in grid.values() {
    if *v {
      total += 1;
    }
  }

  println!("   Part 2: {}", total);
}
