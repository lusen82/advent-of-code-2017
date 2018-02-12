
extern crate regex;
extern crate ascii;
use std::string::String;
use std::i32;
use std::collections::HashMap;
use super::parse_utils;


pub fn day_8()
{
    let inp: Vec<String> = parse_utils::read_line_by_line("inp8.txt");
    let mut current_values0: Vec<String> = inp.iter().map(|row| get_current_values(&row)).collect();
    current_values0.sort();
    current_values0.dedup();
    //let v2: Vec<&str> = v.iter().map(|s| &**s).collect();
    let current_values: Vec<&str> = current_values0.iter().map(|s| &**s).collect();

    let value_list: Vec<(String, i32)> = current_values.iter().map(|std| get_tuple(std)).collect();
    let mut value_map: HashMap<String, i32> = value_list.into_iter().collect();
    let length_of_inp = &inp.len();
    let mut current_max_value: i32 = 0;
    for input in inp {
        let apply: Vec<&str> = input.split_whitespace().collect();
        let to_update: &str = apply[0];
        let dec_inc = apply[1];
        let update_value = apply[2];
        let update_value_i32: i32 = update_value.parse().unwrap();
        let  update_value_signed_i32 = match dec_inc {
            "dec" => (-1) * update_value_i32,
            "inc" => update_value_i32,
            _ => 0, // throw exception?
        };
        let conditional_varible = apply[4];
        let condition: String = apply[5].to_string();
        let condition_value = apply[6];
        let condition_value_i32 = condition_value.parse().unwrap();

        let current_value: i32 = *value_map.get_mut(to_update).unwrap();// value_map[to_update];
        let new_value: i32 = current_value + update_value_signed_i32;
        let condition_bool = match condition.as_str() {
            "<" => value_map[conditional_varible] < condition_value_i32,
            "<=" => value_map[conditional_varible] <= condition_value_i32,
            "==" => value_map[conditional_varible] == condition_value_i32,
            ">" => value_map[conditional_varible] > condition_value_i32,
            ">=" => value_map[conditional_varible] >= condition_value_i32,
            "!=" => value_map[conditional_varible] != condition_value_i32,
            &_ => false // throw exception?
        };
        let non_borrowed: String = to_update.to_owned();
        if condition_bool {
           // println!("Replacing {} with {} for {}.", current_value, new_value, to_update);
            current_max_value = current_max_value.max(new_value);
            value_map.remove(to_update);
            value_map.insert(non_borrowed, new_value);
        }
    }

    let max_value = value_map.values().max().unwrap();
    println!("Size of input = {}", length_of_inp);
    println!("Size of unique = {}", current_values.len());
    println!("{}", max_value);
    println!("Max of all = {}", current_max_value);


}



fn get_current_values(row: &str) -> String
{
    return row.split_whitespace().next().unwrap().parse().unwrap()
}


fn get_tuple(std: &str) -> (String, i32)
{
    let string_val: String = std.to_owned();
    let int_val: i32 = get_zero_i32();
    return  (string_val, int_val);
}

fn get_zero_i32() -> i32
{
    return 0;
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_on_inp()
    {
        super::day_8();
    }
}

