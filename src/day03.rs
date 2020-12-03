mod utils;

fn check_slope(lines: Vec<String>, x_step: usize, y_step: usize) -> usize {
  let width = lines[0].len();

  let mut trees = 0;
  let mut x = 0;

  for i in (0..(lines.len() - 1)).step_by(y_step) {
    let step = x + x_step;
    x = if step < width { step } else { step % width };

    let y = i + y_step;
    let mut chars = lines[y].chars();
    let position = chars.nth(x).unwrap();

    if position == '#' {
      trees += 1;
    }
  }
  return trees;
}

pub fn run() {
  println!("Day 03 !");

  let file_name = "./data/03.txt";
  // Ici, si je lis une fois au début, mon Vec va être "vidé" après le premier run...
  // let lines = utils::read_input(file_name);

  let attempts = [[1, 1], [3, 1], [5, 1], [7, 1], [1, 2]];

  let mut result = 1;

  for i in 0..attempts.len() {
    let [x, y] = attempts[i];

    // TODO: trouver le moyen de ne pas lire à chaque fois
    let lines = utils::read_input(file_name);

    let current = check_slope(lines, x, y);

    result *= current
  }

  println!("NB TREES {}", result);
}
