//#![feature(dedup_by_key)]
//mod dd;
//#![feature(i128_type)]
//mod dd;
extern crate regex;
extern crate ascii;
use std::str::Chars;
use std::str::SplitWhitespace;
use std::str::Split;
use std::string::String;
use std::cmp;
use std::io::{BufReader, BufRead};
use std::io::Lines;
use std::fs::File;
use std::io::Read;
use std::vec::IntoIter;
use std::i32;
use std::convert::From;
use std::collections::HashSet;
use std::iter::FromIterator;
use self::regex::Regex;
use std::iter::Map;
use std::collections::HashMap;
use std::ops::Add;
mod dec_1;
mod dec_2;
mod dec_3;
mod dec_4;
mod dec_5;
mod dec_6;
mod dec_7;
mod dec_8;
mod dec_9;
mod dec_10;
mod dec_11;
mod dec_13;
mod dec_14;
mod dec_15;
mod dec_16;
mod dec_17;
mod dec_18;
mod dec_19;
mod dec_20;
mod dec_21;
mod print_utils;
mod parse_utils;

fn main() {
    dec_1::day1();
    dec_2::day2();
    dec_4::day4();
    dec_5::day_5();
    dec_6::day6();
   // dec_7::day_7();
 //   dec_8::day_8();
    dec_9::day_9();

  //  dec_10::day_10();
 //   dec_10::day_10b();


  //  println!("read {}", it_del);
    // day_12();
    //day_13_solution();
   /* for i in 0..100000{
       // if i%2 ==0
         //   {
                day_13(i);
           // }
    }

    println!("s_x: {}", 0);*/
    dec_18::day_18_b();
    dec_11::day11();
    //day19();
}

