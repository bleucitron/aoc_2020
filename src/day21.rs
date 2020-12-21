use std::collections::HashMap;
use std::collections::HashSet;
use std::iter::FromIterator;
mod utils;

pub fn run() {
  println!("Day 21 !");
  let file_name = "./data/21.txt";

  let input = utils::read_input(file_name);

  let mut allergen_map: HashMap<String, HashSet<String>> = HashMap::new();
  let mut all_ingredients: HashSet<String> = HashSet::new();

  let mut foods: Vec<(Vec<String>, Vec<String>)> = Vec::new();

  for i in 0..input.len() {
    let line = &input[i];

    let parts: Vec<&str> = line.split("(contains").collect();

    let ingredients_string = parts[0].trim().to_string();
    let mut allergens_string = parts[1].trim().to_string();

    allergens_string.pop();

    let ingredients: Vec<String> = ingredients_string
      .split(' ')
      .map(|x| x.to_string())
      .collect();
    let allergens: Vec<String> = allergens_string
      .split(", ")
      .map(|x| x.to_string())
      .collect();

    foods.push((ingredients.clone(), allergens.clone()));

    let ingredients_set = HashSet::from_iter(ingredients.iter().cloned());
    all_ingredients = all_ingredients
      .union(&ingredients_set)
      .map(|x| x.to_string())
      .collect();

    for j in 0..allergens.len() {
      let allergen = allergens[j].clone();

      match allergen_map.get(&allergen) {
        Some(s) => {
          let inter: HashSet<String> = s
            .intersection(&ingredients_set)
            .map(|x| x.to_string())
            .collect();
          allergen_map.insert(allergen.clone(), inter);
        }
        None => {
          allergen_map.insert(allergen.clone(), ingredients_set.clone());
        }
      }
    }
  }
  // println!("ALLERGEN MAP {:?}", allergen_map);
  // println!("All ingredients {:?}", all_ingredients);
  let mut ok_ingredients = all_ingredients.clone();

  for set in allergen_map.values() {
    ok_ingredients = ok_ingredients
      .difference(&set)
      .map(|x| x.to_string())
      .collect()
  }
  // println!("OK ingredients {:?}", ok_ingredients);

  let mut nb = 0;
  for i in 0..foods.len() {
    let (ings, _alls) = &foods[i];

    for j in 0..ings.len() {
      let ing = &ings[j];

      if ok_ingredients.contains(ing) {
        nb += 1;
      }
    }
  }
  println!("   Part 1: {}", nb);

  let mut map: HashMap<String, String> = HashMap::new();

  while allergen_map.len() > 0 {
    for (a, mut i) in allergen_map.clone() {
      if i.len() == 1 {
        allergen_map.remove(&a);

        let ings: Vec<String> = i.drain().collect();
        let ing = ings[0].clone();

        for (a, mut values) in allergen_map.clone() {
          values.remove(&ing);
          allergen_map.insert(a, values);
        }
        map.insert(ing, a);
      }
    }
  }

  let mut v: Vec<(String, String)> = Vec::new();

  for (i, a) in map {
    v.push((i, a));
  }

  v.sort_by(|(_a, a_a), (_b, b_a)| a_a.cmp(b_a));

  let mut ing_list: Vec<String> = Vec::new();
  for i in 0..v.len() {
    let (ing, _all) = &v[i];
    ing_list.push(ing.clone());
  }

  println!("   Part 2: {}", ing_list.join(","));
}
