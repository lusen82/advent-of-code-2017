/*
extern crate regex;
extern crate ascii;
use std::io::Read;
use std::i32;
use std::i64;

use std::string::String;
use std::convert::From;
use std::cell::RefCell;
use std::cell::RefMut;
use std::cell::Ref;
use super::print_utils;
use super::parse_utils;

use dec_10;

pub fn day_14(){

    let input = "hwlqcszp";//"hwlqcszp";
    let i_vector: Vec<usize> = (0..128).into_iter().collect();
    let input_texts: Vec<String> = i_vector.into_iter().map(|row| {
        let mut row_text: String = String::from(input);
        row_text.push('-');
        row_text += &row.to_string();
        return row_text;
    }).collect();


    let matrix: Vec<String> = input_texts.into_iter().map(|row| make_knot_hash_using_10_b(&row)).collect();

    let binary_matrix:Vec<Vec<String>> = matrix.into_iter().map(|row| {
        return row.chars().into_iter().map(|v| {
            let  decimal = v.to_digit(16).unwrap();
            let  binary_string: String = format!("{:#06b}", decimal);
            return binary_string[2..].to_string();
        }).collect();
    }).collect();


    let vec_of_chars: Vec<Vec<char>> = binary_matrix.into_iter().map(|row| {
        return row.into_iter().flat_map(|s| {
            let char_vec:Vec<char> = s.chars().collect();
            return vec![char_vec[0], char_vec[1], char_vec[2], char_vec[3]];
        }).collect();
    }).collect();

    let vec_of_bits: Vec<Vec<i32>> = vec_of_chars.into_iter().map(|row| {
        return row.into_iter().map(|s| match s { '1' => 1, _ => 0 }  ).collect();
    }).collect();


    let vec_of_nodes: Vec<Vec<RefCell<Node>>> = vec_of_bits.into_iter().map(|r| {
        return r.into_iter().map(|b| RefCell::new(Node{square: b == 1, visited: false })).collect();
    }).collect();

    let mut total_nr_of_groups = 0;
    for i in 0..128 {
        for j in 0..128 {
            let square = &vec_of_nodes[i][j].try_borrow().unwrap().square;
            if *square {
                let visited = &vec_of_nodes[i][j].try_borrow().unwrap().visited;
                count_node_neighbours(&vec_of_nodes, i, j, *visited);
                if !visited {
                    if let Ok(mut node) = vec_of_nodes[i][j].try_borrow_mut() {
                        node.set_visited_true();
                    }
                    total_nr_of_groups += 1;
                }
            }
        }
    }

    println!("Number of regions is {}", total_nr_of_groups);

}

struct Node {
    visited: bool,
    square: bool
}

impl Node {
    fn set_value_true(&mut self) {
        self.square = true;
    }
    fn set_visited_true(&mut self) {
        self.visited = true;
    }
}

fn count_node_neighbours(vec_of_nodes: &Vec<Vec<RefCell<Node>>>, i: usize, j: usize, visited: bool) {

    if visited {
        return;
    }

    if j > 0 {
        run_for_adjacent(vec_of_nodes, i, j - 1);
    }
    if i > 0 {
        run_for_adjacent(vec_of_nodes, i - 1, j);
    }
    if j < 127 {
        run_for_adjacent(vec_of_nodes, i, j + 1);
    }
    if i < 127 {
        run_for_adjacent(vec_of_nodes, i + 1, j);
    }

}

fn run_for_adjacent(vec_of_nodes: &Vec<Vec<RefCell<Node>>>, i: usize, j: usize) -> () {
    if let Ok(mut adjacent) = vec_of_nodes[i][j].try_borrow_mut() {
        if adjacent.square {
            count_node_neighbours(vec_of_nodes, i, j, adjacent.visited);
            adjacent.set_visited_true();
        }
    }
}

pub fn make_knot_hash_using_10_b(inp: &str) -> String {

    // Convert to ascii and extend with same ending as in 10b:
    let mut my_input_in_ascii: Vec<usize> = inp.chars().map(|m| m as usize).collect();
    my_input_in_ascii.extend(vec![17, 31, 73, 47, 23]);

    let mut current_position: usize = 0;
    let mut skip_size: u128 = 0;
    let mut vec: Vec<i32> = (0..256).collect();

    for i in 0..64 {
        for it in 0..my_input_in_ascii.len() {
            let next_value: usize = my_input_in_ascii[it];
            let mut clone = vec.clone();
            vec = dec_10::get_sub_split(clone, current_position, next_value);
            let skip = it as u128 + skip_size;
            let next_current_pos: u128 = current_position as u128 + skip + next_value as u128;
            current_position = match next_current_pos {
                n if n >= 256 => next_current_pos % 256,
                _ => next_current_pos,
            } as usize
        }
        let to_add_skip = my_input_in_ascii.len();
        skip_size = skip_size + to_add_skip as u128;
    }

    // Sum up to dense hash:
    let mut sixteen = Vec::new();
    let cc: Vec<&i32> = vec.iter().map(|kl| kl).collect();
    let mut optional = Some(cc.split_at(16));
    while let Some(head) = optional {
        let chunk: i32 = head.0.iter().fold(0, |acc, val| acc ^ *val);
        sixteen.push(chunk);
        optional = match head.1.is_empty() {
            true => None,
            false => Some(head.1.split_at(16))
        };
    }

    let result = sixteen.iter().fold(String::new(), |acc, knot| acc.to_string() + &format!("{:02x}", knot));

    return result.to_string();
}





#[cfg(test)]
mod tests {

    #[test]
    fn test_14() {
        super::day_14();     //Result 2 = 1242 for flqrgnkx

    }

}
*/
