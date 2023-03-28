pub mod data_structs;
pub mod parser;
pub mod ls;

fn main() {
    let (res, zone, veh) = parser::read_file("input/360_5_71_25.csv");

    let mut ls = ls::LocalSearch::new(res, zone, veh);
    ls.run(5);

    let _ = ls.write_output("test.csv");
}