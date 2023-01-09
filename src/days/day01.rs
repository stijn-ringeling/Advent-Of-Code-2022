use crate::solution;
use crate::days::common;

pub struct Solver{
    filename: String
}

impl Solver{
    pub fn new(filename: String) -> Solver{
        Solver { filename: filename }
    }
}

impl solution::Solver for Solver{
    fn solve(&self) -> solution::SolutionPair{
        let mut max:u32 = 0;
        let mut values = Vec::new();
        if let Ok(lines) = common::get_input(&self.filename){
            let mut total = 0;
            for line in lines{
                if let Ok(value) = line{
                    let value = value.trim();
                    if value.is_empty(){
                        if total > max{
                            max = total;
                        }
                        values.push(total);
                        total = 0;
                    }else{
                        total += value.parse::<u32>().unwrap_or_default();
                    }
                }
            }
        }else{
            println!("Could not load file {}", &self.filename);
        }
        values.sort_by(|a, b| b.cmp(a));
        let ans2 = match values.len() {
            0 => 0,
            _ => values[0] + values[1] + values[2],
        };
        return (max.into(), ans2.into());
    }
}