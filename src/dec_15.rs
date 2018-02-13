extern crate regex;
extern crate ascii;
use std::io::Read;
use std::i32;
use std::i64;
use std::str::Chars;

use std::string::String;
use std::convert::From;
use std::cell::RefCell;
use std::cell::RefMut;
use std::cell::Ref;
use super::print_utils;
use super::parse_utils;


pub fn day_15() {

    println!("Running 15 a.");
    let factor_a = 16807;
    let factor_b = 48271;

    let divider = 2147483647;

    let start_a = 65;//591;
    let start_b = 8921;//393;

    let mut generated_a: i128 = start_a;
    let mut generated_b: i128 = start_b;
    let mut nr_of_matches = 0;
    let tries: i128 = 40000000;
    for it in 0..tries {
        let previous_a = generated_a;
        let previous_b = generated_b;
        let multi_a: i128 = previous_a * factor_a;
        let multi_b: i128 = previous_b * factor_b;

        // x % 7 == (x+x/7) & 7
//        generated_a = multi_a % divider; -> generated_a = (multi_a + multi_a/divider) & divider;

        generated_a = (multi_a + multi_a/divider) & divider;
        generated_b = (multi_b + multi_b/divider) & divider;

        // The right most bits create a bit_mask and compare:
        let bits = 16;
        let bit_mask = (1 << bits) - 1;
        if (generated_a & bit_mask) == (generated_b & bit_mask) {
            nr_of_matches +=1;
        }
    }
    println!("Number OF MATCHES: {}", nr_of_matches);

}
pub fn day_15_b() {

    let factor_a = 16807;
    let factor_b = 48271;

    let divider = 2147483647;

    let start_a = 591;//591, 65;
    let start_b = 393;//393, 8921;

    let tries: usize = 5000000;

    let found_for_generator_a = get_generated_numbers(start_a, factor_a, 3, tries, divider);
    let found_for_generator_b = get_generated_numbers(start_b, factor_b, 7, tries, divider);
    let mut nr_of_matches = 0;
    for it in 0..tries {
        let a: i128 = found_for_generator_a[it];
        let b: i128 = found_for_generator_b[it];
        let bits = 16;
        let bit_mask = (1 << bits) - 1;
        if (a & bit_mask) == (b & bit_mask) {
            nr_of_matches +=1;
        }
    }
    println!("Number OF MATCHES: {}", nr_of_matches);

}

fn get_generated_numbers(start: i128,
                         factor: i128, mod_bit_and: i128, tries: usize, divider: i128) -> Vec<i128> {
    let mut generated: i128 = start;
    let mut found_for_generator: Vec<i128> = Vec::new();
    while found_for_generator.len() < tries {
        let previous = generated;
        let multi: i128 = previous * factor;
        generated = (multi + multi / divider) & divider;
        if generated & mod_bit_and == 0 {
            found_for_generator.push(generated);
        }
    }
    found_for_generator
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_15() {
//         Resutl for:
//        let start_a = 65;
//        let start_b = 8921;
        // Is 588.
        //    Generator A starts with 591
        //    Generator B starts with 393
        // Should give 619 (mine was 618, weird..)
        super::day_15();

    }

}
