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
        let mut setup = true;
        let mut p1 = String::new();
        let mut p2 = String::new();
        match get_input(&self.filename){
            Err(_) => print!("Could not load {}", &self.filename),
            Ok(mut lines) => {
                let line = lines.nth(0).unwrap().unwrap();
                let num_cols = line.len() / 4 + 1;
                let mut boat = Boat::new(num_cols);
                let mut boat_p2 = Boat::new(num_cols);
                boat.add_line(&line);
                boat_p2.add_line(&line);
                for line in lines{
                    if let Ok(line) = line{
                        match setup{
                            true => {
                                let num_cols = line.len() / 4 + 1;
                                
                                if num_cols == 1{
                                    setup = false;
                                    continue;
                                }
                                if line.chars().nth(1).unwrap() == '1'{
                                    continue;
                                }
                                boat.add_line(&line);
                                boat_p2.add_line(&line);
                            },
                            false => {
                                let parts: Vec<&str> = line.split(' ').collect();
                                let move_amount: usize = parts[1].parse().expect("Could not parse move as number");
                                let move_from: usize = parts[3].parse().expect("Could not parse from stack as number");
                                let move_to: usize = parts[5].parse().expect("Could not parse to stack as number");
                                for _ in 0..move_amount{
                                    if let Some(c) = boat.pop(move_from-1){
                                        boat.push(move_to-1, c);
                                    }else{
                                        println!("Tried to pop from empty stack at {} in {:?}", move_from, boat);
                                    }
                                   
                                }
                                if let Some(cs) = boat_p2.pop_n(move_from-1, move_amount){
                                    boat_p2.push_n(move_to-1, cs);
                                }else{
                                    println!("Tried to pop more elements than were in the stack");
                                }
                            }
                        }
                        
                    }
                }
                //println!("{:?}", boat);
                for stack in 0..boat.stacks.len(){
                    if let Some(c) = boat.peek(stack){
                        p1.push(c);
                    }
                }
                //println!("{:?}", boat_p2);
                for stack in 0..boat_p2.stacks.len(){
                    if let Some(c) = boat_p2.peek(stack){
                        p2.push(c);
                    }
                }
            }
        }
        (p1.into(), p2.into())
    }
}

#[derive(Debug)]
struct Boat{
    stacks: Vec<Vec<char>>
}

impl Boat{
    fn new(size: usize) -> Boat{
        Boat { stacks: vec![vec![];size] }
    }

    fn push_back(&mut self, stack: usize, c: char){
        self.stacks[stack].insert(0, c);
    }

    fn peek(&self, stack: usize)-> Option<char>{
        self.stacks[stack].last().copied()
    }

    fn push(&mut self, stack: usize, c: char){
        self.stacks[stack].push(c);
    }

    fn push_n(&mut self, stack: usize, mut cs: Vec<char>){
        self.stacks[stack].append(&mut cs);
    }

    fn pop(&mut self, stack: usize) -> Option<char>{
        self.stacks[stack].pop()
    }

    fn pop_n(&mut self, stack: usize, n: usize) -> Option<Vec<char>>{
        let stack_size = self.stacks[stack].len();
        if stack_size < n{
            return None;
        }else{
            return Some(self.stacks[stack].drain(stack_size-n..).collect());
        }
    }

    fn add_line(&mut self, line: &str) {
        let num_cols = line.len() / 4 + 1;
        for idx in 0..num_cols{
            let char  = line.chars().nth(idx*4+1).expect("Could not find character at position");
            //println!("{}", char);
            if char != ' '{
               self.push_back(idx, char);
            }
        }
    }
}