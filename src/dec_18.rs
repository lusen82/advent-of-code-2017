extern crate regex;
extern crate ascii;
use std::io::Read;
use std::i32;
use std::i64;

use std::char;
use std::string::String;
use std::convert::From;
use std::collections::HashMap;
use super::print_utils;
use super::parse_utils;

use std::str::Chars;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;



pub fn day_18_b()
{
    let (tx1, rx1): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let (tx2, rx2): (Sender<i64>, Receiver<i64>) = mpsc::channel();
    let t1: JoinHandle<()> = thread::spawn(|| {
        day_18_b_impl(tx1, rx2, 0);
        println!("thread {} finished", 0);
    });

    // Each thread will send its id via the channel
    let t2 = thread::spawn(|| {
        day_18_b_impl(tx2, rx1, 1);
        println!("thread {} finished", 1);
    });

    println!("Try joining.");
    match t1.join() {
       Ok(r) => println!("Ready"),
       Err(e) => println!("Error"),
    }

    match t2.join() {
       Ok(r) => println!("Ready"),
       Err(e) => println!("Error"),
    }
    println!("DONE");

}

pub fn day_18_b_impl(thread_tx: Sender<i64>, thread_rx: Receiver<i64>, id: i32)
{
    let instruction_input = parse_utils::read_line_by_line("inp18.txt");

    let queue: Vec<i64> = vec![];
    let mut value_map: HashMap<String, i64> = HashMap::new();
    value_map.insert("p".to_string(), id as i64);
    let mut it: i64 = 0;
    let mut current_sound = 0;

    while it >= 0 && it < instruction_input.len() as i64 {
        // println!("Starting a new instruction!");
        let  line = &instruction_input[it as usize];
       // println!("Instruction: {} in program {}", line, id);

        for k in value_map.keys(){
           // println!("Key is {} and value is {}", k, value_map.get(k).unwrap());
        }

        let mut split = line.split_whitespace().into_iter();
        let instruction: String = split.next().unwrap().to_string();
        let key = split.next().unwrap();
        let value_str = split.next();
        let cloned_value_map = value_map.clone();

        match instruction.as_str() {
            "set" => {
                match value_str {
                    Some(val) => {
                        if let Ok(nr) = val.parse::<i64>() {
                            value_map.insert(key.to_string(), nr);
                        }
                        else {
                            let new_val_option = cloned_value_map.get(val);
                            match new_val_option {
                                Some(new_val) => {
                                    value_map.insert(key.to_string(), *new_val);
                                },
                                None => {

                                }
                            }
                        }
                    }
                    None => {
                        break;
                    }
                }
            },
            "mul" => {
                value_map = operate(key.to_string(), value_str, value_map, &cloned_value_map, &multiply);
            },
            "jgz" => {
                if let Ok(x) = key.parse::<i64>() {
                    if x <= 0 {
                        it += 1;
                        continue;
                    }
                } else {
                    let new_key_option = cloned_value_map.get(key);
                    if new_key_option.is_none() {
                        it += 1;
                        continue;
                    }
                    let new_key = new_key_option.unwrap();
                    if *new_key <= 0 {
                        it += 1;
                        continue;
                    }
                }
                match value_str {
                    Some(val) => {
                        if let Ok(nr) = val.parse::<i64>() {
                            it += nr;
                            continue; // Don't us the normal up count.
                        } else {
                            let new_val_option = cloned_value_map.get(val);
                            if new_val_option.is_some() {
                                it += new_val_option.unwrap();
                                continue; // Don't us the normal up count.
                            }
                        }

                    }
                    None => {
                        continue;
                    }
                }
            },
            "add" => {
                value_map = operate(key.to_string(), value_str, value_map, &cloned_value_map,  &add);
            },
            "mod" => {
                value_map = operate(key.to_string(), value_str, value_map, &cloned_value_map, &modulus);
            },
            "snd" => {
                 current_sound = *cloned_value_map.get(key).unwrap();
                //println!("Updating sound: {}", current_sound);
                    let send_res = thread_tx.send(current_sound);
                match send_res {
                    Ok(s) => println!("PROGRAM {} SENDING {}", id, current_sound),
                    Err(e) => {}
                }
            },
            "rcv" => {
                match thread_rx.recv_timeout(Duration::from_millis(4000)) {
                    Ok(r) => {
                        if r == -1 {
                            println!("OTHER PROGRAM FORCED {} TO EXIT", id);
                            break;
                        }
                        println!("PROGRAM {} RECIEVED, {}", id, r);
                        value_map.insert(key.to_string(), r);
                    },
                    Err(e) => {
                        println!("PROGRAM {} IS SENDING EXIT", id);
                        let send_res = thread_tx.send(-1);
                        break
                    },
                }

            },
            _ => {}

        }
        it += 1;
    }


    println!("Sound for program {}: {}", id, current_sound);
}

