
extern crate regex;
extern crate ascii;
use std::string::String;
use std::i32;
use std::collections::HashSet;
use super::parse_utils;



pub fn day6() {
    let inn: String = parse_utils::parse_input_file("inp6.txt");
    day_6_calc(&inn);
    day_6_b_calc(&inn);
}

pub fn day_6_calc<'a>(inn: &'a str) -> Vec<i32> {
    let vec_ints: Vec<i32> = get_vec_str_pointers(&inn);;
    let mut seen: HashSet<Vec<i32>> = HashSet::new();

    let mut cycles = 0;
    let mut res = vec_ints.clone();
    seen.insert(vec_ints);
    let resulting_vector = loop {
        res = re_calc_ints_(&res);
        cycles += 1;
        if seen.contains(&res) {
            println!("Result 6 a: {}", cycles);
            break res;
        }
        seen.insert(res.clone());
    };

    return resulting_vector;
}

pub fn day_6_b_calc<'a>(inn: &'a str) {

    let result_in_a = day_6_calc(inn);


    let mut res_b = result_in_a.clone();
    let mut cycles = 0;
    let result = loop {
        res_b = re_calc_ints_(&res_b);
        cycles += 1;
        if result_in_a == res_b {
            println!("Result 6b: {}", cycles);
            break;
        }
    };

}

pub fn re_calc_ints_(vec_ints: &Vec<i32>) -> Vec<i32> {
    let max_val: &i32 = vec_ints.iter().max().unwrap();
    let max_i: usize = vec_ints.iter().position(|it| cmpare(it, max_val)).unwrap();
    let  mut result = vec_ints.clone();
    result[max_i] = 0;
    for i in 0..*max_val as usize {
        let new_index = i + max_i + 1;
        let  index = match new_index >= vec_ints.len() {
            true => new_index % vec_ints.len(),
            false => new_index
        };
        result[index] += 1;
    }
    return result;

}

fn cmpare(it: &i32, max_val: &i32) -> bool
{
     return it == max_val;
}

fn get_vec_str_pointers<'a>(inn: &'a &str) -> Vec<i32> {
    return inn.split_whitespace().into_iter().flat_map(|s| s.parse::<i32>()).collect();

}

#[cfg(test)]
mod tests {
        #[test]
        fn test_some_stuff() {
           // assert_eq!("0","m");
            super::day6();           //Result 6 a: 11137 Result 6b: 1037
        }

}
