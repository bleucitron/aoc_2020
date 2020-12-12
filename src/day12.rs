use regex::Regex;
use std::f64::consts::PI;
mod utils;

pub fn run() {
  let file_name = "./data/12.txt";
  let data = utils::read_input(file_name);

  let mut orientation = 0;
  let mut pos_x = 0;
  let mut pos_y = 0;

  for i in 0..data.len() {
    let line = &data[i];

    let r = Regex::new(r"([A-Z])(\d*)").unwrap();

    let mut direction: String = "".to_string();
    let mut distance: i64 = 0;

    for cap in r.captures_iter(line) {
      direction = cap[1].to_string();
      distance = cap[2].parse::<i64>().unwrap();
    }

    if direction == "N" {
      pos_y += distance;
    } else if direction == "S" {
      pos_y -= distance;
    } else if direction == "E" {
      pos_x += distance;
    } else if direction == "W" {
      pos_x -= distance;
    } else if direction == "L" {
      orientation = ((orientation + distance) % 360).abs();
    } else if direction == "R" {
      orientation = ((orientation + 360 - distance) % 360).abs();
    } else if direction == "F" {
      if orientation == 0 {
        pos_x += distance;
      }
      if orientation == 90 {
        pos_y += distance;
      }
      if orientation == 180 {
        pos_x -= distance;
      }
      if orientation == 270 {
        pos_y -= distance;
      }
    }
  }

  println!("   Part 1: {}", pos_x.abs() + pos_y.abs());

  let mut wp_x = 10.;
  let mut wp_y = 1.;
  let mut x = 0.;
  let mut y = 0.;

  for i in 0..data.len() {
    let line = &data[i];

    let r = Regex::new(r"([A-Z])(\d*)").unwrap();

    let mut direction: String = "".to_string();
    let mut distance: f64 = 0.;

    for cap in r.captures_iter(line) {
      direction = cap[1].to_string();
      distance = cap[2].parse::<f64>().unwrap();
    }

    if direction == "N" {
      wp_y += distance;
    } else if direction == "S" {
      wp_y -= distance;
    } else if direction == "E" {
      wp_x += distance;
    } else if direction == "W" {
      wp_x -= distance;
    } else if direction == "L" || direction == "R" {
      let angle = if direction == "R" {
        -distance
      } else {
        distance
      };

      let rad = angle * PI / 180.;
      let x2 = wp_x * rad.cos() - wp_y * rad.sin();
      let y2 = wp_y * rad.cos() + wp_x * rad.sin();

      wp_x = x2.round();
      wp_y = y2.round();
    } else if direction == "F" {
      x += distance * wp_x;
      y += distance * wp_y;
    }
  }

  println!("   Part 2: {}", x.abs() + y.abs());
}
