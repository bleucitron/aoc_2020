use std::fs::File;
use std::io::BufRead;
use std::io::BufReader;

fn main() {
  println!("Hello, world!");
  let file_name = "./data/01.txt";

  let file = File::open(file_name).expect("file not found!");
  let reader = BufReader::new(file);

  let mut numbers = Vec::new();

  for line in reader.lines() {
    let my_string = &line.unwrap();
    let my_int: i32 = my_string.parse().unwrap();

    numbers.push(my_int);
  }

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
