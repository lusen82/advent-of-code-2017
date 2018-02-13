//extern crate regex;
//extern crate ascii;
//use std::io::Read;
//use std::i32;
//use std::i64;
//
//use std::char;
//use std::string::String;
//use std::convert::From;
//use std::collections::HashMap;
//use super::print_utils;
//use super::parse_utils;
//
//use std::str::Chars;
//use std::sync::mpsc::{Sender, Receiver};
//use std::sync::mpsc;
//use std::thread;
//use std::thread::JoinHandle;
//use std::time::Duration;
//
//pub fn day11(){

// USE WINDOWS VERSION, ITS LATER!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
//    let cube_directions = vec!(Cube {x: 1, y: -1,  z:0}, Cube{x: 1, y:  0,z: -1}, Cube{x: 0, y: 1,z: -1},
//                               Cube { x: -1, y: 1, z: 0 }, Cube { x: -1, y: 0, z: 1 }, Cube { x: 0, y: -1, z: 1 });
//    let real = parse_utils::parse_input_file("inp11.txt");
//    let s = "se,sw,se,sw,sw";
//    let s2 = "ne,ne,s,s";
//    let s3 = "ne,ne,sw,sw";
//    let s4 = "ne,ne,ne";
//    let s5 = "n,s,s,n,n,nw,nw,ne,ne,se,sw,se,sw,sw";
//    let s6 = "n,s,s,ne,ne,nw,nw,se,s,se,sw,se,sw,sw";
//    let s7 = "n,s,s,ne,ne,s,nw,se,s,se,sw,se,s,sw";
//    let mut instructions = s7.split(',');
//    let mut current_cube = Cube{x: 0, y: 0, z: 0};
//    while let Some(next_val) = instructions.next() {
//        let direction: usize = match next_val {
//
//            "se" => 0,
//            "ne" => 1,
//            "n" => 2,
//            "nw" => 3,
//            "sw" => 4,
//            "s" => 5,
//            _ => 10
//        };
//
//        current_cube = cube_neighbor(current_cube, direction, &cube_directions);
//       // println!("X: {}", &current_cube.x);
//
//    }
//    println!("X: {}", &current_cube.x);
//
//    let hex = cube_to_oddq(&current_cube);
//
//    println!(": {}", &current_cube.x);
//}
//
//fn cube_to_axial(cube: Cube) -> Hex {
//    let q = cube.x;
//    let r = cube.z;
//    return Hex{col: q, row: r};
//}
//
//fn axial_to_cube(hex: Hex) -> Cube {
//
//    let x = hex.col;
//    let z = hex.row;
//    let y = -x-z;
//    Cube{x, y, z}
//}
//
//
//fn cube_neighbor(cube: Cube, direction: usize, cube_directions: &Vec<Cube>) -> Cube {
//
//    cube_add(cube, &cube_directions[direction])
//}
//
//
//fn cube_add(cube: Cube, cube_direction: &Cube) -> Cube
//{
//   let x = cube.x;
//    let y = cube.y;
//    let z = cube.z;
//    let add_x = cube_direction.x;
//       let add_y = cube_direction.y;
//       let add_z = cube_direction.z;
//   Cube{x: x + add_x, y: y + add_y, z: z + add_z}
//}
//
//struct Hex {
//   col: i32,
//    row: i32
//}
//
//struct Cube {
//    x: i32,
//    y: i32,
//    z: i32
//}
//
//fn cube_to_oddq(cube: Cube) ->Hex {
//
//      let col = cube.x;
//    let row = cube.z + (cube.x - (cube.x&1)) / 2;
//      return Hex{col, row};
//}
//
//fn oddq_to_cube(hex: Hex) -> Cube {
//
//    let x = hex.col;
//    let z = hex.row - (hex.col - (hex.col&1)) / 2;
//    let y = -x-z;
//      return Cube{x, y, z};
//}
//
//
//
//#[cfg(test)]
//mod tests {
//
//    #[test]
//    fn test_11() {
//
//
//    }
//
//}
