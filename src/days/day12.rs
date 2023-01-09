use std::{cell::RefCell, rc::{Rc, Weak}, fmt::{Display, Debug}};

use crate::solution;

use super::common::get_input;

#[derive(Debug)]
struct Node{
    elevation: u32,
    neighbors: Vec<Weak<RefCell<Node>>>,
    reverse_neighbors: Vec<Weak<RefCell<Node>>>,
    visited: bool,
    dist: u32
}

impl Node{
    fn new(c: char) -> Node{
        let elevation = c as u32 - 97;
        Node{elevation, neighbors: Vec::new(), reverse_neighbors: Vec::new(), visited: false, dist: 0}
    }
}

impl Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.elevation, self.dist)
    }
}

#[derive(Debug)]
struct Map{
    items: Vec<Vec<Rc<RefCell<Node>>>>,
    start: Option<Rc<RefCell<Node>>>,
    end: Option<Rc<RefCell<Node>>>
}
impl Map{
    fn new() -> Self{
        Self { items: Vec::new(), start: None, end: None }
    }

    fn add_node(&mut self, x: usize, y: usize, c: char){
        let node = match c{
            'S' => Rc::new(RefCell::new(Node::new('a'))),
            'E' => Rc::new(RefCell::new(Node::new('z'))),
            _ => Rc::new(RefCell::new(Node::new(c)))
        };
        let nb_clone = Rc::clone(&node);
        if c == 'S'{
            self.start = Some(Rc::clone(&node));
            
        }
        if c == 'E'{
            self.end = Some(Rc::clone(&node));
        }
        if y >= self.items.len(){
            let new_row = vec![node];
            self.items.push(new_row);
        }else{
            self.items[y].push(node);
        }

        if x >= 1{
            Map::update_neighbor(&nb_clone, &self.items[y][x-1]);
        }
        if y >= 1{
            Map::update_neighbor(&nb_clone, &self.items[y-1][x]);
        }
    }

    fn update_neighbor(first: &Rc<RefCell<Node>>, second: &Rc<RefCell<Node>>){
        if second.borrow().elevation <= first.borrow().elevation + 1{
            first.borrow_mut().neighbors.push(Rc::downgrade(second));
            second.borrow_mut().reverse_neighbors.push(Rc::downgrade(first));
        }
        if first.borrow().elevation <= second.borrow().elevation + 1{
            second.borrow_mut().neighbors.push(Rc::downgrade(first));
            first.borrow_mut().reverse_neighbors.push(Rc::downgrade(second));
        }
    }

    fn bfs(&self){
        match &self.start{
            None => return,
            Some(start) => {
                {
                    let mut s_m = start.borrow_mut();
                    s_m.visited = true;
                    s_m.dist = 0;
                }
                

                let mut stack = vec![Rc::clone(start)];
                
                loop {
                    if stack.len() == 0{
                        break;
                    }
                    let current = Rc::clone(&stack[0]);
                    stack.remove(0);
                    for nb in &current.borrow().neighbors{
                        match nb.upgrade(){
                            None => println!("Neighbor no longer exists. Whoops"),
                            Some(neighbor) => {
                                let mut nb_mut = neighbor.borrow_mut();
                                if nb_mut.visited == false{
                                    //println!("Exploring node with elevation {}", nb_mut.elevation);
                                    nb_mut.dist = current.borrow().dist + 1;
                                    nb_mut.visited = true;
                                    stack.push(Rc::clone(&neighbor));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn reverse_bfs(&self) {
        match &self.end{
            None => return,
            Some(start) => {
                {
                    let mut s_m = start.borrow_mut();
                    s_m.visited = true;
                    s_m.dist = 0;
                }
                

                let mut stack = vec![Rc::clone(start)];
                
                loop {
                    if stack.len() == 0{
                        break;
                    }
                    let current = Rc::clone(&stack[0]);
                    stack.remove(0);
                    for nb in &current.borrow().reverse_neighbors{
                        match nb.upgrade(){
                            None => println!("Neighbor no longer exists. Whoops"),
                            Some(neighbor) => {
                                let mut nb_mut = neighbor.borrow_mut();
                                if nb_mut.visited == false{
                                    //println!("Exploring node with elevation {}", nb_mut.elevation);
                                    nb_mut.dist = current.borrow().dist + 1;
                                    nb_mut.visited = true;
                                    stack.push(Rc::clone(&neighbor));
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    fn reset_dists(&self){
        for row in &self.items{
            for item in row{
                item.borrow_mut().dist = 0;
                item.borrow_mut().visited = false;
            }
        }
    }

    fn get_end_dist(&self) -> u32{
        match &self.end {
            None => 0,
            Some(node) => node.borrow().dist
        }
    }

    fn get_shortest_path(&self) -> u32{
        let mut min_dist = 1000;
        for row in &self.items{
            for item in row{
                if item.borrow().dist < min_dist && item.borrow().elevation == 0 && item.borrow().dist != 0{
                    min_dist = item.borrow().dist;
                }
            }
        }
        return min_dist;
    }
}

impl Display for Map{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for row in &self.items{
            for node in row{
                std::fmt::Display::fmt(&*node.borrow(), f)?;
                print!(" ");
            }
            println!();
        }
        self.end.fmt(f)
    }
}

pub struct Solver{
    filename: String
}

impl Solver{
    pub fn new(filename: String) -> Solver{
        Solver{filename}
    }
}

impl solution::Solver for Solver{
    fn solve(&self) -> solution::SolutionPair {
        let mut p1 = 0;
        let mut p2 = 0;
        match get_input(&self.filename){
            Err(_) => println!("Could not load file {}", self.filename),
            Ok(lines) => {
                let mut map = Map::new();
                let mut y = 0;
                for line in lines{
                    let mut x = 0;
                    if let Ok(line) = line{
                        for char in line.chars(){
                            map.add_node(x, y, char);
                            x+=1;
                        }
                    }
                    y+=1;
                }
                map.bfs();
                //println!("{}", map);
                p1 = map.get_end_dist();
                map.reset_dists();
                map.reverse_bfs();
                p2 = map.get_shortest_path();
                //println!("{}", map);
            }
        }
        (p1.into(), p2.into())
    }
}