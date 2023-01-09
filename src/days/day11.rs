use std::io::{BufReader, Lines};
use std::fs::File;

use anyhow::Result;


use crate::solution;

use super::common::get_input;

pub struct Solver{
    filename: String
}

#[derive(Debug)]
struct Monkey{
    items: Vec<u128>,
    divisor: u128,
    true_monkey: usize,
    false_monkey: usize,
    operation: Vec<String>,
    inspections: u128
}

impl Monkey{
    fn parse(linebuffer: &mut Lines<BufReader<File>>) -> Result<Monkey>{
        let initial_info = linebuffer.next().expect("Could not read monkey info line")?;
        let p = initial_info.split(": ").nth(1).expect("No initial parts given");
        let values: Vec<u128> = p.split(", ").map(|x| x.parse::<u128>().expect("Could not parse value")).collect();
        let operation_info = linebuffer.next().expect("Could not read operation info")?;
        let operation = operation_info.split(": ").nth(1).expect("Could not get operation info").split("= ").nth(1).expect("Could not find operator after = sign").split(" ").map(|x| String::from(x)).collect();
        let divisor_info = linebuffer.next().expect("Could not read divisor info")?;
        let divisor = divisor_info.split(" ").last().expect("Could not find divisor string").parse::<u128>()?;
        let true_info = linebuffer.next().expect("Could not read true line")?;
        let true_monkey = true_info.split(" ").last().expect("Could not find true info").parse::<usize>()?;
        let false_info = linebuffer.next().expect("Could not reaf false info line")?;
        let false_monkey = false_info.split(" ").last().expect("Could not find false monkey info").parse::<usize>()?;

        Ok(Monkey { items: values, divisor, true_monkey, false_monkey, operation, inspections: 0 })
    }
}

impl Monkey{

    fn apply_op(&self, item: u128) -> u128{
        let p1 = match self.operation[0].as_str(){
            "old" => item,
            _ => self.operation[0].parse::<u128>().expect("Could not parse operator as number")
        };
        let p2 = match self.operation[2].as_str(){
            "old" => item,
            _ => self.operation[2].parse::<u128>().expect("Could not parse final operator")
        };
        match self.operation[1].as_str(){
            "+" => p1 + p2,
            "*" => p1 * p2,
            "-" => p1 - p2,
            _ => todo!("Implement operator {}", self.operation[1])
        }
    }
}

impl Solver{
    pub fn new(filename: String) -> Solver{
        Solver{filename}
    }
}

impl solution::Solver for Solver{
    fn solve(&self) -> solution::SolutionPair {
        let mut p1:u128 = 0;
        match get_input(&self.filename){
            Err(_) => println!("Could not read file {}", self.filename),
            Ok(mut lines) => {
                let mut monkey_modulo = 1;
                let mut monkeys: Vec<Monkey> = Vec::new();
                loop {
                    match lines.next() {
                        Some(_) => {
                            match Monkey::parse(&mut lines){
                                Ok(m) => {
                                    //println!("{:?}", m);
                                    monkey_modulo *= m.divisor;
                                    monkeys.push(m);
                                },
                                Err(e) => panic!("{}", e)
                            }
                        },
                        None => break,
                    }
                    lines.next();
                }
                for _ in 0..10000{
                    //println!("Iteration {}", round);
                    for idx in 0..monkeys.len(){
                        for item_idx in 0..monkeys[idx].items.len(){
                            monkeys[idx].inspections += 1;
                            let new_value = monkeys[idx].apply_op(monkeys[idx].items[item_idx]);
                            if new_value % monkeys[idx].divisor == 0{
                                let true_idx = monkeys[idx].true_monkey;
                                monkeys.split_at_mut(true_idx).1[0].items.push(new_value % monkey_modulo);
                            }else{
                                let false_idx = monkeys[idx].false_monkey;
                                monkeys.split_at_mut(false_idx).1[0].items.push(new_value % monkey_modulo);
                            }
                        }
                        monkeys[idx].items.clear();
                    }
                }
                monkeys.sort_by(|x, y| y.inspections.cmp(&x.inspections));
                println!("{:?}, {:?}", monkeys[0], monkeys[1]);
                p1 = monkeys[0].inspections * monkeys[1].inspections;
            }
        }
        (111210.into(), p1.into())
    }
}