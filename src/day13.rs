mod utils;

pub fn run() {
  let file_name = "./data/13.txt";
  let data = utils::read_input(file_name);

  println!("DATA {:?}", data);
}
