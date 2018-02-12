
extern crate regex;
extern crate ascii;
use std::str::SplitWhitespace;
use std::string::String;

use super::parse_utils;


pub fn day4() {
    let mut inp: Vec<String> = parse_utils::read_line_by_line("inp4.txt");
    inp.retain(|r| require_unique_anagrams(r));
    println!("Result day 4: ");
    println!("{}", inp.len());
}


fn require_unique_anagrams(inn: &str) -> bool
{
    let whitespace: SplitWhitespace = inn.split_whitespace();
    let initial_words: Vec<&str> = whitespace.collect();

    let original_count_of_words = &initial_words.len();

    let mut vec: Vec<String> = initial_words.into_iter().map(|w| get_sorted_characters_in_words(w)).collect();

    vec.sort();
    vec.dedup();

    return vec.len() == *original_count_of_words

}

fn get_sorted_characters_in_words(inn: &str) -> String
{
    let mut chars: Vec<char> = inn.chars().collect();
    chars.sort();
    return chars.into_iter().collect::<String>();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_some_stuff() {
       // assert_eq!("0","m");
        super::day4();// 45972, 326
    }
}


