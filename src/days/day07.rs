use std::{rc::{Rc, Weak}, cell::RefCell};
use std::collections::HashMap;

use itertools::Itertools;

use crate::solution;

use super::common::get_input;

const P2_SIZE_REQ:u128 = 30000000;
const P2_TOTAL_SPACE:u128 = 70000000;

pub struct Solver{
    filename: String
}

impl Solver{
    pub fn new(filename: String) -> Solver{
        Solver{filename: filename}
    }
}


impl solution::Solver for Solver{
    fn solve<'a>(&self) -> solution::SolutionPair {
        let mut p1 = 0;
        let mut p2 = 0;
        match get_input(&self.filename){
            Err(_) => println!("Could not read file {}", self.filename),
            Ok(lines) => {
                let mut fs = FileSystem::new();
                for line in lines{
                    if let Ok(line) = line{
                        let first_char = line.as_bytes()[0] as char;
                        match first_char {
                            '$' => {
                                let parts: Vec<&str> = line.split(' ').collect();
                                if parts[1] == "cd"{
                                    fs.change_directory(parts[2]);
                                    //todo!("Changing of current directory");
                                }
                            },
                            _ => {
                                let parts: Vec<&str> = line.split(' ').collect();
                                if parts[0] == "dir"{
                                    let new_dir = Rc::new(RefCell::new(Node::new_dir(String::from(parts[1]))));
                                    fs.add_entry(new_dir);
                                }else{
                                    let new_file = Rc::new(RefCell::new(Node::new_file(String::from(parts[1]), parts[0].parse().expect("Could not parse file size as number"))));
                                    fs.add_entry(new_file);
                                }
                            }
                        }
                    }
                }
                //println!("{:?}", fs);
                p1 = fs.root.borrow().get_p1_size();
                p2 = fs.root.borrow().get_p2_size(P2_SIZE_REQ - (P2_TOTAL_SPACE - fs.root.borrow().size));
            }
        }
        (p1.into(), p2.into())
    }
}

#[derive(Debug)]
#[allow(dead_code)]
struct Node{
    name: String,
    size: u128,
    node_type: NODETYPE,
    children: HashMap<String, Rc<RefCell<Node>>>,
    parent: Weak<RefCell<Node>>
}

#[derive(Debug)]
#[derive(PartialEq)]
enum NODETYPE{
    FILE,
    DIR
}

#[derive(Debug)]
#[allow(dead_code)]
struct FileSystem{
    pwd: String,
    root: Rc<RefCell<Node>>,
    current: Rc<RefCell<Node>>
}

impl Node{
    fn new_file(name: String, size: u128) -> Node{
        Node { name, size, parent: Weak::new(), children: HashMap::new(), node_type: NODETYPE::FILE}
    }

    fn new_dir(name: String) -> Node{
        Node { name, size: 0, node_type: NODETYPE::DIR, children: HashMap::new(), parent: Weak::new() }
    }

    fn add_child(&mut self, child: &Rc<RefCell<Node>>){
        let name = child.borrow().name.clone();
        self.children.insert(name, Rc::clone(child));
    }

    fn increase_size(&mut self, size: u128 ){
        match self.node_type{
            NODETYPE::DIR => {
                self.size += size;
            },
            _ => {}
        }
        match self.parent.upgrade(){
            Some(p) => {
                p.borrow_mut().increase_size(size);
            },
            None => {}
        }
    }

    fn get_p1_size(&self) -> u128{
        if self.node_type == NODETYPE::FILE{
            return 0;
        }
        let mut cur_total = 0;
        if self.size <= 100000{
            cur_total += self.size;
        }
        for node in self.children.values(){
            cur_total += node.borrow().get_p1_size();
        }
        return cur_total;
    }

    fn get_p2_size(&self, required_space: u128) -> u128{
        if self.node_type == NODETYPE::FILE{
            return 0;
        }
        let smallest: Option<&Rc<RefCell<Node>>> = self.children.values()
            .filter(|x| x.borrow().size >= required_space && x.borrow().node_type == NODETYPE::DIR)
            .sorted_by(|a, b| Ord::cmp(&a.borrow().size, &b.borrow().size)).nth(0);
            
        return match smallest{
            Some(child) => child.borrow().get_p2_size(required_space),
            None => self.size
        }
    }
}


impl FileSystem{
    fn new() -> FileSystem{
        let dir = Rc::new(RefCell::new(Node::new_dir(String::from("/"))));
        FileSystem { pwd: String::from("/"), root: Rc::clone(&dir), current: Rc::clone(&dir) }
    }

    fn add_entry(&mut self, entry: Rc<RefCell<Node>>){
        let size ;
        {
            let mut mut_borrow = (*entry).borrow_mut();
            mut_borrow.parent = Rc::downgrade(&self.current);
            size = mut_borrow.size;
        }
        (*self.current).borrow_mut().add_child(&entry);
        let mut mut_borrow = (*entry).borrow_mut();
        if mut_borrow.node_type == NODETYPE::FILE{
            mut_borrow.increase_size(size);
        }
    }

    fn change_directory(&mut self, path: &str){
        match path{
            "/" => { 
                self.current = Rc::clone(&self.root)
            },
            ".." => {
                let parent = (*self.current).borrow().parent.upgrade().expect("Tried to move up in directory without parent");
                self.current = Rc::clone(&parent);
            },
            _ => {
                let current_ptr = Rc::clone(&self.current);
                let node = (*current_ptr).borrow();
                let child = node.children.get(path).expect("Could not find child directory");
                self.current = Rc::clone(child);
            }
        };
    }
}