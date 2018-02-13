

use std::str::Chars;
use std::io::Read;
use super::parse_utils;
pub fn day1()
{
    println!("Result day 1: ");
    day1a();
    day1b();
}


fn day1a()
{
    let inp = parse_utils::parse_input_file("inp.txt");
    let characters: Vec<char> = inp.chars().collect();

    let first = characters.get(0).unwrap();

    let mut sum: u32 = 0;
    let mut iter = 1;
    while let Some(nr) = characters.get(iter) {
        if let Some(ff) = characters.get(iter) {
            if nr == ff {
                let nu = nr.to_digit(10).unwrap();
                sum = sum + nu;
            }
        } else if characters.get(iter).is_none() && nr == first {
            let nu = nr.to_digit(10).unwrap();
            sum = sum + nu;
        }
        iter = iter + 1;
    }

    println!("{}", sum);
}

fn day1b()
{


    let inp = parse_utils::parse_input_file("inp.txt");
    let mut chars: Chars = inp.chars();
    let numbers: Chars = chars.clone();

    let lll = numbers.clone().count();
    let mut nr = chars.next();
    let mut sum: u32 = 0;
    let mut iter: usize = 0;
    let mut siter: usize = 0;
    while nr.is_some() {
        let mut numm: Chars = numbers.clone();
        let ind = (iter + lll / 2) as usize;
        if ind >= lll
            {
                let rest = lll - iter;
                let index = siter;
                let char_comp: char = numm.nth(index).unwrap();

                if nr.unwrap() == char_comp {
                    let nu = nr.unwrap().to_digit(10).unwrap();
                    sum = sum + nu;
                }
                //nr = chars.next();
                siter = siter + 1;
            } else {
            let char_comp: char = numm.nth(ind).unwrap();

            if nr.unwrap() == char_comp {
                let nu = nr.unwrap().to_digit(10).unwrap();
                sum = sum + nu;
            }
        }
        nr = chars.next();
        iter = iter + 1;
    }
    println!("{}", sum);
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_some_stuff() {
       // assert_eq!("0","m");
        super::day1a();// 10173)
        super::day1b();// 1072)
    }
}

