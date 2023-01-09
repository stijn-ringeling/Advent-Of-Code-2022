use itertools::enumerate;

use crate::solution;

use super::common::get_input;

struct Forrest{
    lines: Vec<Vec<u32>>,
    columns: Vec<Vec<u32>>,
    visible: Vec<Vec<bool>>,
}

pub struct Solver{
    filename: String
}

impl Solver{
    pub fn new(filename: String) -> Solver{
        Solver{filename: filename}
    }
}

impl solution::Solver for Solver{
    fn solve(&self) -> solution::SolutionPair{
        let mut p1 = 0;
        let mut p2 = 0;
        match get_input(&self.filename){
            Err(_) => println!("Could not read file {}", self.filename),
            Ok(lines) => {
                let mut forrest = Forrest{lines: vec![], columns: vec![], visible: vec![]};
                for line in lines{
                    if let Ok(line) = line{
                        let mut row_chars = Vec::<u32>::new();
                        let mut row_visible = Vec::<bool>::new();
                        for (idx, tree) in enumerate(line.chars().map(|x| x.to_digit(10).unwrap())){
                            row_chars.push(tree); //Create the current row of trees.
                            row_visible.push(false);
                            if forrest.columns.len() == idx{// During the first line we will not have any columns yet
                                let column_start = vec![tree];
                                forrest.columns.push(column_start); //Push a vector with the first tree to start
                            }else{
                                forrest.columns[idx].push(tree); //Add to the existing vectors
                            }
                        }
                        forrest.lines.push(row_chars);
                        forrest.visible.push(row_visible);
                    }
                }
                //Done parsing input. Starting looking for trees
                //Do part 2 first for borrow checking purposes.
                let width = forrest.columns.len();
                let height = forrest.lines.len();

                for row in 0..height{
                    for column in 0..width{
                        let cur_tree = forrest.lines[row][column];
                        //top check
                        let mut top_dist = 0;
                        if row > 0{
                            for row_idx in (0..(row)).rev(){
                                top_dist += 1;
                                if forrest.lines[row_idx][column] >= cur_tree{
                                    break;
                                }
                            }
                        }
                        //bot check
                        let mut bot_dist = 0;
                        if row < height-1{
                            for row_idx in (row+1)..height{
                                bot_dist += 1;
                                if forrest.lines[row_idx][column] >= cur_tree{
                                    break;
                                }
                            }
                        }
                        //left check
                        let mut left_dist = 0;
                        if column > 0{
                            for col_idx in (0..(column)).rev(){
                                left_dist += 1;
                                if forrest.lines[row][col_idx] >= cur_tree{
                                    break;
                                }
                            }
                        }
                        //right check
                        let mut right_dist = 0;
                        if column < width-1{
                            for col_idx in (column+1)..height{
                                right_dist += 1;
                                if forrest.lines[row][col_idx] >= cur_tree{
                                    break;
                                }
                            }
                        }
                        let score = left_dist * right_dist * top_dist * bot_dist;
                        //println!("{}, {}: {}", row, column, score);
                        if score > p2{
                            p2 = score;
                            //println!("Found new high score at {}, {} (value of {})", row, column, p2)
                        }
                    }
                }
                
                //Left to right
                for (row_idx, row) in enumerate(forrest.lines){
                    let mut max_row: i32 = -1;
                    let mut reverse_row = row.to_vec();
                    reverse_row.reverse();
                    for (col_idx, tree) in enumerate(row){
                        if i32::try_from(tree).unwrap() > max_row{
                            max_row = i32::try_from(tree).unwrap();
                            //println!("{}, {} visible from the left", row_idx, col_idx);
                            if forrest.visible[row_idx][col_idx] == false{
                                p1+=1;
                                forrest.visible[row_idx][col_idx] = true;
                                
                            }
                        }
                    }
                    //Right to left
                    let mut max_row2: i32 = -1;
                    let row_len = reverse_row.len();
                    for (reverse_col_idx, tree) in enumerate(reverse_row){
                        let col_idx = row_len - reverse_col_idx - 1;
                        if i32::try_from(tree).unwrap() > max_row2{
                            //println!("{}, {} visible from the right", row_idx, col_idx);
                            max_row2 = i32::try_from(tree).unwrap();
                            if forrest.visible[row_idx][col_idx] == false{
                                p1+=1;
                                forrest.visible[row_idx][col_idx] = true;
                                
                            }
                        }
                    }
                }
                //Top down
                for (col_idx, column) in enumerate(forrest.columns){
                    let mut max_column: i32 = -1;
                    let mut reverse_column = column.to_vec();
                    reverse_column.reverse();
                    for (row_idx, tree) in enumerate(column){
                        if i32::try_from(tree).unwrap() > max_column{
                            max_column = i32::try_from(tree).unwrap();
                            //println!("{}, {} visible from the top", row_idx, col_idx);
                            if forrest.visible[row_idx][col_idx] == false{
                                p1+=1;
                                forrest.visible[row_idx][col_idx] = true;
                                
                            }
                        }
                    }
                    //Right to left
                    let mut max_column2: i32 = -1;
                    let column_len = reverse_column.len();
                    for (reverse_row_idx, tree) in enumerate(reverse_column){
                        let row_idx = column_len - reverse_row_idx - 1;
                        if i32::try_from(tree).unwrap() > max_column2{
                            max_column2 = i32::try_from(tree).unwrap();
                            //println!("{}, {} visible from the bottom", row_idx, col_idx);
                            if forrest.visible[row_idx][col_idx] == false{
                                p1+=1;
                                forrest.visible[row_idx][col_idx] = true;
                                
                            }
                        }
                    }
                }
                //println!("{:?}", forrest.visible);
            }
        }
        (p1.into(), p2.into())
    }
}