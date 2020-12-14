use std::f64;
mod utils;

fn check_time(timestamp: f64, time: f64) -> f64 {
  return (time - timestamp % time) % time;
}

pub fn run() {
  let file_name = "./data/test.txt";
  let data = utils::read_input(file_name);

  // println!("DATA {:?}", data);

  let estimate = data[0].parse::<f64>().unwrap();
  let lines: Vec<&str> = data[1].split(',').collect();

  // Part 1
  let mut wait_time = f64::INFINITY;
  let mut bus_nb = 0.;

  for i in 0..lines.len() {
    let bus = lines[i];
    if bus == "x" {
      continue;
    }

    let time = bus.parse::<f64>().unwrap();

    let cur_wait_time = time - estimate % time;
    if cur_wait_time < wait_time {
      wait_time = cur_wait_time;
      bus_nb = time;
    }
  }
  println!("   Part 1: {}", wait_time * bus_nb);

  // Part 2

  // let first_bus = lines[0].parse::<f64>().unwrap();
  // let start = 100000000000000.;

  let mut buses: Vec<(f64, f64)> = Vec::new();

  for i in 0..lines.len() {
    let bus = lines[i];
    if bus == "x" {
      continue;
    }

    let time = bus.parse::<f64>().unwrap();

    buses.push((time, i as f64));
  }

  println!("{:?}", buses);

  let mut answer = 0.;
  // let mut answer = 100000000000000.;

  // for i in 0..buses.len() {
  //   let (nb, _pos) = buses[i];
  //   answer *= nb;
  // }

  // println!("ANSWER {}", answer);

  let mut continues = true;
  let mut step = 1.;
  let mut i = 0;

  while continues && i < buses.len() {
    // let mut temp_continues = false;
    answer += step;
    println!("ANSWER {} step {}", answer, step);
    // println!("STEP {}", step);

    // for i in start..buses.len() {
    let (bus, position) = buses[i];

    let offset = check_time(answer, bus);
    let valid = offset == position as f64;
    continues = continues || !valid;

    if !valid {
      continue;
    }

    // println!("ITER {}", nb_iter);
    step *= bus;
    i += 1;
    println!(
      "CURRENT {} BUS {} POSITION {} STEP {}",
      answer, bus, position, step
    );
    // println!("BUS {} POSITION {}", bus, position);
    // println!("OFFSET {}", offset);
    // println!("STEP {}", step);
    // continues = temp_continues;
  }

  // println!("   Nb: {}", nb);
  println!("   Part 2: {}", answer);
}
