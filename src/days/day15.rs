use crate::solution;

pub struct Solver{
    filename: String
}

impl Solver{
    pub fn new(filename: String) -> Solver{
        Solver{filename}
    }
}

impl solution::Solver for Solver{
}