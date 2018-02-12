extern crate regex;
extern crate ascii;
extern crate num_traits;
use std::string::String;
use std::i32;
use super::parse_utils;
use dec_5::num_traits::ops::checked::CheckedAdd;


pub fn day_5()
{
    // a
    let inp: Vec<String> = parse_utils::read_line_by_line("in5.txt");
    let mut inp_ints: Vec<i32> = inp.into_iter().flat_map(|s| s.parse::<i32>()).collect();
    let length: &usize = &inp_ints.len();
    let mut iter = 0;
    let mut index: usize = 0;
    while index < *length {
        iter = iter + 1;
        //if let Ok(next_move_found) = inp[index].parse::<i32>()  {
        let  next_move_found = inp_ints[index];
        let current_value: i32 = next_move_found + 1;
        inp_ints[index] = current_value;
        let index_i32: i32 = index as i32 + next_move_found;
        if index_i32 < 0 {
            break;
        }
        index = index_i32 as usize;
       // };// else Throw exception?

    }

    println!("Number of iterations: {}", iter);

    // b
    let inp: Vec<String> = parse_utils::read_line_by_line("in5.txt");
    let mut inp_ints: Vec<i32> = inp.into_iter().flat_map(|s| s.parse::<i32>()).collect();

       let length: &usize = &inp_ints.len();
       let mut it = 0;
       let mut index: usize = 0;
       while index < *length {
           it += 1;
           let next_move = inp_ints[index];
           let current_value: i32 = match next_move  {
               n if n < 3  => next_move + 1,
               _  => next_move - 1,
           };
           inp_ints[index] = current_value;
          // let  new_index: Option<Self> = CheckedAdd::checked_add(&index as i32, next_move);
           let index_i32: i32 = index as i32 + next_move;
           if index_i32 < 0 {
               break;
           }
           index = index_i32 as usize;
       }

       println!("Number of iterations: {}", it);
}

mod tests {
    #[test]
    pub fn test_on_inp()
    {
        super::day_5();    // 339351 and 24315397
    }
}
