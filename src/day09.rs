mod utils;

pub fn run() {
  println!("Day 09 !");

  let file_name = "./data/09.txt";
  let data = utils::read_input_as_nb(file_name);

  // PART 1
  let mut index: usize = 0;

  for i in 25..data.len() {
    let nb_i = data[i];

    let mut valid = false;

    for j in i - 25..i {
      let nb_j = data[j];

      for k in j + 1..i {
        let nb_k = data[k];

        if nb_j != nb_k && (nb_j + nb_k == nb_i) {
          // println!("{} and {} add up to {}", nb_j, nb_k, nb_i);
          valid = true;
          break;
        }
      }
    }

    if !valid {
      index = i;
    }
  }

  println!("NOT VALID {}", data[index]);

  for j in 0..index {
    let nb_j = data[j];

    let mut step = 0;
    let mut sum = nb_j;

    while sum != data[index] && j + step < data.len() - 1 {
      step += 1;
      sum += data[j + step];
    }

    if sum == data[index] {
      println!("FOUND {} j {} j+step {}", sum, j, j + step);
      let mut suite: Vec<i64> = Vec::new();

      for k in j..j + step {
        suite.push(data[k]);
      }

      let min = suite.iter().min().unwrap();
      let max = suite.iter().max().unwrap();

      println!("RESULT {} ({} + {})", min + max, min, max);
      break;
    }
  }
}
