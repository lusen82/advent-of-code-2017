//
//extern crate regex;
//extern crate ascii;
//extern crate num_traits;
//use std::io::Read;
//use std::i32;
//use std::i64;
//
//use std::char;
//use std::string::String;
//use std::convert::From;
//use std::collections::HashSet;
//use std::collections::HashMap;
//use std::str;
//use super::print_utils;
//use super::parse_utils;
//
//use  std::cmp::Ord;
//use std::cmp::Ordering;
//use std::str::Chars;
//use std::num;
//use std::iter::FromIterator;
//
//use std::collections::hash_map::DefaultHasher;
//use std::hash::{Hash, Hasher};
//
//
//pub fn day_3()
//{
//    let input: u64 = 347991;       //344569 (587) , 351649 (593)
//
//    let primes: Vec<u64> = primes_up_to(592);
//    let last_prime = primes.last().unwrap();
//
//    let nr_of_steps_down_to_corner = primes.len() - 1;
//    let nr_of_steps_right_to_corner = primes.len() - 1;
//
//    let difference_between_value_and_prime = input - *last_prime;
//
//    println!("difference_between_value_and_prime: {}", difference_between_value_and_prime);
//    println!("nr_of_steps_down_to_corner: {}", nr_of_steps_down_to_corner);
//    println!("Last prime: {}", last_prime);
//
//    // Size of quadrat is prime + 1.
//    let start = Point{value: 1, i:0, j: 0};
//    let mut step = Step::right;
//    let mut all_points = Vec::new();
//    all_points.push(start);
//    let mut current_point = start;
//    while current_value < 1000 {
//        if (step == Step::right) {
//            all_points.sort_by_key(|p| p.j);
//            if current_point.j <= all_points.first().unwrap().j{
//                current_point = get_next_point(current_point, step, all_points);
//            }
//            }
//    }
//}
//
//fn get_next_point(cu: Point, st: Step , all:Vec<Point>) -> Point {
//    let val = cu.value;
//    let mut adjacent = HashSet::new();
//    adjacent.push((cu.i - 1, cu.j - 1));
//    adjacent.push((cu.i - 1, cu.j));
//    adjacent.push((cu.i - 1, cu.j + 1));
//    adjacent.push((cu.i, cu.j + 1));
//    adjacent.push((cu.i + 1, cu.j + 1));
//    adjacent.push((cu.i + 1, cu.j));
//    adjacent.push((cu.i + 1, cu.j - 1));
//    adjacent.push((cu.i, cu.j - 1));
//
//    let new_value = all.iter().filter(|point| adjacent.contains((point.i, point.j))).fold(|s, acc| acc + s.value);
//
//Point{value: new_value, i: }
//}
//
//enum Step {
//    up,
//    down,
//    left,
//    right
//}
//
//struct Point{
//    value: i64,
//    i: i64,
//    j: i64
//}
//
//
//fn primes_up_to(limit: u64) -> Vec<u64> {
//    let mut vec: Vec<_> = (2..limit).collect();
//
//    for p in 2..limit {
//        vec.retain(|&x| x <= p || x % p != 0);
//    }
//
//    vec
//}
//
//
//#[cfg(test)]
//mod tests {
//    #[test]
//    pub fn test_on_inp()
//    {
//        super::day_3();
//    }
//}
