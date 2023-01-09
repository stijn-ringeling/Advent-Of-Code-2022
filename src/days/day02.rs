use crate::solution;
use crate::days::common;
use std::cmp::Ordering;


enum RPC{
    ROCK,
    PAPER,
    SCISSORS
}

impl From<char> for RPC{
    fn from(v: char) -> Self {
        match v{
            'A' => Self::ROCK,
            'B' => Self::PAPER,
            'C' => Self::SCISSORS,
            'X' => Self::ROCK,
            'Y' => Self::PAPER,
            'Z' => Self::SCISSORS,
            _ => unimplemented!("Unknown move")
        }
    }
}


impl PartialEq for RPC{
    fn eq(&self, other: &Self) -> bool {
        core::mem::discriminant(self) == core::mem::discriminant(other)
    }
}

impl PartialOrd for RPC{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self{
            RPC::ROCK => {
                match other {
                    RPC::PAPER => Some(Ordering::Less),
                    RPC::SCISSORS => Some(Ordering::Greater),
                    _ => Some(Ordering::Equal)
                }
            },
            RPC::PAPER => {
                match other{
                    RPC::ROCK => Some(Ordering::Greater),
                    RPC::SCISSORS => Some(Ordering::Less),
                    _ => Some(Ordering::Equal)
                }
            },
            RPC::SCISSORS => {
                match other{
                    RPC::ROCK => Some(Ordering::Less),
                    RPC::PAPER => Some(Ordering::Greater),
                    _ => Some(Ordering::Equal)
                }
            }
        }
    }
}

enum OUTCOME{
    WIN,
    LOSE,
    DRAW
}

impl From<char> for OUTCOME{
    fn from(v: char) -> Self {
        match v{
            'X' => Self::LOSE,
            'Y' => Self::DRAW,
            'Z' => Self::WIN,
            _ => unimplemented!("Unkown game outcome")
        }
    }
}

fn get_score(opponent:&RPC, mine:&RPC) -> u32{
    let mut score = 0;
    score += match mine {
        RPC::ROCK => 1,
        RPC::PAPER => 2,
        RPC::SCISSORS => 3
    };
    if mine > opponent{
        score += 6;
    }
    
    if mine == opponent {
        score += 3;
    }
    score
}

fn get_move(opponent:&RPC, outcome:&OUTCOME) -> RPC{
    match opponent{
        RPC::ROCK =>  match outcome{
            OUTCOME::WIN => RPC::PAPER,
            OUTCOME::LOSE => RPC::SCISSORS,
            OUTCOME::DRAW => RPC::ROCK
        },
        RPC::PAPER => match outcome{
            OUTCOME::WIN => RPC::SCISSORS,
            OUTCOME::LOSE => RPC::ROCK,
            OUTCOME::DRAW => RPC::PAPER
        },
        RPC::SCISSORS => match outcome {
            OUTCOME::WIN => RPC::ROCK,
            OUTCOME::LOSE => RPC::PAPER,
            OUTCOME::DRAW => RPC::SCISSORS
        }
    }
}

pub struct Solver{
    filename: String
}

impl Solver{
    pub fn new(filename:String) -> Self{
        Self{filename: filename}
    }
}

impl solution::Solver for Solver {
    fn solve(&self) -> solution::SolutionPair {
        let mut score = 0;
        let mut score2 = 0;
        if let Ok(lines) = common::get_input(&self.filename){
            for line in lines{
                if let Ok(line) = line{
                    let opponent:RPC = line.chars().nth(0).unwrap().into();
                    let mine:RPC = line.chars().nth(2).unwrap().into();
                    let outcome:OUTCOME = line.chars().nth(2).unwrap().into();
                    let mine_part2 = get_move(&opponent, &outcome);
                    score += get_score(&opponent, &mine);
                    score2 += get_score(&opponent, &mine_part2);
                }
            }
        }
        (score.into(), score2.into())
    }
}