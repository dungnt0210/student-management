mod controller;
mod helper;
mod model;
mod view;
type Result<T> = std::result::Result<T, error::Error>;

fn main() {
    println!("Hello, world!");
}
