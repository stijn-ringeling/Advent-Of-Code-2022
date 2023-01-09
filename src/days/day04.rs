use crate::solution;

use super::common::get_input;

pub struct Solver{
    filename: String
}

impl Solver{
    pub fn new(filename: String) -> Solver{
        Solver{filename: filename}
    }
}

impl solution::Solver for Solver{
    fn solve(&self) -> solution::SolutionPair {
        let mut p1 = 0;
        let mut p2 = 0;
        match get_input(&self.filename){
            Err(_) => println!("Could not open file {}", self.filename),
            Ok(lines) => {
                for line in lines{
                    if let Ok(line) = line {
                        let ranges = line.split(",");
                        let ranges:Vec<Range> = ranges.map(|r| Range::from(r)).collect();
                        if ranges[0].contains(&ranges[1]) || ranges[1].contains(&ranges[0]){
                            p1 += 1;
                        }
                        if ranges[0].overlaps(&ranges[1]){
                            p2 += 1;
                        }
                    }
                }
            }
        }
        (p1.into(), p2.into())
    }
}

struct Range{
    lower: u32,
    upper: u32
}

impl From<&str> for Range{
    fn from(val: &str) -> Self {
        let numbers: Vec<&str> = val.split("-").collect();
        Range { 
            lower: numbers[0].parse().expect(format!("Could not parse {} as u32", numbers[0]).as_str()),
            upper: numbers[1].parse().expect(format!("Could not parse {} as u32", numbers[1]).as_str()),
        }
    }
}

impl Range{
    fn contains(&self, other:&Range) -> bool{
        return self.lower <= other.lower && self.upper >= other.upper;
    }

    fn overlaps(&self, other:&Range) -> bool{
        let self_overlap_other = (self.lower >= other.lower && self.lower <= other.upper) ||
                                       (self.upper >= other.lower && self.upper <= other.upper);
        let other_overlap_self = (other.lower >= self.lower && other.lower <= self.upper) ||
                                       (other.upper >= self.lower && other.upper <= self.upper);
        self_overlap_other || other_overlap_self
    }
}