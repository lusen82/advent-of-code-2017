extern crate regex;
extern crate ascii;
use std::io::Read;
use std::i32;
use super::print_utils;
use super::parse_utils;

pub fn day_10(){
    let inp = parse_utils::parse_input_file("inp10.txt");

    let my_input: Vec<&str> = inp.split(',').collect();

    let mut vec: Vec<i32> = (0..256).collect();
    let mut current_position: usize = 0;
    for it in 0..my_input.len(){
        let x: &str = my_input[it].trim();
        let next_value : usize = x.parse().unwrap();
        let clone = vec.clone();
        vec = get_sub_split(clone, current_position, next_value);
        let next_current_pos = current_position + it + next_value;
        current_position = match next_current_pos {
            n if n >= 256  => next_current_pos - 256,
            _  => next_current_pos,
        };
    }

    let multiplication = vec[0] * vec[1];
    println!("Multiplication = {}", multiplication);
}

/*

pub fn day_10b() {
    let inp = parse_utils::parse_input_file("inp10.txt");

    let mut my_input_in_ascii: Vec<usize> = inp.chars().map(|m| m as usize).collect();
    let len_before = &my_input_in_ascii.len() - 1;
    my_input_in_ascii.remove(len_before);
    my_input_in_ascii.extend(vec![17, 31, 73, 47, 23]);

    let mut current_position: usize = 0;
    let mut skip_size: u128 = 0;
    let mut vec: Vec<i32> = (0..256).collect();

    for i in 0..64 {
        for it in 0..my_input_in_ascii.len() {
            let next_value: usize = my_input_in_ascii[it];
            let mut clone = vec.clone();
            vec = get_sub_split(clone, current_position, next_value);
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

    print_utils::print_vector(&vec);

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

    sixteen.iter().for_each(|knot| print!("{:02x}", knot));

}
*/


pub fn get_sub_split(mut vec: Vec<i32>, current_position: usize, length: usize) -> Vec<i32> {
    if length >= vec.len() {
        return vec;
    }
    let (before_current_pos, after_current_pos) = vec.split_at_mut(current_position);
    let remaining_less_or_equal_length = length <= after_current_pos.len();
    if remaining_less_or_equal_length {
        let (to_reverse, rest) = after_current_pos.split_at_mut(length);
        let mut updated: Vec<i32> = vec![];
        to_reverse.reverse();
        updated.extend_from_slice(before_current_pos);
        updated.extend_from_slice(to_reverse);
        updated.extend_from_slice(rest);
        return updated;
    } else {
        let length_after_current_pos = after_current_pos.len();
        let nr_need_to_take_from_start = length - length_after_current_pos;
        let (to_also_reverse, rest) = before_current_pos.split_at_mut(nr_need_to_take_from_start);
        let mut to_reverse: Vec<i32> = vec![];
        to_reverse.extend_from_slice(after_current_pos);
        to_reverse.extend_from_slice(to_also_reverse);
        to_reverse.reverse();
        let (new_end, new_start) = to_reverse.split_at_mut(length_after_current_pos);

        let mut updated: Vec<i32> = vec![];

        updated.extend_from_slice(new_start);
        updated.extend_from_slice(rest);
        updated.extend_from_slice(new_end);
        return updated;
    }
}


pub fn take_first_reversed_sublist() -> Vec<i32>{
    let inp = parse_utils::parse_input_file("inp10.txt");
    let first_value : usize = inp.split(',').next().unwrap().parse().unwrap();
    let mut vec: Vec<i32> = (0..255).collect();
    let (part1, part2) = vec.split_at_mut(first_value);
    let mut updated : Vec<i32> = vec![];
    part1.reverse();
    updated.extend_from_slice(part1);
    updated.extend_from_slice(part2);

    return updated;

}


#[cfg(test)]
mod tests {

    #[test]
    fn test_10_a() {
        super::day_10();     //Multiplication = 11375
        super::day_10b();
    }

    #[test]
    fn test_some_stuff() {
       // assert_eq!("0","m");
        let first = super::take_first_reversed_sublist();
        println!("Length: {}", first.len());
        assert_eq!(255 as usize, first.len());
        assert_eq!(87 as i32, *first.first().unwrap());
        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let res = super::get_sub_split(vec, 6, 7);
        println!("{}", res[0]);

        let vec = vec![2, 1, 0, 3, 4];
        let res = super::get_sub_split(vec, 3, 4);
        println!("{}", res[0]);
        assert_eq!(vec![4, 3, 0, 1, 2], res);

        let vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
        let res = super::get_sub_split(vec, 6, 7);
        println!("{}", res[0]);
    }
}
