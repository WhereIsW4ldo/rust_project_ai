pub mod data_structs;
pub mod parser;

use std::env;

fn main() {
    parser::read_file("input/toy1.csv")
}
