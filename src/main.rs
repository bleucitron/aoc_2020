// #[path = "./day11.rs"]
// mod day11;
mod utils;

fn main() {
  let file_name = "./data/test.txt";
  let data = utils::read_input_as_nb(file_name);
  // day11::run();

  println!("DATA {:?}", data);
}
