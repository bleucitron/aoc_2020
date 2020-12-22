use std::collections::HashSet;
mod utils;

fn part1(p1: Vec<i64>, p2: Vec<i64>) {
  let mut player1 = p1.clone();
  let mut player2 = p2.clone();

  while player1.len() > 0 && player2.len() > 0 {
    let card1 = player1.remove(0);
    let card2 = player2.remove(0);

    if card1 > card2 {
      player1.push(card1);
      player1.push(card2);
    } else {
      player2.push(card2);
      player2.push(card1);
    }
  }

  let winner = if player1.len() > player2.len() {
    player1
  } else {
    player2
  };

  let nb = winner.len();
  let mut score = 0;

  for i in 0..nb {
    let card = winner[i];
    score += ((nb - i) as i64) * card;
  }
  println!("   Part 1: {}", score);
}

fn make_string(v1: Vec<i64>, v2: Vec<i64>) -> String {
  let s1: Vec<String> = v1.iter().map(|x| x.to_string()).collect();
  let s2: Vec<String> = v2.iter().map(|x| x.to_string()).collect();

  let _s1 = s1.join("-");
  let _s2 = s2.join("-");

  return [_s1, _s2].join("/");
}

fn game(p1: Vec<i64>, p2: Vec<i64>) -> (usize, Vec<i64>, Vec<i64>) {
  let mut player1 = p1.clone();
  let mut player2 = p2.clone();

  let mut rounds: HashSet<String> = HashSet::new();

  let mut s = make_string(p1, p2);

  while player1.len() > 0 && player2.len() > 0 {
    if rounds.contains(&s) {
      return (1, player1, player2);
    }

    rounds.insert(s.clone());

    let card1 = player1.remove(0);
    let card2 = player2.remove(0);

    let l1 = player1.len() as i64;
    let l2 = player2.len() as i64;

    if card1 <= l1 && card2 <= l2 {
      let nb1 = card1 as usize;
      let nb2 = card2 as usize;

      let new1 = player1[..nb1].to_vec();
      let new2 = player2[..nb2].to_vec();

      let (winner, _p1, _p2) = game(new1, new2);

      if winner == 1 {
        player1.push(card1);
        player1.push(card2);
      } else {
        player2.push(card2);
        player2.push(card1);
      }
    } else {
      if card1 > card2 {
        player1.push(card1);
        player1.push(card2);
      } else {
        player2.push(card2);
        player2.push(card1);
      }
    }

    s = make_string(player1.clone(), player2.clone());
  }

  let winner = if player1.len() > player2.len() { 1 } else { 2 };

  return (winner, player1, player2);
}

fn part2(p1: Vec<i64>, p2: Vec<i64>) {
  let (winner, end1, end2) = game(p1, p2);

  let cards = if winner == 1 { end1 } else { end2 };
  let nb = cards.len();
  let mut score = 0;

  for i in 0..nb {
    let card = cards[i];
    score += ((nb - i) as i64) * card;
  }
  println!("   Part 2: {}", score);
}

pub fn run() {
  println!("Day 22 !");
  let file_name = "./data/22.txt";

  let input = utils::read_input_with_empty_lines(file_name);

  let mut input1: Vec<String> = input[0].trim().split('\n').map(|x| x.to_string()).collect();
  let mut input2: Vec<String> = input[1].trim().split('\n').map(|x| x.to_string()).collect();

  input1.remove(0);
  input2.remove(0);

  let player1: Vec<i64> = input1.iter().map(|x| x.parse::<i64>().unwrap()).collect();
  let player2: Vec<i64> = input2.iter().map(|x| x.parse::<i64>().unwrap()).collect();

  part1(player1.clone(), player2.clone());
  part2(player1.clone(), player2.clone());
}
