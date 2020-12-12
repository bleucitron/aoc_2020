mod utils;

fn clone(matrix: &Vec<Vec<char>>) -> Vec<Vec<char>> {
  let mut clone: Vec<Vec<char>> = Vec::new();
  for i in 0..matrix.len() {
    let line = matrix[i].clone();

    clone.push(line);
  }
  return clone;
}

pub fn run() {
  let file_name = "./data/11.txt";
  let data = utils::read_input(file_name);
  let len = data.len();
  // day11::run();

  let mut nb_occupied = 0;
  let mut nb_iter = 0;

  let mut continues = true;

  let mut matrix_1: Vec<Vec<char>> = Vec::new();

  for i in 0..len {
    let line = &data[i];

    let mut row: Vec<char> = Vec::new();

    for j in 0..line.len() {
      row.push(line.chars().nth(j).unwrap());
    }

    matrix_1.push(row);
  }

  let mut matrix_2 = clone(&matrix_1);

  // Part 1
  while continues {
    nb_iter += 1;
    let mut matrix_copy = clone(&matrix_1);

    let mut current_nb = 0;

    for i in 0..len {
      let line = &data[i];
      let line_len = line.len();

      for j in 0..line_len {
        if matrix_1[i][j] == '.' {
          continue;
        }

        if matrix_1[i][j] == '#' {
          current_nb += 1;
        }

        let mut nb_around = 0;

        if i > 0 && j > 0 {
          if matrix_1[i - 1][j - 1] == '#' {
            nb_around += 1;
          }
        }
        if i > 0 {
          if matrix_1[i - 1][j] == '#' {
            nb_around += 1;
          }
        }
        if i > 0 && j + 1 < line_len {
          if matrix_1[i - 1][j + 1] == '#' {
            nb_around += 1;
          }
        }
        if j > 0 {
          if matrix_1[i][j - 1] == '#' {
            nb_around += 1;
          }
        }
        if j + 1 < line_len {
          if matrix_1[i][j + 1] == '#' {
            nb_around += 1;
          }
        }
        if i + 1 < len && j > 0 {
          if matrix_1[i + 1][j - 1] == '#' {
            nb_around += 1;
          }
        }
        if i + 1 < len {
          if matrix_1[i + 1][j] == '#' {
            nb_around += 1;
          }
        }
        if i + 1 < len && j + 1 < line_len {
          if matrix_1[i + 1][j + 1] == '#' {
            nb_around += 1;
          }
        }
        // println!("i {} j {}: {}, nb {}", i, j, matrix_1[i][j], nb_around);

        // println!("nb {}", nb_around);
        if matrix_1[i][j] == 'L' && nb_around == 0 {
          matrix_copy[i][j] = '#';
          nb_occupied += 1;
        } else if matrix_1[i][j] == '#' && nb_around >= 4 {
          matrix_copy[i][j] = 'L';
          nb_occupied -= 1;
        }
      }
    }

    matrix_1 = matrix_copy;

    continues = current_nb != nb_occupied;
  }

  println!("Nb iter {}", nb_iter);
  println!("   Part 1: {}", nb_occupied);

  // Part 2

  nb_iter = 0;
  nb_occupied = 0;
  continues = true;

  while continues {
    nb_iter += 1;
    let mut matrix_copy = clone(&matrix_2);

    let mut current_nb = 0;

    for i in 0..len {
      let line = &data[i];
      let line_len = line.len();

      for j in 0..line_len {
        if matrix_2[i][j] == '.' {
          continue;
        }

        if matrix_2[i][j] == '#' {
          current_nb += 1;
        }

        let mut nb_visible = 0;

        for t in &[
          (-1, -1),
          (-1, 0),
          (-1, 1),
          (0, -1),
          (0, 1),
          (1, -1),
          (1, 0),
          (1, 1),
        ] {
          let (ti, tj) = t;

          let mut k = 0;

          loop {
            k += 1;

            let pos_i = i as i32;
            let pos_j = j as i32;
            let length = len as i32;
            let line_length = line_len as i32;

            let check_i = pos_i + ti * k;
            let check_j = pos_j + tj * k;

            if check_i < 0 || check_i == length {
              break;
            }
            if check_j < 0 || check_j == line_length {
              break;
            }

            let seat = matrix_2[check_i as usize][check_j as usize];

            if seat == '#' {
              nb_visible += 1;
              break;
            }
            if seat == 'L' {
              break;
            }
          }
        }

        if matrix_2[i][j] == 'L' && nb_visible == 0 {
          matrix_copy[i][j] = '#';
          nb_occupied += 1;
        } else if matrix_2[i][j] == '#' && nb_visible >= 5 {
          matrix_copy[i][j] = 'L';
          nb_occupied -= 1;
        }
      }
    }

    // println!("CURRENT {} OCCUPIED {}", current_nb, nb_occupied);
    matrix_2 = matrix_copy;
    continues = current_nb != nb_occupied;
  }

  println!("Nb iter {}", nb_iter);
  println!("   Part 2: {}", nb_occupied);
}
