pub mod data_structs;
pub mod parser;
pub mod ls;

fn main() {

    let input_filename = std::env::args().nth(1).expect("No input file given...");
    let output_filename = std::env::args().nth(2).expect("No output file given...");
    let time = std::env::args().nth(3).expect("No time has been given...");
    let seed = std::env::args().nth(4).expect("No seed had been given...");

    let (res, zone, veh) = parser::read_file(&input_filename);

    let mut ls = ls::LocalSearch::new(res, zone, veh);
    ls.run(time.parse::<i32>().expect("No number given as time"), seed.parse::<i32>().expect("No number given as seed"));

    let _ = ls.write_output(&output_filename);
}