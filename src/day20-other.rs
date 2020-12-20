fn get_all_borders(image: &HashMap<(usize, usize), char>) -> Vec<String> {
  let mut borders: Vec<String> = Vec::new();

  borders.push(get_top(image));

  let r1 = rotate(image);
  borders.push(get_top(&r1));

  let r2 = rotate(image);
  borders.push(get_top(&r2));

  let r3 = rotate(image);
  borders.push(get_top(&r3));

  let flip_1 = flip_x(image);
  borders.push(get_top(&flip_1));

  let f1 = rotate(&flip_1);
  borders.push(get_top(&f1));

  let f2 = rotate(&flip_1);
  borders.push(get_top(&f2));

  let f3 = rotate(&flip_1);
  borders.push(get_top(&f3));

  let flip_2 = flip_y(image);
  borders.push(get_top(&flip_2));

  let f4 = rotate(&flip_2);
  borders.push(get_top(&f4));

  let f5 = rotate(&flip_2);
  borders.push(get_top(&f5));

  let f6 = rotate(&flip_2);
  borders.push(get_top(&f6));

  let flip_3 = flip_x(&flip_y(image));
  borders.push(get_top(&flip_3));

  let f7 = rotate(&flip_3);
  borders.push(get_top(&f7));

  let f8 = rotate(&flip_3);
  borders.push(get_top(&f8));

  let f9 = rotate(&flip_3);
  borders.push(get_top(&f9));

  return borders;
}


for (id, image) in map.iter() {
    println!("CEHCKIN {}", id);
    let borders = get_all_borders(image);
    for (_id, _image) in map.iter() {
      if id != _id {
        let _borders = get_all_borders(_image);

        let mut matched = false;

        for i in 0..borders.len() {
          for j in 0.._borders.len() {
            if borders[i] == _borders[j] {
              match neighbor_by_id.clone().get(id) {
                Some(v) => {
                  neighbor_by_id.insert(*id, v + 1);
                }
                None => {
                  neighbor_by_id.insert(*id, 1);
                }
              }

              matched = true;
              break;
            }
          }

          if matched {
            break;
          }
        }
      }
    }
  }
