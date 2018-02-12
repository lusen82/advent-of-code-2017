
extern crate regex;
extern crate ascii;
use std::string::String;
use std::io::{BufReader, BufRead};
use std::io::Lines;
use std::fs::File;
use std::io::Read;


pub fn parse_input_file(file_name: &'static str) -> String {
        let mut f = File::open(file_name).expect("file not found");

        let mut contents = String::new();
        f.read_to_string(&mut contents)
            .expect("something went wrong reading the file");
    return contents;
        // `file` goes out of scope, and the "hello.txt" file gets closed
}

pub fn read_line_by_line(file_name: &'static str) -> Vec<String> {

    let f = File::open(file_name).expect("file not found");

    let file: BufReader<&File> = BufReader::new(&f);
    let lines_read: Lines<BufReader<&File>> = file.lines();

    let mut all_lines_vec: Vec<String> = Vec::new();
    for line in lines_read
        {
            let ll = line.unwrap();
            all_lines_vec.push(ll);
        }

    return all_lines_vec;

}
