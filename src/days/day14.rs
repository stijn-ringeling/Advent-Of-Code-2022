use std::{collections::HashSet, fmt::Display};

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
#[derive(PartialEq)]
enum CellState{
    Free,
    Full
}
#[derive(Debug)]
struct SandMap{
    rocks: HashSet<(u32, u32)>,
    sand: HashSet<(u32, u32)>,
    max_y: u32,
    min_x: u32,
    max_x: u32
}

impl SandMap{
    fn new() -> SandMap{
        SandMap { rocks: HashSet::new(), sand: HashSet::new(), max_y: u32::MIN, min_x: u32::MAX, max_x: u32::MIN }
    }

    fn add_line(&mut self, start_coord: (u32, u32), end_coord: (u32, u32)){
        self.max_y = self.max_y.max(start_coord.1).max(end_coord.1);
        self.min_x = self.min_x.min(start_coord.0).min(end_coord.0);
        self.max_x = self.max_x.max(start_coord.0).max(end_coord.0);
        if start_coord.0 != end_coord.0{
            for i in start_coord.0.min(end_coord.0)..(start_coord.0.max(end_coord.0)+1){
                self.rocks.insert((i, start_coord.1));
            }
        }
        if start_coord.1 != end_coord.1{
            for i in start_coord.1.min(end_coord.1)..(start_coord.1.max(end_coord.1)+1){
                self.rocks.insert((start_coord.0, i));
            }
        }
    }

    fn add_sand(&mut self, sand_coord: (u32, u32)){
        //self.min_x = self.min_x.min(sand_coord.0);
        //self.max_x = self.max_x.max(sand_coord.0);
        //self.max_y = self.max_y.max(sand_coord.1);
        self.sand.insert(sand_coord);
    }

    fn get_cell(&self, coord: (u32, u32)) -> CellState{
        if coord.1 >= self.max_y+2 || self.rocks.contains(&coord) || self.sand.contains(&coord){
            return CellState::Full;
        }
        CellState::Free
    }
}

impl Display for SandMap{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "X: ({},{})\nY: ({},{})", self.min_x, self.max_x, 0, self.max_y)?;
        writeln!(f, "Sand={}", self.sand.len())?;
        for y in 0..self.max_y+1{
            write!(f, "{:03}", y)?;
            for x in self.min_x..self.max_x+1{
                if x==500 && y == 0{
                    write!(f, "+")?;
                    continue;
                }
                if self.rocks.contains(&(x, y)){
                    write!(f, "#")?;
                    if self.sand.contains(&(x,y)){
                        panic!("Rock and sand in same location. This should not be possible")
                    }
                }else{
                    if self.sand.contains(&(x,y)){
                        write!(f,"o")?;
                    }else{
                        write!(f, ".")?;
                    }
                }
            }
            writeln!(f,"")?;
        }
        for _ in self.min_x..self.max_x+1{
            write!(f, "#")?;
        }
        Ok(())
    }
}

impl solution::Solver for Solver{

    fn solve(&self) -> solution::SolutionPair {
        let mut p1 = 0;
        let mut p2 = 0;
        match get_input(&self.filename){
            Err(_) => println!("Could not read file {}", self.filename),
            Ok(lines) => {
                let mut sandmap = SandMap::new();
                for line in lines{
                    if let Ok(line) = line{
                        let corners: Vec<&str> = line.split(" -> ").collect();
                        let mut start_corner = corners[0];
                        for corner in &corners[1..]{
                            let start_coords: Vec<u32> = start_corner.split(',').map(|x| x.parse().expect("Could not parse coordinate as u32")).collect();
                            let end_coords: Vec<u32> = corner.split(',').map(|x| x.parse().expect("Could not parse coord as u32")).collect();
                            sandmap.add_line((start_coords[0], start_coords[1]), (end_coords[0], end_coords[1]));
                            start_corner = corner;
                        }
                    }
                }
                const START_X: u32 = 500;
                const START_Y: u32 = 0;
                loop{
                    let mut sand_x = START_X;
                    let mut sand_y = START_Y;
                    loop{
                        if sand_y >= sandmap.max_y{
                            if p1 == 0{
                                p1 = sandmap.sand.len();
                                println!("{}", sandmap);
                            }
                        }
                        if sandmap.get_cell((sand_x, sand_y+1)) == CellState::Free{
                            sand_y += 1;
                        }else{
                            if sandmap.get_cell((sand_x-1, sand_y+1)) == CellState::Free{
                                sand_x -= 1;
                                sand_y += 1;
                            }else{
                                if sandmap.get_cell((sand_x+1, sand_y+1)) == CellState::Free{
                                    sand_x += 1;
                                    sand_y += 1;
                                }else{
                                    sandmap.add_sand((sand_x, sand_y));
                                    break;
                                }
                            }
                        }
                    }
                    if sandmap.get_cell((START_X, START_Y)) == CellState::Full{
                        p2 = sandmap.sand.len();
                        break;
                    }
                }
                //p1 = sandmap.sand.len();
                println!("{}", sandmap)
            }
        }
        (p1.into(), p2.into())
    }
}