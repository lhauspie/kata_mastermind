pub mod service;

fn main() {
  println!("Hello, world!");
  println!("{:?}", service::evaluate(vec!(), vec!()));
}
