extern crate regex;
extern crate ascii;
use std::i32;
use super::parse_utils;

pub fn day2()
{
    println!("Sum a {} ", get_dec_second_a());
    println!("Sum b {} ", get_dec_second_b());

}

fn get_dec_second_a() -> i32 {
    let inp = parse_utils::read_line_by_line("inp2.txt");

    let sum = inp.iter().fold(0, (|acc, val| {
        let mut int_values: Vec<i32>  = val.split_whitespace().map(|s| s.parse().unwrap()).collect();
        int_values.sort();
        let min = int_values.first().unwrap();
        let max = int_values.last().unwrap();

        let diff: i32 = max - min;
        return acc + diff;
    }));
    return sum;
}


pub fn get_dec_second_b() -> i32 {
    let inp = parse_utils::read_line_by_line("inp2.txt");
    let  sum = inp.iter().fold(0, (|acc, val| {
        let int_values: Vec<i32> = val.split_whitespace().map(|s| s.parse().unwrap()).collect();

        for nr1 in &int_values {
            for nr2 in &int_values {
                if nr1 != nr2 && nr1 % nr2 == 0 {
                    return acc + nr1 / nr2;
                }
            }
        }
        return acc;
       }));

    return sum;
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_some_stuff() {
       // assert_eq!("0","m");
        super::day2();// 45972, 326
    }
}
