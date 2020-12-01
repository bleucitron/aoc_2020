mod utils;

fn main() {
  println!("Hello, world!");
  let file_name = "./data/01.txt";

  let numbers = utils::read_input(file_name);

  for i in 0..numbers.len() {
    let first = numbers[i];

    for j in i + 1..numbers.len() {
      let second = numbers[j];

      for k in j + 1..numbers.len() {
        let third = numbers[k];

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
