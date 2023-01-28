
use crate::etc::solution::Solver;

pub mod common;
pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;
pub mod day06;
pub mod day07;
pub mod day08;
pub mod day09;
pub mod day10;
pub mod day11;
pub mod day12;
pub mod day13;
pub mod day14;
pub mod day15;
pub mod day16;


pub fn get_solver(day: u8, run_example:bool) -> Box<dyn Solver>{
    let filename = common::get_filename(day, run_example);
    match day{
            1 => Box::new(day01::Solver::new(filename)),
            2 => Box::new(day02::Solver::new(filename)),
            3 => Box::new(day03::Solver::new(filename)),
            4 => Box::new(day04::Solver::new(filename)),
            5 => Box::new(day05::Solver::new(filename)),
            6 => Box::new(day06::Solver::new(filename)),
            7 => Box::new(day07::Solver::new(filename)),
            8 => Box::new(day08::Solver::new(filename)),
            9 => Box::new(day09::Solver::new(filename)),
            10 => Box::new(day10::Solver::new(filename)),
            11 => Box::new(day11::Solver::new(filename)),
            12 => Box::new(day12::Solver::new(filename)),
            13 => Box::new(day13::Solver::new(filename)),
            14 => Box::new(day14::Solver::new(filename)),
            15 => Box::new(day15::Solver::new(filename)),
            16 => Box::new(day16::Solver::new(filename)),
            _ => todo!("Day not implemented (yet)")
    }
}