use std::{env, collections::HashSet};

use crate::solution;

use super::common::get_input;

pub struct Solver{
    filename: String
}

impl Solver{
    pub fn new(filename: String) -> Solver{
        Solver{filename}
    }
}

struct Scanner{
    x: i32,
    y: i32,
    range: i32
}

impl Scanner{
    fn new(x1: i32, y1: i32, x2: i32, y2: i32) -> Scanner{
        Scanner { x: x1, y: y1, range: Scanner::range(x1, y1, x2, y2) }
    }
    fn range(x1: i32, y1: i32, x2: i32, y2: i32) -> i32{
        (y2-y1).abs()+(x2-x1).abs()
    }

    fn get_circumferance(&self) -> Vec<(i32, i32)> {
        //println!("Circumferance for scanner at x={}, y={}, range={}", self.x, self.y, self.range);
        let mut vals: Vec<(i32, i32)> = vec![];
        for offset in -self.range -1..self.range + 2{
            vals.push((self.x - (offset + self.range + 1).abs() , self.y + offset));
            vals.push((self.x + (offset + self.range + 1).abs(), self.y + offset))
        }
        vals
    }
}

impl solution::Solver for Solver{
    fn solve(&self) -> solution::SolutionPair {
        let mut p1 = 0;
        let mut p2: i128 = 0;
        match get_input(&self.filename) {
            Err(_) => println!("Could not read filename: {}", self.filename),
            Ok(lines) => {
                let mut beacons: HashSet<(i32, i32)> = HashSet::new();
                let mut scanners: Vec<Scanner> = Vec::new();
                let mut min_x: i32 = i32::MAX;
                let mut max_x: i32 = i32::MIN;
                for line in lines{
                    if let Ok(line) = line{
                        let parts: Vec<&str> = line.split(' ').collect();
                        let x1 = parts[2];
                        let y1 = parts[3];
                        let x2 = parts[8];
                        let y2 = parts[9];

                        let x1: i32 = x1[2..x1.len()-1].parse().expect("Could not parse x1 as number");
                        let y1: i32 = y1[2..y1.len()-1].parse().expect("Could not parse y1 as number");
                        let x2: i32 = x2[2..x2.len()-1].parse().expect("Could not parse x2 as number");
                        let y2: i32 = y2[2..].parse().expect("Could not parse y2 as number");
                        let scanner = Scanner::new(x1, y1, x2, y2);
                        min_x = min_x.min(scanner.x - scanner.range);
                        max_x = max_x.max(scanner.x + scanner.range);
                        scanners.push(scanner);
                        beacons.insert((x2, y2));
                    }
                }
                const EXAMPLE_LINE: i32 = 10;
                const EXAMPLE_RANGE: i32 = 20;
                const REAL_LINE: i32 = 2000000;
                const REAL_RANGE: i32 = 4000000;
                let scan_line: i32;
                let scan_range: i32;
                match env::var("AOC_EXAMPLE") {
                    Ok(value) => {
                        if value == "1"{
                            scan_line = EXAMPLE_LINE;
                            scan_range = EXAMPLE_RANGE;
                        }else{
                            scan_line = REAL_LINE;
                            scan_range = REAL_RANGE;
                        }
                    },
                    Err(_) => {
                        scan_line = REAL_LINE;
                        scan_range = REAL_RANGE;
                    }
                };
                for x in min_x..max_x+1{
                    if beacons.contains(&(x, scan_line)){
                        continue;
                    }
                    for scanner in &scanners{
                        if Scanner::range(x, scan_line, scanner.x, scanner.y) <= scanner.range{
                            p1 += 1;
                            break;
                        }
                    }
                }
                for scanner  in &scanners{
                    for (x, y) in scanner.get_circumferance(){
                        //println!("Considering x={}, y={}", x, y);
                        if x < 0 || x > scan_range || y < 0 || y > scan_range{
                            continue;
                        }
                        let mut found = true;
                        for scanner2 in &scanners{
                            if Scanner::range(x, y, scanner2.x, scanner2.y) <= scanner2.range{
                                found = false;
                                break;
                            }
                        }
                        if found{
                            //println!("Found point at x={}, y={}", x, y);
                            p2 = (x as i128)*4000000+y as i128;
                            return (p1.into(), p2.into());
                        }
                    }
                }
            }
        }
        (p1.into(), p2.into())
    }
}