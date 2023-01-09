use std::io::{self, BufRead};
use std::fs::File;

pub fn get_input(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>>{

    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
pub fn get_filename(day:u8, run_example:bool) -> String{
    match run_example{
        true => format!("examples/{:02}.txt", day),
        false => format!("inputs/{:02}.txt", day)
    }
}