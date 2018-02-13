extern crate regex;
extern crate ascii;
use std::io::Read;
use std::i32;
use std::i64;

use std::char;
use std::string::String;
use std::convert::From;
use std::cell::RefCell;
use std::cell::RefMut;
use std::cell::Ref;
use super::print_utils;
use super::parse_utils;

use std::str::Chars;


pub fn day_16(){

    let programs = "abcdefghijklmnop";

    let instruction_input = parse_utils::parse_input_file("inp_16.txt");

    let result = dance(programs.to_string(), &instruction_input.to_string());

    println!("Output: {}", result);
}

pub fn day_16_b(){

    let mut programs = "abcdefghijklmnop".to_string();

    let instruction_input = parse_utils::parse_input_file("inp_16.txt");

    let result = dance_many_times(programs, &instruction_input.to_string());

    println!("Output: {}", result);
}

fn dance_many_times(mut programs: String, instruction_input: &String) -> String {

    // Det återkommer varje 4 så behöver bara köra 1000000000%4 ggr.
    let modulus = 1000000000%42;
    println!("Mod {}", modulus);
    for i in 0..modulus {
        programs = dance(programs, &instruction_input);
        println!("Partial output was {} after {} iterations.", programs, i + 1);
    }
    return programs;
}

fn dance(programs: String, instruction_input: &String) -> String {
    let split: Vec<&str> = instruction_input.split(",").collect();
    let mut output = programs.chars().collect();
    for i in 0..split.len() {
        let task: &str = split[i];
        //println!("Task: {}", task);
        let mut chars: Chars = task.chars();
        match chars.next() {
            Some('s') => {
                let string_number = chars.collect::<String>();
                let s = string_number.parse::<usize>().unwrap();
                //println!("s {}", s);
                output = spin(s, output);
            }
            Some('x') => {
                let string_instruction = chars.collect::<String>();
                let split: Vec<&str> = string_instruction.split("/").collect();

                let n = split[0].parse::<usize>().unwrap();
                let res = match split[1].parse::<usize>() {
                    Ok(t) => t,
                    Err(e) => split[1][..2].parse::<usize>().unwrap()
                };
                //println!("t {}", res);
                output = exchange(res, n, output);
            }
            Some('p') => {
                let n = chars.next().unwrap();
                let t = chars.last().unwrap();
                output = partner(n, t, output);
            }
            _ => {} // Throw exception?
        }
    }
    return output.iter().fold("".to_string(), |mut acc, val| {
                acc.push(*val);
                return acc;
            });
}


fn spin(x: usize, input_vector: Vec<char>) -> Vec<char> {
    let len = &input_vector.len();

    let (left, right) = input_vector.split_at(len - x );
    let  mut output: Vec<char> = right.to_owned();
    output.extend(left.to_owned());
    return output;
}

fn exchange(pos1: usize, pos2:usize, input_vector: Vec<char>) -> Vec<char> {
    let at_1 = input_vector.get(pos1).unwrap();
    let at_2 = input_vector.get(pos2).unwrap();

    let mut output_vector = input_vector.clone();
    output_vector[pos1] = *at_2;
    output_vector[pos2] = *at_1;
    return  output_vector;
}


fn partner(a: char, b:char, input_vector: Vec<char>) -> Vec<char> {
    let index_of_a = input_vector.iter().position(|p| *p == a).unwrap();
    let index_of_b = input_vector.iter().position(|p| *p == b).unwrap();

    let mut output_vector = input_vector.clone();
    output_vector[index_of_a] = b;
    output_vector[index_of_b] = a;
    return  output_vector;
}

#[cfg(test)]
mod tests {

    #[test]
    fn test_16() {

        super::day_16();     //Result for  = 1242 for flqrgnkx
        //println!("{}", output[0]);

    //    let inp_vec: Vec<char> = input_string.chars().collect();
    //
    //    let output = spin(3, inp_vec);
    //    println!("{}", output[0]);
    //
    //    let inp_vec2: Vec<char> = input_string.chars().collect();
    //    let output = partner('b', 'c', inp_vec2);
    //    println!("{}", output[0]);
    //    println!("{}", output[1]);
    //    println!("{}", output[2]);
    //
    //    let inp_vec3: Vec<char> = input_string.chars().collect();
    //    let output = exchange(1, 2, inp_vec3);
    //    println!("{}", output[0]);
    //    println!("{}", output[1]);
    //    println!("{}", output[2]);

        let input = "abcde";
        let instructions = "s1,x3/4,pe/b";
        let output = super::dance(input.to_string(), &instructions.to_string());
        assert_eq!("baedc", output);



        let mut programs = "abcde".to_string();
        println!("Start: {}", &programs);
           let instruction_input = "s1,x3/4,pe/b";

           let result = super::dance_many_times(programs, &instruction_input.to_string());

           println!("Output: {}", result);
    }

}
