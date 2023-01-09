use std::collections::HashSet;

use crate::solution;

use super::common::get_input;

pub struct Solver{
    filename: String
}
#[derive(Clone, Copy)]
enum Direction{
    Right,
    Left,
    Up,
    Down
}

struct Rope2{
    knots: Vec<RopeEnd>
}

struct Rope{
    head: RopeEnd,
    tail: RopeEnd
}

impl Rope2{
    fn new() -> Rope2{
        let mut knots = Vec::with_capacity(10);
        for _ in 0..10{
            knots.push(RopeEnd::new());
        }
        Rope2 { knots: knots }
    }

    fn move_rope(&mut self, dir: Direction, amount: u32){
        for _ in 0..amount{
            self.knots[0].move_dir(dir);
            for idx in 1..self.knots.len(){
                let x = self.knots.split_at_mut(idx);
                x.1[0].move_towards(x.0.last().expect("No previous kniot to move to"));
            }
        }
    }
}

impl Rope{
    fn new() -> Rope{
        Rope { head: RopeEnd::new(), tail: RopeEnd::new() }
    }

    fn move_rope(&mut self, dir: Direction, amount: u32){
        for _ in 0..amount{
            self.head.move_dir(dir);
            self.tail.move_towards(&self.head);
            //println!("H: {}, {}; T: {}, {}", self.head.x, self.head.y, self.tail.x, self.tail.y);
        }
    }
}

struct RopeEnd{
    x: i32,
    y: i32,
    visit_map: HashSet<(i32, i32)>
}

impl RopeEnd{
    fn new() -> RopeEnd{
        let mut end = RopeEnd { x: 0, y: 0, visit_map: HashSet::new() };
        end.update_visit();
        return end;
    }

    fn move_dir(&mut self, dir: Direction){
        match dir{
            Direction::Up => self.y += 1,
            Direction::Down => self.y -= 1,
            Direction::Right => self.x += 1,
            Direction::Left => self.x -= 1,
        }
            self.update_visit();
        
    }

    fn move_dx_dy(&mut self, dx: i32, dy: i32){
        self.x += dx;
        self.y += dy;
        self.update_visit();
    }

    fn update_visit(&mut self){
        self.visit_map.insert((self.x, self.y));
    }

    fn move_towards(&mut self, other: &RopeEnd){
        if (self.x - other.x).abs() > 1 || (self.y - other.y).abs() > 1{
            let mut min_dx = 0;
            let mut min_dy = 0;
            let mut min_dist = 1000;
            for dx in -1..2{
                for dy in -1..2{
                    if dx == 0 && dy == 0 {
                        continue
                    }
                    let dist = (self.x + dx - other.x).pow(2) + (self.y + dy - other.y).pow(2);
                    if dist < min_dist{
                        min_dx = dx;
                        min_dy = dy;
                        min_dist = dist;
                    }
                }
            }
            self.move_dx_dy(min_dx, min_dy);
        }
    }
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
            Err(_) => println!("Could not read file {}", self.filename),
            Ok(lines) => {
                let mut rope = Rope::new();
                let mut rope2 = Rope2::new();
                for line in lines{
                    if let Ok(line) = line{
                        let parts: Vec<&str> = line.split(' ').collect();
                        let dir = match parts[0]{
                            "R" => Direction::Right,
                            "L" => Direction::Left,
                            "U" => Direction::Up,
                            "D" => Direction::Down,
                            _ => panic!("Unknown direction string {}", parts[0])
                        };
                        let amount: u32 = parts[1].parse().expect("Could not parse amount of steps as u32");
                        rope.move_rope(dir, amount);
                        rope2.move_rope(dir, amount);
                    }
                }
                p1 = rope.tail.visit_map.len();
                p2 = rope2.knots.last().expect("No last knot in rope2").visit_map.len();
            }
        }

        (p1.into(), p2.into())
    }
}