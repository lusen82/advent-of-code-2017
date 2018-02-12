
extern crate regex;
extern crate ascii;
use std::i32;

pub fn print_vector_str(v: &Vec<String>)
{
    print!("[");
    v.iter().for_each(|f| print!("{} ", f));
    println!("]");
}

pub fn print_vector(v: &Vec<i32>)
{
    print!("[");
    v.iter().for_each(|f| print!("{} ", f.to_string()));
    println!("]");
}

pub fn print_vector_u(v: &Vec<usize>)
{
    print!("[");
    v.iter().for_each(|f| print!("{} ", f.to_string()));
    println!("]");
}
