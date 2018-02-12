
extern crate regex;
extern crate ascii;

extern crate num_traits;
use std::io::Read;
use std::i32;
use std::i64;

use std::char;
use std::string::String;
use std::convert::From;
use std::collections::HashMap;
use super::print_utils;
use super::parse_utils;

use std::str::Chars;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

/*
.#.
..#
###
*/

pub fn day_21() {
    let input = ".#./..#/###";
    let rules = super::parse_utils::read_line_by_line("inp21.txt");

    let all_rules = rules.iter().flat_map(|rule| rotate_and_flip_rule(&rule)).collect();

    let found_rule = check_for_rule(input, &all_rules);


}

fn rotate_and_flip_rule(rule: &String) -> Vec<&str> {

   let mut vector: Vec<&str> = Vec::new();
    let rows: Vec<&str> = rule.split("/").collect();
    // FV:
    let mut fvrows = rows;
    fvrows.reverse();
    vector.push(vec_to_string(fvrows));
    return vector;
}

fn vec_to_string(vector: Vec<&str>) -> &str {
  // let r = vector.iter().fold(| acc, val| acc + "" + val);
    return "".as_ref();
}

fn check_for_rule(input: &str, rules: &Vec<&str>) -> Option<String> {
    let result = rules.iter().flat_map(|row| {
        let pattern: Vec<&str> = row.split(" => ").collect();
        let in_pattern: &str = pattern[0];
        let out_pattern: &str = pattern[1];
        if input == in_pattern {
            return Some(String::from(out_pattern));
        }
        return None;
    });
    return result.last();
}