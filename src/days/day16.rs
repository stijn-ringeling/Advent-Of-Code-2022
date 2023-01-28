use crate::solution;

use super::common::get_input;

pub struct Solver{
    filename: String
}

impl Solver{
    pub fn new(filename: String) -> Solver{
        Solver{filename}
    }
}

impl solution::Solver for Solver{
    fn solve(&self) -> solution::SolutionPair {
        match get_input(&self.filename) {
            Err(_) => println!("Could not read filename: {}", self.filename),
            Ok(lines) => {
                for line in lines{
                    if let Ok(line) = line{
                        
                    }
                }
            }
        }
        (0.into(), 0.into())
    }
}