use std::collections::HashSet;
mod utils;

pub fn run() {
  println!("Day 06 !");

  let file_name = "./data/06.txt";
  let data = utils::read_input_with_empty_lines(file_name);

  let mut total = 0;

  for i in 0..data.len() {
    let group = &data[i].trim();
    let nb = group.matches("\n").count() + 1;
    let answers = group.replace("\n", "");

    let mut unique_answers = HashSet::new();

    for answer in answers.chars() {
      unique_answers.insert(answer);
    }

    let mut nb_all = 0;

    for unique in unique_answers {
      let current_nb = group.matches(unique).count();
      if nb == current_nb {
        nb_all += 1;
      }
    }

    total += nb_all;
  }

  println!("Total {}", total);
}
