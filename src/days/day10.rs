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

struct CPU{
    x: i32,
    cycle: i32
}

impl CPU{
    fn get_score(&self) -> i32{
        
        /*if self.cycle % 40 == 0{
            println!();
        }*/
        if (self.cycle - 20) % 40 == 0 {
            let s = self.x * self.cycle;
            //println!("Cycle: {}. X={}. Score={}", self.cycle, self.X, s);
            return s;
        }
        0
    }

    fn crt_display(&self){
        if (self.cycle -1) % 40 == 0{
            println!();
        }
        //print!("Cycle: {}, X: {} ", self.cycle, self.x);
        if ((self.cycle - 1) % 40 - self.x).abs() <= 1{ //Sprite visible
            print!("#");
        }else{
            print!(".");
        }
    }
}

impl solution::Solver for Solver{
    fn solve(&self) -> solution::SolutionPair {
        let mut p1 = 0;
        match get_input(&self.filename){
            Err(_) => println!("Could not read filename {}", self.filename),
            Ok(lines) => {
                let mut cpu = CPU{x: 1, cycle: 0};
                for line in lines{
                    if let Ok(line) = line{
                        if line == "noop"{
                            cpu.cycle += 1;
                            p1 += cpu.get_score();
                            cpu.crt_display();
                        }else{
                            let parts: Vec<&str> = line.split(' ').collect();
                            cpu.cycle += 1;
                            p1 += cpu.get_score();
                            cpu.crt_display();
                            cpu.cycle += 1;
                            p1 += cpu.get_score();
                            cpu.crt_display();
                            cpu.x += parts[1].parse::<i32>().expect("Could not parse addition");
                        }
                        
                    }
                }
            }
        }
        println!();
        (p1.into(), 0.into())
    }
}