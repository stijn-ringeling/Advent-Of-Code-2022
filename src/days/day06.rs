use std::{fs::File, io::{self, Read}, collections::HashSet};

use crate::solution;

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
        let f = File::open(&self.filename).expect(&format!("Could not open file {}", &self.filename));
        let reader = io::BufReader::new(f);
        let mut bytes: Vec<u8> = Vec::new();
        let mut bytes2: Vec<u8> = Vec::new();
        let mut p1: Option<u32> = None;
        let mut p2: Option<u32> = None;
        let mut idx = 0;
        for byte in reader.bytes(){
            let b = byte.expect("Could not read byte from file");
            bytes.push(b);
            bytes2.push(b);
            idx += 1;
            if bytes.len() == 4{
                let mut uniq: HashSet<u8> = HashSet::new();
                bytes.iter().all(|x| uniq.insert(*x));
                if uniq.len() == 4{
                    p1 = match p1{
                        None => Some(idx),
                        Some(x) => Some(x)
                    };
                }
                bytes.remove(0);
            }
            if bytes2.len() == 14{
                let mut uniq: HashSet<u8> = HashSet::new();
                bytes2.iter().all(|x| uniq.insert(*x));
                if uniq.len() == 14{
                    p2 = match p2 {
                        None => Some(idx),
                        Some(x) => Some(x)
                    }
                }
                bytes2.remove(0);
            }
            if p1.is_some() && p2.is_some() {
                break;
            }
        }
        println!("Found {} at {}", String::from_utf8(bytes).expect("Could not parse bytes as string"), idx);
        (p1.unwrap().into(), p2.unwrap().into())
    }
}