pub fn day_18_a(){

    let instruction_input = parse_utils::read_line_by_line("inp18.txt");

    let mut value_map: HashMap<String, i64> = HashMap::new();
    let mut it: i64 = 0;
    let mut current_sound = 0;

    while it >= 0 && it < instruction_input.len() as i64 {
        println!("Starting a new instruction!");
        let  line = &instruction_input[it as usize];
        println!("Instruction: {}", line);

        for k in value_map.keys(){
            println!("Key is {} and value is {}", k, value_map.get(k).unwrap());
        }

        let mut split = line.split_whitespace().into_iter();
        let instruction: String = split.next().unwrap().to_string();
        let key = split.next().unwrap();
        let value_str = split.next();
        let cloned_value_map = value_map.clone();

        match instruction.as_str() {
            "set" => {
                match value_str {
                    Some(val) => {
                        if let Ok(nr) = val.parse::<i64>() {
                            value_map.insert(key.to_string(), nr);
                        }
                        else {
                            let new_val = cloned_value_map.get(val).unwrap();
                            value_map.insert(key.to_string(), *new_val);
                        }
                    }
                    None => {
                        break;
                    }
                }
            },
            "mul" => {
                value_map = operate(key.to_string(), value_str, value_map, &cloned_value_map, &multiply);
            },
            "jgz" => {
                if let Ok(x) = key.parse::<i64>() {
                    if x <= 0 {
                        it += 1;
                        continue;
                    }
                } else {
                    let new_key_option = cloned_value_map.get(key);
                    if new_key_option.is_none() {
                        it += 1;
                        continue;
                    }
                    let new_key = new_key_option.unwrap();
                    if *new_key <= 0 {
                        it += 1;
                        continue;
                    }
                }
                match value_str {
                    Some(val) => {
                        if let Ok(nr) = val.parse::<i64>() {
                            it += nr;
                            continue; // Don't us the normal up count.
                        } else {

                            let new_val_option = cloned_value_map.get(val);
                            if new_val_option.is_some() {
                                it += new_val_option.unwrap();
                                continue; // Don't us the normal up count.

                            }
                        }

                    }
                    None => {
                        continue;
                    }
                }
            },
            "add" => {
                value_map = operate(key.to_string(), value_str, value_map, &cloned_value_map,  &add);
            },
            "mod" => {
                value_map = operate(key.to_string(), value_str, value_map, &cloned_value_map, &modulus);
            },
            "snd" => {
                 current_sound = *cloned_value_map.get(key).unwrap();
                println!("Updating sound: {}", current_sound);
            },
            "rcv" => {
                println!("RECOVER {}", key);
                break;
            },
            _ => {}

        }
        it += 1;
    }


    println!("Sound: {}", current_sound);
}

fn operate(key: String, value_str: Option<&str>, mut value_map: HashMap<String, i64>,
           cloned_value_map: &HashMap<String, i64>, operator: &Fn(i64, i64) -> i64) -> HashMap<String, i64> {

    match value_str {
        Some(val) => {
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
                        None => {}
                    }
                }
            }
        }
        None => {}
    }
    return value_map;
}

fn multiply(v1: i64, v2: i64) -> i64 {
    return v1 * v2;
}

fn add(v1: i64, v2: i64) -> i64 {
    return v1 + v2;
}

fn modulus(v1: i64, v2: i64) -> i64 {
    return v1 % v2;
}


#[cfg(test)]
mod tests {

    #[test]
    fn test_18() {

        let input = "abcde";
        let instructions = "s1,x3/4,pe/b";
        let output = super::dance(input.to_string(), &instructions.to_string());
        assert_eq!("baedc", output);



        let mut programs = "abcde".to_string();
        println!("Start: {}", &programs);
           let instruction_input = "s1,x3/4,pe/b";

           let result = super::dance_many_times(programs, &instruction_input.to_string());

           println!("Output: {}", result);
    }

}
