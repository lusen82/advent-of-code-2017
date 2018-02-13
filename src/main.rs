//#![feature(dedup_by_key)]
#![feature(i128_type)]
//mod dd;
extern crate regex;
extern crate ascii;
use std::string::String;
use std::convert::From;
mod dec_1;
mod dec_2;
mod dec_3;
mod dec_4;
mod dec_5;
mod dec_6;
mod dec_7;
mod dec_8;
mod dec_9;
pub mod  dec_10;
mod  dec_11;
mod  dec_14;
mod  dec_15;
mod  dec_16;
mod  dec_18;
mod  dec_20;
mod  dec_23;
mod  print_utils;
mod  parse_utils;





fn main() {
    let all = false;
    dec_23::day_23_b_test_2();
    //dec_18::day_18_b();
    if all {
   // dec_11::day11();
        dec_20::day_20();
        dec_16::day_16_b();
        dec_15::day_15_b();
        dec_14::day_14();
        dec_1::day1();
        dec_2::day2();
        dec_4::day4();
        dec_5::day_5();
        dec_6::day6();

        dec_7::day_7();
        dec_8::day_8();
        dec_9::day_9();
        dec_10::day_10b();
        let real = parse_utils::parse_input_file("inp11.txt");
        let s = "se,sw,se,sw,sw";
        let s2 = "ne,ne,s,s";
        let s3 = "ne,ne,sw,sw";
        let s4 = "ne,ne,ne";
        let s5 = "n,s,s,n,n,nw,nw,ne,ne,se,sw,se,sw,sw";
        let s6 = "n,s,s,ne,ne,nw,nw,se,s,se,sw,se,sw,sw";
        let s7 = "n,s,s,ne,ne,s,nw,se,s,se,sw,se,s,sw";
        let v: Vec<&str> = real.split(',').collect();
        let mut  text: Vec<String> = v.iter().map(|inp| String::from(*inp)).collect();
        text.sort();
        print_utils::print_vector_str(&text);
        let mut remaining = collapse_possible_n_s(&text);
        println!("Count 0: {}", &remaining.len());
        print_utils::print_vector_str(&remaining);
        let mut rem2 = collapse_possible_sw_ne(&remaining);
        println!("Count 1: {}", &rem2.len());
        print_utils::print_vector_str(&rem2);
        let mut rem3 = collapse_possible_nw_se(&rem2);


        println!("Count 1: {}", &rem3.len());
        print_utils::print_vector_str(&rem3);

        let mut rem5 = collapse_possible_nw_ne(&rem3);
        println!("Count 2: {}", &rem5.len());
        print_utils::print_vector_str(&rem5);

        let mut rem4 = collapse_possible_sw_se(&rem5);
        println!("Count 3: {}", &rem4.len());
        print_utils::print_vector_str(&rem4);
//
//        let mut rem = collapse_possible_n_s(&rem5);
//        println!("Count: {}", &rem.len());
//        print_utils::print_vector_str(&rem);

        let mut rem6 = collapse_possible_s_ne(&rem4);
        println!("Count 4: {}", &rem6.len());
        print_utils::print_vector_str(&rem6);

        let mut rem7 = collapse_possible_n_sw(&rem6);
        println!("Count 5: {}", &rem7.len());
        print_utils::print_vector_str(&rem7);

//    let mut rem4 = collapse_possible_sw_se(&rem3);
//    let mut rem5 = collapse_possible_nw_ne(&rem4);
//    print_vector_str(&rem5);
//    let mut remainingA = collapse_possible_n_s(&rem5);
//        let mut rem2A = collapse_possible_sw_ne(&remainingA);
//        let mut rem3A = collapse_possible_nw_se(&rem2A);
//        let mut rem4A = collapse_possible_sw_se(&rem3A);
//    let mut rem5A = collapse_possible_nw_ne(&rem4A);
//        let mut rem6A = collapse_possible_s_ne(&rem5A);
//
//    print_vector_str(&rem6A);
//
//
//    println!("Count: {}", &rem6A.len());
        //}
    }




}

fn collapse_possible_n_s<'a>(text: &Vec<String>) -> Vec<String>{
    let mut remaining: Vec<String> = Vec::new();
    remaining = rule_out("s", "n", remaining, text);
    remaining = copy_over("se", remaining, text);
    remaining = copy_over("ne", remaining, text);
    remaining = copy_over("sw", remaining, text);
    remaining = copy_over("nw", remaining, text);

    return  remaining;
}

