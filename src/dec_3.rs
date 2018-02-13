
extern crate regex;
extern crate ascii;
extern crate num_traits;
use std::io::Read;
use std::i32;
use std::i64;

use std::char;
use std::string::String;
use std::convert::From;
use std::collections::HashSet;
use std::collections::HashMap;
use std::str;
use super::print_utils;
use super::parse_utils;

use  std::cmp::Ord;
use std::cmp::Ordering;
use std::str::Chars;
use std::num;
use std::iter::FromIterator;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


pub fn day_3()
{
    let input: u64 = 347991;       //344569 (587) , 351649 (593)

    let primes: Vec<u64> = primes_up_to(592);
    let last_prime = primes.last().unwrap();

    let nr_of_steps_down_to_corner = primes.len() - 1;
    let nr_of_steps_right_to_corner = primes.len() - 1;

    let difference_between_value_and_prime = input - *last_prime;

    println!("difference_between_value_and_prime: {}", difference_between_value_and_prime);
    println!("nr_of_steps_down_to_corner: {}", nr_of_steps_down_to_corner);
    println!("Last prime: {}", last_prime);

    // Size of quadrat is prime + 1.
}

pub fn day_3_b()
{
    let input: u64 = 347991;       //344569 (587) , 351649 (593)

    let mut matrix = vec![vec![]];

    matrix[1000][1000] = 1;
    let mut i = 0;
    let mut j = 0;
    loop {
        // Step right:
        matrix[i] [j + 1] = sum_neighbours(&matrix, i, j + 1);

    }
    let primes: Vec<u64> = primes_up_to(592);
    let last_prime = primes.last().unwrap();

    let nr_of_steps_down_to_corner = primes.len() - 1;
    let nr_of_steps_right_to_corner = primes.len() - 1;

    let difference_between_value_and_prime = input - *last_prime;

    println!("difference_between_value_and_prime: {}", difference_between_value_and_prime);
    println!("nr_of_steps_down_to_corner: {}", nr_of_steps_down_to_corner);
    println!("Last prime: {}", last_prime);

    // Size of quadrat is prime + 1.
}

fn sum_neighbours(matrix: &Vec<Vec<i64>>, i: usize, j: usize) -> i64 {
    let top_left = matrix[i + 1][j - 1];
    let top = matrix[i + 1][j];
    let top_right = matrix[i + 1][j + 1];
    let right = matrix[i ][j + 1];
    let bottom_right = matrix[i - 1][j + 1];
    let bottom = matrix[i - 1][j];
    let bottom_left = matrix[i - 1][j - 1];
    let left = matrix[i ][j - 1];
    return top_left + top + top_right + bottom+ bottom_right + bottom_left + left + right;
}


fn primes_up_to(limit: u64) -> Vec<u64> {
    let mut vec: Vec<_> = (2..limit).collect();

    for p in 2..limit {
        vec.retain(|&x| x <= p || x % p != 0);
    }

    vec
}


#[cfg(test)]
mod tests {
    #[test]
    pub fn test_on_inp()
    {
        super::day_3();
    }
}
