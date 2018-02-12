
extern crate regex;
extern crate ascii;
extern crate num_traits;
use std::io::Read;
use std::i32;
use std::i64;

use std::char;
use std::string::String;
use std::convert::From;
use std::collections::HashMap;
use std::str;
use super::print_utils;
use super::parse_utils;

use  std::cmp::Ord;
use std::cmp::Ordering;
use std::str::Chars;
use std::num;
use dec_20::num_traits::abs;



pub fn day_20()
{
    let instruction_input:Vec<String> = parse_utils::read_line_by_line("inp20.txt");

    let re = regex::Regex::new(r"<(.*?)>").unwrap();
    println!("Regex");

    let mut particles:Vec<Particle> = instruction_input.into_iter().map(|p| {
        let mut caps = re.captures_iter(&p);
        let coords_text = caps.next().unwrap().get(1).map_or("", |m| m.as_str());
        let vel_text = caps.next().unwrap().get(1).map_or("", |m| m.as_str());
        let acc_text = caps.next().unwrap().get(1).map_or("", |m| m.as_str());
        println!("Point: {}", coords_text);
        println!("Vel: {}", vel_text);
        println!("Acc: {}", acc_text);
        let coord: Vec<&str> = coords_text.split(',').collect();
        let  co: Vec<i32>  = coord.into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
        let velocity: Vec<&str> = vel_text.split(',').collect();
        let vel: Vec<i32> = velocity.into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
        let acceleration: Vec<&str> = acc_text.split(',').collect();
        let acc : Vec<i32>= acceleration.into_iter().map(|s| s.parse::<i32>().unwrap()).collect();
        return  Particle{coordinates: (co[0],co[1],co[2]),
            velocity: (vel[0],vel[1],vel[2]), acceleration: (acc[0],acc[1],acc[2]), p_index: 0};
    }).collect();

   // let mut particles:Vec<&Particle>
    for it in 0..1000 {
        // Update all particles.
//        let old_particles = particles.clone();
//        let old_particles2 = particles.clone();
       // let closest_to_origo = &particles.iter().min().unwrap();
       // println!("Closest to origo: {}", closest_to_origo.p_index);
        let old_parts = particles.clone();
        particles = old_parts.into_iter().map(| p| {
            let n = update(p);
//            let  clone = Particle {coordinates: p.coordinates, velocity: p.velocity,
//                acceleration: p.acceleration, p_index: p.p_index};
           // p.update();
            return n;
        }).collect();

    }
    for p in &particles{
        println!("Point: {}, {}, {}", p.coordinates.0, p.coordinates.1, p.coordinates.2);
    }
    particles.sort_by_key(|co| abs(co.coordinates.0)  + abs(co.coordinates.1) + abs(co.coordinates.2));
    for p in &particles{
        println!("Point: {}, {}, {}", p.coordinates.0, p.coordinates.1, p.coordinates.2);
    }
}


#[derive(Clone)]
struct Particle {
    coordinates: (i32, i32, i32),
    velocity: (i32, i32, i32),
    acceleration: (i32, i32, i32),
    p_index: i32
}

    fn update(p: Particle) -> Particle {
        return Particle{ coordinates: (p.coordinates.0 + p.velocity.0, p.coordinates.1 + p.velocity.1, p.coordinates.2 + p.velocity.2),
        velocity: (p.velocity.0 + p.acceleration.0, p.velocity.1 + p.acceleration.0, p.velocity.1 + p.acceleration.2),
        acceleration: (p.acceleration.0, p.acceleration.1, p.acceleration.2), p_index: p.p_index};
    }


impl Ord for Particle {
    fn cmp(&self, other: &Particle) -> Ordering {
        let self_sum = self.coordinates.0 + self.coordinates.1 + self.coordinates.2;
        let other_sum = other.coordinates.0 + other.coordinates.1 + other.coordinates.2;
        return self_sum.cmp(&other_sum);
    }
}

impl PartialOrd for Particle {
    fn partial_cmp(&self, other: &Particle) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Particle {
    fn eq(&self, other: &Particle) -> bool {
        let self_sum = self.coordinates.0 + self.coordinates.1 + self.coordinates.2;
                let other_sum = other.coordinates.0 + other.coordinates.1 + other.coordinates.2;
                return self_sum == other_sum;
    }
}

impl Eq for Particle {}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_on_inp()
    {
        super::day_20();
    }
}
