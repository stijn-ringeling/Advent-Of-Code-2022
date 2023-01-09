mod days;
mod etc;
use days::get_solver;
use etc::solution;
use std::env;
use std::time::Instant;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut run_example = false;
    let days;
    match args.len() {
        1 => {
            println!("This is Stijn's AOC 2022 implementation. Please specify a list of days to run. Optionally specify -e first to run examples");
            return;
        },
        0 => unimplemented!("Should not receive 0 arguments"),
        _ => {
            if args[1] == "-e"{
                run_example = true;
                days = &args[2..];
            }else{
                days = &args[1..];
            }
        }
    }
    let days:Vec<u8> = days.iter().filter_map(|day| match day.parse::<u8>(){
        Ok(val) => Some(val),
        Err(_) => {
            println!("{} is not a valid number", day);
            None
        }
    }).collect();
    for day in days{
        let solver = get_solver(day, run_example);
    
        let start = Instant::now();
        let (p1, p2) = solver.solve();
        let time_elapsed = start.elapsed();
        println!("Day {:02} -- {:?}", day, time_elapsed);
        println!("Part1: {}", p1);
        println!("Part2: {}", p2);
    }
    
}
