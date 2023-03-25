pub mod data_structs;
pub mod parser;
pub mod ls;

fn main() {
    let (res, zone, veh) = parser::read_file("input/toy1.csv");

    let mut ls = ls::LocalSearch::new(res, zone, veh);

    
}