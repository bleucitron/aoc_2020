mod utils;

pub fn run() {
  println!("Day 01 !");
  let file_name = "./data/01.txt";

  let numbers = utils::read_input(file_name);

  for i in 0..numbers.len() {
    let first: i32 = numbers[i].parse().unwrap();

    for j in i + 1..numbers.len() {
      let second: i32 = numbers[j].parse().unwrap();

      for k in j + 1..numbers.len() {
        let third: i32 = numbers[k].parse().unwrap();

        if first + second + third == 2020 {
          println!("{0} + {1} + {2} = 2020", first, second, third);
          println!(
            "{0} * {1} * {2} = {3}",
            first,
            second,
            third,
            first * second * third
          );
          break;
        }
      }
    }
  }
}
