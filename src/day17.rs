mod utils;
use std::collections::HashMap;
use std::iter::FromIterator;

fn display2d(
  universe: &HashMap<(i32, i32, i32), char>,
  x: (i32, i32),
  y: (i32, i32),
  z: (i32, i32),
) {
  let (x_min, x_max) = x;
  let (y_min, y_max) = y;
  let (z_min, z_max) = z;

  for z in z_min..(z_max + 1) {
    println!("");
    for x in x_min..(x_max + 1) {
      let mut line: Vec<char> = Vec::new();

      for y in y_min..(y_max + 1) {
        match universe.get(&(x, y, z)) {
          Some(c) => {
            line.push(*c);
          }
          None => line.push('.'),
        }
      }
      println!("{}", String::from_iter(&line));
    }
    println!("");
  }
}

fn count(universe: &HashMap<(i32, i32, i32), char>) {
  let mut nb = 0;

  for (_, c) in universe.iter() {
    if *c == '#' {
      nb += 1;
    }
  }

  println!("NB active {}", nb)
}

fn count4d(universe: &HashMap<(i32, i32, i32, i32), char>) {
  let mut nb = 0;

  for (_, c) in universe.iter() {
    if *c == '#' {
      nb += 1;
    }
  }

  println!("NB active {}", nb)
}

pub fn run() {
  part1();
  part2();
}

fn part2() {
  println!("PART 2");

  let file_name = "./data/test.txt";
  let data = utils::read_input(file_name);

  let mut universe: HashMap<(i32, i32, i32, i32), char> = HashMap::new();

  let mut x_min = 0;
  let mut y_min = 0;
  let mut z_min = 0;
  let mut w_min = 0;
  let mut x_max = 0;
  let mut y_max = 0;
  let mut z_max = 0;
  let mut w_max = 0;

  let line_len = data[0].len();

  for x in 0..data.len() {
    let chars: Vec<char> = data[x].chars().collect();
    let _x = x as i32;
    x_max += 1;

    for y in 0..line_len {
      y_max += 1;

      let _y = y as i32;

      universe.insert((_x, _y, 0, 0), chars[y]);
    }
  }

  y_max = y_max / x_max;

  let mut turn = 1;

  while turn <= 6 {
    let mut new_universe = universe.clone();

    for x in (x_min - 1)..(x_max + 2) {
      for y in (y_min - 1)..(y_max + 2) {
        for z in (z_min - 1)..(z_max + 2) {
          for w in (w_min - 1)..(w_max + 2) {
            // Get position
            let mut position = '.';

            match universe.get(&(x, y, z, w)) {
              Some(c) => {
                position = *c;
              }
              None => {}
            }

            // Check 80 neighbors

            let mut neighbors = 0;

            for i in &[-1, 0, 1] {
              for j in &[-1, 0, 1] {
                for k in &[-1, 0, 1] {
                  for l in &[-1, 0, 1] {
                    if i == &0 && j == &0 && k == &0 && l == &0 {
                      continue;
                    } else {
                      let mut neighbor = '.';

                      match universe.get(&(x + i, y + j, z + k, w + l)) {
                        Some(n) => {
                          neighbor = *n;
                        }
                        None => {}
                      }

                      if neighbor == '#' {
                        neighbors += 1;
                      } else {
                        continue;
                      }
                    }
                  }
                }
              }
            }

            // Change state according to active neighbor numbers
            if position == '#' && !(neighbors == 2 || neighbors == 3) {
              new_universe.insert((x, y, z, w), '.');
            } else if position == '.' && neighbors == 3 {
              new_universe.insert((x, y, z, w), '#');
            } else {
            }
          }
        }
      }
    }

    x_min -= 1;
    y_min -= 1;
    z_min -= 1;
    w_min -= 1;

    x_max += 1;
    y_max += 1;
    z_max += 1;
    w_max += 1;

    universe = new_universe.clone();
    turn += 1;
  }

  count4d(&universe);
}

fn part1() {
  println!("PART 1");
  let file_name = "./data/17.txt";
  let data = utils::read_input(file_name);

  let mut universe: HashMap<(i32, i32, i32), char> = HashMap::new();

  let mut x_min = 0;
  let mut y_min = 0;
  let mut z_min = 0;
  let mut x_max = 0;
  let mut y_max = 0;
  let mut z_max = 0;

  let line_len = data[0].len();

  for x in 0..data.len() {
    let chars: Vec<char> = data[x].chars().collect();
    let _x = x as i32;
    x_max += 1;

    for y in 0..line_len {
      y_max += 1;

      let _y = y as i32;

      universe.insert((_x, _y, 0), chars[y]);
    }
  }

  y_max = y_max / x_max;

  let mut turn = 1;

  while turn <= 6 {
    let mut new_universe = universe.clone();

    for x in (x_min - 1)..(x_max + 2) {
      for y in (y_min - 1)..(y_max + 2) {
        for z in (z_min - 1)..(z_max + 2) {
          // Get position
          let mut position = '.';

          match universe.get(&(x, y, z)) {
            Some(c) => {
              position = *c;
            }
            None => {}
          }

          // Check 26 neighbors

          let mut neighbors = 0;

          for i in &[-1, 0, 1] {
            for j in &[-1, 0, 1] {
              for k in &[-1, 0, 1] {
                if i == &0 && j == &0 && k == &0 {
                  continue;
                } else {
                  let mut neighbor = '.';

                  match universe.get(&(x + i, y + j, z + k)) {
                    Some(n) => {
                      neighbor = *n;
                    }
                    None => {}
                  }

                  if neighbor == '#' {
                    neighbors += 1;
                  } else {
                    continue;
                  }
                }
              }
            }
          }

          // Change state according to active neighbor numbers
          if position == '#' && !(neighbors == 2 || neighbors == 3) {
            new_universe.insert((x, y, z), '.');
          } else if position == '.' && neighbors == 3 {
            new_universe.insert((x, y, z), '#');
          } else {
          }
        }
      }
    }

    x_min -= 1;
    y_min -= 1;
    z_min -= 1;

    x_max += 1;
    y_max += 1;
    z_max += 1;

    universe = new_universe.clone();
    turn += 1;

    display2d(
      &universe.clone(),
      (x_min, x_max),
      (y_min, y_max),
      (z_min, z_max),
    );
  }
  count(&universe);
}
