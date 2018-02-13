

extern crate ascii;
use std::io::Read;
use std::i32;
use std::i64;
use std::string::String;
use std::convert::From;
use std::collections::HashMap;
use super::parse_utils;


pub fn day_23_b_test_2() {
    let instruction_input = parse_utils::read_line_by_line("inp23.txt");
    let a: i128 = 1;
    let mut b: i128 = 106700;
    let c: i128 = b + 17000;
    let mut h: i128 = 0;
    loop {
        let mut f: i128 = 1;
        let mut d: i128 = 2;

        'l: while d != b {
            let mut e: i128 = 2;
            if b % d == 0 {
                f = 0;
                break;
            }
            d = d + 1;
        }

        if f == 0 {
            h = h + 1;
        }
        if b == c {
            println!("FINISHED h Key value is {}", h);
            break;
        }
        b = b + 17;
    }
}

pub fn day_23(){

    let instruction_input = parse_utils::read_line_by_line("inp23.txt");

    let mut value_map: HashMap<String, i64> = HashMap::new();
    value_map.insert("a".to_string(), 1);
    value_map.insert("b".to_string(), 0);
    value_map.insert("c".to_string(), 0);
    value_map.insert("d".to_string(), 0);
    value_map.insert("e".to_string(), 0);
    value_map.insert("f".to_string(), 0);
    value_map.insert("g".to_string(), 0);
    value_map.insert("h".to_string(), 0);
    let mut it: i64 = 0;
    let mut nr_of_times = 0;

    while it >= 0 && it < instruction_input.len() as i64 && nr_of_times < 1000 {
       // println!("Starting a new instruction!");
        let  line = &instruction_input[it as usize];
       // println!("Instruction: {}", line);
       // if it % 1000 == 0 {

        for k in value_map.keys(){
          //  println!("Key is {} and value is {}", k, value_map.get(k).unwrap());
        }
//
//
//        println!("d Key value is {}", value_map.get("d").unwrap());
//
//        println!("e Key value is {}", value_map.get("e").unwrap());

//        println!("h Key value is {}", value_map.get("h").unwrap());
        let mut split = line.split_whitespace().into_iter();
        let instruction: String = split.next().unwrap().to_string();
        let key = split.next().unwrap();

        let val = match split.next() {
            Some(value) => value,
            None => panic!("All rowns should have some kind of value string.")
        };

        let cloned_value_map = value_map.clone();

        match instruction.as_str() {
            "set" => {
                let value_nr = match val.parse::<i64>() {
                    Ok(nr) => nr,
                    Err(error) => *cloned_value_map.get(val).unwrap()
                };
                value_map.insert(key.to_string(), value_nr);
                println!("set: {} with {}", key, val);
                it += 1;
            },
            "mul" => {
                value_map = operate(key.to_string(), val, value_map, &cloned_value_map, &multiply);
                println!("mul: {} with {}", key, val);
                nr_of_times += 1;
                it += 1;
            },
            "jnz" => {
                let key_nr: i64 = match key.parse::<i64>() {
                   Ok(keynr) => keynr,
                   Err(error) => *cloned_value_map.get(key).unwrap()
                };
                let jump_nr: i64 =  match val.parse::<i64>() {
                    Ok(nr) => nr,
                    Err(error) => *cloned_value_map.get(val).unwrap()
                };
                if key_nr == 0 {
                   it += 1; // NO JUMP IF ZERO

                }
                else {
                    println!("jnz: {} with {}", key, val);
                    it += &jump_nr;
                    if "g" == key.to_string()  {
                                        println!("d Key value is {}", value_map.get("d").unwrap());
                                        println!("e Key value is {}", value_map.get("e").unwrap());
                                        println!("b Key value is {}", value_map.get("b").unwrap());
                                    }

                }


            },
            "sub" => {
                value_map = operate(key.to_string(), val, value_map, &cloned_value_map,  &sub);
                it += 1;
                println!("sub: {} with {}", key, val);
                if "h" == key.to_string()  {
                    println!("c Key value is {}", value_map.get("c").unwrap());
                    println!("b Key value is {}", value_map.get("b").unwrap());
                    println!("h Key value is {}", value_map.get("h").unwrap());
                }
            },
            _ => panic!("Unknown instruction.")

        }

    }


    println!("Multiple is nr of times: {}", nr_of_times);
}



fn operate(key: String, val: &str, mut value_map: HashMap<String, i64>,
           mut cloned_value_map: &HashMap<String, i64>, operator: &Fn(i64, i64) -> i64) -> HashMap<String, i64> {

       // println!("A val is found at least");
        let initial_val_option = cloned_value_map.get(key.as_str());
        if initial_val_option.is_some() {
            let initial_val = initial_val_option.unwrap();
            if let Ok(nr) = val.parse::<i64>() {
                value_map.insert(key, operator(*initial_val, nr));
            } else {
                let new_val_option = cloned_value_map.get(val);
                match new_val_option {
                    Some(new_val) => {
                        value_map.insert(key, operator(*initial_val, *new_val));
                    },
                    None => {
                        eprintln!("operate did not succeed");
                    }
                }
            }
        }
        else {
            eprintln!("operate did not succeed");
        }



    return value_map;
}

fn multiply(v1: i64, v2: i64) -> i64 {
    return v1 * v2;
}

fn sub(v1: i64, v2: i64) -> i64 {
    return v1 - v2;
}
