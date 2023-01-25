use std::{fmt::{Display, Debug}, cmp::Ordering};

use anyhow::Result;
use itertools::{enumerate};

use crate::solution;

use super::common::get_input;

#[derive(Debug, Eq, Ord)]
enum Item{
    Value((u32, usize)),
    List((Vec<Item>, usize))
}

impl From<&str> for Item {
    fn from(value: &str) -> Self {
        if value.chars().nth(0) == Some('[') {
            //todo!("Implement recursive list")
            let mut item_list = Vec::<Item>::new();
            let mut curr_index: usize = 1;
            loop{
                let next_item = Self::from(&value[curr_index..]);
                let end_index = match &next_item {
                    Self::Value(v) => v.1,
                    Self::List(v) => v.1,
                };
                
                let end_character = value.chars().nth(curr_index + end_index).expect("End character not found");
                //println!("{:?}", end_character);
                item_list.push(next_item);
                curr_index = curr_index + end_index + 1;
                if end_character == ']'{
                    break;
                }
            }
            return Self::List((item_list, curr_index));
        }else{
            //todo!("Finish parsing of one number");
            let end_index ;
            let comma_index = value.find(',');
            let list_end_index = value.find(']');
            if comma_index == None{
                end_index = list_end_index.expect("Could not find comma or ] for end of list");
            }else{
                let comma_index = comma_index.expect("Tried to unwrap comma index after checking if it was none");
                let list_end_index = list_end_index.expect("There should always be a closing list tag ]");
                end_index = comma_index.min(list_end_index);
            }
            if end_index == 0{
                return Self::List((Vec::new(), 0))
            }
            return Self::Value((value[..end_index].parse().expect(format!("Could not parse {} as u32", &value[..end_index]).as_str()), end_index))
        }
    }
}


impl PartialEq for Item{
    fn eq(&self, other: &Self) -> bool {
        match self{
            Self::Value((v, _)) => {
                match other {
                    Self::Value((v_o, _)) => v == v_o,
                    Self::List((_, _)) => {
                        let new_left = Self::List((Vec::<Item>::from([Self::Value((*v, 0))]),0));
                        return new_left.eq(&other);
                    }
                }
            },
            Self::List((v, _)) => {
                match other{
                    Self::Value((v_o, _)) => {
                        let new_right = Self::List((Vec::<Item>::from([Self::Value((*v_o,0))]),0));
                        return self.eq(&new_right);
                    },
                    Self::List((v_o, _)) => {
                        for idx in 0..v.len(){
                            if idx >= v_o.len(){
                                return false
                            }
                            if !v[idx].eq(&v_o[idx]){
                                return false
                            }
                        }
                        if v.len() == v_o.len(){// Ran out of left items and right items at the same time
                            return true
                        }
                        false//Ran out of left items before right items
                    }
                }
            }
        }
    }
}

impl PartialOrd for Item{
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self{
            Self::Value((v, _)) => {
                match other {
                    Self::Value((v_o, _)) => Some(v.cmp(v_o)),
                    Self::List((_, _)) => {
                        let new_left = Self::List((Vec::<Item>::from([Self::Value((*v, 0))]),0));
                        return new_left.partial_cmp(&other);
                    }
                }
            },
            Self::List((v, _)) => {
                match other{
                    Self::Value((v_o, _)) => {
                        let new_right = Self::List((Vec::<Item>::from([Self::Value((*v_o,0))]),0));
                        return self.partial_cmp(&new_right);
                    },
                    Self::List((v_o, _)) => {
                        for idx in 0..v.len(){
                            if idx >= v_o.len(){
                                return Some(Ordering::Greater)
                            }
                            if v[idx] != v_o[idx]{
                                return v[idx].partial_cmp(&v_o[idx])
                            }
                        }
                        if v.len() == v_o.len(){// Ran out of left items and right items at the same time
                            return Some(Ordering::Equal)
                        }
                        Some(Ordering::Less)//Ran out of left items before right items
                    }
                }
            }
        }
    }
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Value(v) => std::fmt::Display::fmt(&v.0, f),
            Self::List(v) => {
                write!(f, "[")?;
                let mut first = true;
                for item in &v.0{
                    if first{
                        first = false
                    }else{
                        write!(f, ",")?;
                    }
                    std::fmt::Display::fmt(item, f)?;
                    
                }
                write!(f,"]")?;
                Ok(())
            }
        }
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
        let mut p2 = 1;
        match get_input(&self.filename){
            Err(_) => println!("Could not read file {}", self.filename),
            Ok(lines) => {
                let lines = lines.collect::<Result<Vec<String>, _>>().unwrap();
                let mut items: Vec<Item> = Vec::new();
                for (i, line) in enumerate(lines.chunks(3)){
                    let item_1 = Item::from(line[0].as_str());
                    let item_2 = Item::from(line[1].as_str());
                    //println!("Input 1: {}", line[0]);
                    //println!("Output 1: {}", item_1);
                    //println!("Input 2: {}" , line[1]);
                    //println!("Output 2: {}", item_2);
                    if item_1 < item_2{
                        //println!("{}", i+1);
                        p1 += i+1;
                    }
                    items.push(item_1);
                    items.push(item_2);
                }
                let key_1 = Item::List(
                    (vec![
                        Item::List(
                            (vec![
                                Item::Value(
                                    (2,0))
                                ],
                            0)
                        )],
                    0)
                );
                let key_1_ref = Item::List(
                    (vec![
                        Item::List(
                            (vec![
                                Item::Value(
                                    (2,0))
                                ],
                            0)
                        )],
                    0)
                );
                let key_2 = Item::List(
                    (vec![
                        Item::List(
                            (vec![
                                Item::Value(
                                    (6,0))
                                ],
                            0)
                        )],
                    0)
                );
                let key_2_ref = Item::List(
                    (vec![
                        Item::List(
                            (vec![
                                Item::Value(
                                    (6,0))
                                ],
                            0)
                        )],
                    0)
                );
                items.push(key_1);
                items.push(key_2);
                items.sort();
                for (i, item) in enumerate(&items){
                    //println!("{}", item);
                    if *item == key_1_ref{
                        //println!("Found key 1")
                        p2 *= i+1;
                    }
                    if *item == key_2_ref{
                        //println!("Found key 2")
                        p2 *= i+1;
                    }
                }
            }
        }
        (p1.into(), p2.into())
    }
}