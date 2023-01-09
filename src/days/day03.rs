use crate::solution;

use super::common::get_input;

use itertools::Itertools;

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
            Err(_) => println!("Could not open file {}", &self.filename),
            Ok(lines) => {
                for chunk in lines.chunks(3).into_iter(){
                    let mut group_list: i128 = 0;
                    for line in chunk{
                        if let Ok(line) = line{
                            let bag_contents = parse_compartment(&line);
                            if group_list == 0{
                                group_list = bag_contents;
                            }else{
                                group_list &= bag_contents;
                            }
                            let length = line.len();
                            let compartment1 = &line[..length/2];
                            let compartment2 = &line[length/2..];
    
                            let compartment1 = parse_compartment(compartment1);
                            let compartment2 = parse_compartment(compartment2);
                            let double_item = compartment1&compartment2;
                            let double_item_prio = compartment_to_priority(double_item);
                            p1 += double_item_prio;
                        }
                    }
                    p2 += compartment_to_priority(group_list);
                }
            }
        }
        (p1.into(), p2.into())
    }
}

fn compartment_to_priority(compartment: i128) -> u32{
    compartment.trailing_zeros() + 1
}

fn parse_compartment(items: &str) -> i128{
    let mut compartment:i128 = 0;
    for char in items.chars(){
        compartment |= 1<<(get_priority(char)-1);
    }
    compartment
}

fn get_priority(c: char) -> u32{
    let ascii = c as u32;
    let priority ;
    if ascii > 96{
        priority = ascii - 96;
    }else{
        if ascii > 64{
            priority = ascii - 64 + 26;
        }else{
            println!("Got unexpected ascii character: {}. Setting priority to 0", c);
            priority = 0;
        }
    }
    priority
}