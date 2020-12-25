mod utils;

fn step(value: i64, input: i64) -> i64 {
  let mut result = value;

  result = result * input;
  result = result % 20201227;

  return result;
}

fn transform(input: i64, size: i64) -> i64 {
  let mut result = 1;

  for i in 0..size {
    result = step(result, input);
  }

  return result;
}

fn get_loop_size(nb: i64) -> i64 {
  let mut loop_size = 0;

  let mut result = 1;

  loop {
    loop_size += 1;
    result = step(result, 7);

    // println!("RESULT {} NB {}", result, nb);

    if result == nb {
      break;
    }
  }

  return loop_size;
}

pub fn run() {
  println!("Day 25 !");
  let file_name = "./data/25.txt";

  let input = utils::read_input(file_name);

  let card_pk: i64 = input[0].parse().unwrap();
  let door_pk: i64 = input[1].parse().unwrap();

  let card_secret = get_loop_size(card_pk);
  let door_secret = get_loop_size(door_pk);

  let card_enc_key = transform(door_pk, card_secret);
  let door_enc_key = transform(card_pk, door_secret);
  println!("CARD KEY {}", card_enc_key);
  println!("DOOR KEY {}", door_enc_key);
}
