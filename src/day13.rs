use std::f64;
mod utils;

pub fn run() {
  let file_name = "./data/13.txt";
  let data = utils::read_input(file_name);

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
  let mut buses: Vec<(f64, f64)> = Vec::new();

  for i in 0..lines.len() {
    let bus = lines[i];
    if bus == "x" {
      continue;
    }

    let time = bus.parse::<f64>().unwrap();

    buses.push((time, i as f64));
  }

  let mut answer = 0.;
  let mut continues = true;
  let mut step = 1.;
  let mut i = 0;

  while continues && i < buses.len() {
    answer += step;

    let (bus, position) = buses[i];
    let valid = (answer + position) % bus == 0.;

    if !valid {
      continue;
    }
    continues = continues || !valid;

    step *= bus;
    i += 1;
  }

  println!("   Part 2: {}", answer);
}
