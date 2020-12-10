mod utils;

fn count(n: i32) -> i32 {
  if n == 2 {
    1
  } else {
    n - 2 + count(n - 1)
  }
}

pub fn run() {
  println!("Day 10 !");

  let file_name = "./data/10.txt";
  let mut data = utils::read_input_as_nb(file_name);
  data.push(0);

  data.sort_by(|a, b| a.cmp(b));

  let mut one = 0;
  let mut three = 1;

  for i in 0..data.len() - 1 {
    if (data[i + 1] - data[i]) == 1 {
      one += 1;
    } else if i + 3 < data.len() && (data[i + 1] - data[i]) == 3 {
      three += 1;
    }
  }

  let mut i = 0;
  let mut lengths = Vec::new();

  while i < data.len() - 1 {
    let is_one = data[i + 1] - data[i] == 1;

    let mut j = 0;
    loop {
      j += 1;
      if i + j == data.len() || data[i + j] - data[i] != j as i64 {
        break;
      }
    }

    if is_one {
      if j > 1 {
        lengths.push(j);
      }
    }
    i += j;
  }

  println!("Ones {}", one);
  println!("Threes {}", three);
  println!("Lengths {:?}", lengths);

  let mut total: i64 = 1;

  for i in 0..lengths.len() {
    let n = lengths[i] as i32;

    if n == 3 {
      total *= 2;
    } else {
      let v = count(n) as i64;
      total *= v;
    }
  }

  println!("   1) {}", one * three);
  println!("   2) {}", total);
}