fn rule_out(first: &str, second: &str, mut input: Vec<String>, text: &Vec<String> ) -> Vec<String>{
    let nr_first = text.iter().filter(|c| **c == String::from(first)).count();
    let nr_sec = text.iter().filter(|c| **c == String::from(second)).count();


    if nr_first > nr_sec {
        let amount: usize = nr_first - nr_sec;
        let vec: Vec<String> = vec![String::from(first); amount];
        input.extend(vec);
    }
    else if nr_sec > nr_first {
        let i = nr_sec - nr_first;
        input.extend(vec![String::from(second); i]);
    }

    return  input;
}

fn combine(first: &str, second: &str, replace_to: &str, mut input: Vec<String>, text: &Vec<String> ) -> Vec<String>{
   let nr_of_first = text.iter().filter(|c| c == &&first).count();
   let nr_of_second = text.iter().filter(|c| c == &&second).count();
   let to_replace = nr_of_first.min(nr_of_second);
   input.extend(vec![String::from(replace_to); to_replace]);
   if nr_of_second > nr_of_first {
       input.extend(vec![String::from(second); nr_of_second-nr_of_first]);
   } else if nr_of_first > nr_of_second {
       input.extend(vec![String::from(first); nr_of_first-nr_of_second]);
   }
    return  input;
}
fn copy_over(instruction: &str, mut input: Vec<String>, text: &Vec<String> ) -> Vec<String>{
    let nr = text.iter().filter(|c| c == &&instruction).count();
    let vec = vec![String::from(instruction); nr];
    //println!("nr {}", nr);
    //println!("vec size {}", &vec.len());
   // print_vector_str(&vec);
    input.extend(vec);
    return input;
}

fn collapse_possible_sw_ne(text: &Vec<String>) -> Vec<String> {

    let mut remaining: Vec<String> = Vec::new();
    remaining = rule_out("sw", "ne", remaining, text);
    remaining = copy_over("s", remaining, text);
    remaining = copy_over("n", remaining, text);
    remaining = copy_over("se", remaining, text);
    remaining = copy_over("nw", remaining, text);

    return  remaining;
}

fn collapse_possible_nw_se(text: &Vec<String>) -> Vec<String> {

    let mut remaining: Vec<String> = Vec::new();
    remaining = rule_out("se", "nw", remaining, text);
    remaining = copy_over("s", remaining, text);
    remaining = copy_over("n", remaining, text);
    remaining = copy_over("sw", remaining, text);
    remaining = copy_over("ne", remaining, text);

    return  remaining;
}

fn collapse_possible_sw_se(text: &Vec<String>) -> Vec<String> {

    let mut remaining: Vec<String> = Vec::new();
    remaining = combine("sw", "se", "s", remaining, text);

   remaining = copy_over("s", remaining, text);
   remaining = copy_over("n", remaining, text);
   remaining = copy_over("ne", remaining, text);
   remaining = copy_over("nw", remaining, text);


    return  remaining;
}


fn collapse_possible_nw_ne(text: &Vec<String>) -> Vec<String> {


    let mut remaining: Vec<String> = Vec::new();
    remaining = combine("nw", "ne", "n", remaining, text);

    remaining = copy_over("s", remaining, text);
    remaining = copy_over("n", remaining, text);
    remaining = copy_over("se", remaining, text);
    remaining = copy_over("sw", remaining, text);

    return  remaining;
}
//
//fn collapse_possible_n_se(text: &Vec<String>) -> Vec<String> {
//    let mut remaining: Vec<String> = Vec::new();
//    remaining = combine("s", "ne", "se", remaining, text);
//
//    remaining = copy_over("n", remaining, text);
//    remaining = copy_over("nw", remaining, text);
//    remaining = copy_over("se", remaining, text);
//    remaining = copy_over("sw", remaining, text);
//
//    return  remaining;
//}

fn collapse_possible_s_ne(text: &Vec<String>) -> Vec<String> {
    let mut remaining: Vec<String> = Vec::new();
    remaining = combine("s", "ne", "se", remaining, text);

    remaining = copy_over("n", remaining, text);
    remaining = copy_over("nw", remaining, text);
    remaining = copy_over("se", remaining, text);
    remaining = copy_over("sw", remaining, text);

    return  remaining;
}

fn collapse_possible_n_sw(text: &Vec<String>) -> Vec<String> {
    let mut remaining: Vec<String> = Vec::new();
    remaining = combine("n", "sw", "nw", remaining, text);

    remaining = copy_over("s", remaining, text);
    remaining = copy_over("nw", remaining, text);
    remaining = copy_over("se", remaining, text);
    remaining = copy_over("ne", remaining, text);

    return  remaining;
}








