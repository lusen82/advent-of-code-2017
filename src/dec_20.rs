
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
use dec_20::num_traits::abs;

use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};


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
    //    println!("Point: {}", coords_text);
      //  println!("Vel: {}", vel_text);
        //println!("Acc: {}", acc_text);
        let coord: Vec<&str> = coords_text.split(',').collect();
        let  co: Vec<i64>  = coord.into_iter().map(|s| s.parse::<i64>().unwrap()).collect();
        let velocity: Vec<&str> = vel_text.split(',').collect();
        let vel: Vec<i64> = velocity.into_iter().map(|s| s.parse::<i64>().unwrap()).collect();
        let acceleration: Vec<&str> = acc_text.split(',').collect();
        let acc : Vec<i64>= acceleration.into_iter().map(|s| s.parse::<i64>().unwrap()).collect();
        return  Particle{coordinates: (co[0],co[1],co[2]),
            velocity: (vel[0],vel[1],vel[2]), acceleration: (acc[0],acc[1],acc[2]), p_index: 0};
    }).collect();

    let mut data: HashMap<i64, Vec<i64>> = HashMap::new();

    particles.iter().for_each(|particle| {
        data.insert(particle.p_index, Vec::new());
    });

    let mut hashSet: HashSet<Particle> = HashSet::from_iter(particles);

    for it in 0..10000 {
        // Update all particles.
        let old_parts = hashSet.clone();

        hashSet = old_parts.into_iter().map(| mut p| {
            let mut y_values: &mut Vec<i64> = data.get_mut(&p.p_index).unwrap();
            let sum: i64 = abs(*&p.coordinates.0) + abs(*&p.coordinates.1) + abs(*&p.coordinates.2);
            y_values.push(sum);
            let n = update(p);
            return n;
        }).collect();
       // println!("Point bef: {}, {}, {}", &particles[0].coordinates.0, &particles[0].coordinates.1, &particles[0].coordinates.2);

//        particles.sort_by_key(|co| abs(co.coordinates.0)  + abs(co.coordinates.1) + abs(co.coordinates.2));
//
//        particles.dedup();
//
//        for i in 0..particles.len(){
//            let found = particles.iter().filter(|f| f.coordinates.0 == particles[i].coordinates.0
//            && f.coordinates.1 == particles[i].coordinates.1 && f.coordinates.2 == particles[i].coordinates.2);
//            if (found.count()>1){
//                println!("Found!");
//            }
//
//        }
       // println!("Point: {}, {}, {}", &particles[0].coordinates.0, &particles[0].coordinates.1, &particles[0].coordinates.2);
            //  println!("Vel: {}", vel_text);
              //println!("Acc: {}", acc_text);

    }
        println!(" len: {}", &hashSet.len());
    //let mut s1 = Scatter::from_vec(&[(0.0f64, 0.0f64)]);

/*    let mut v: View = View::new();
    for particle in &particles {
        // Scatter plots expect a list of pairs

        let data_for_particle: &Vec<i32> =  data.get(&particle.p_index).unwrap();
        let  mut data_values: Vec<(f64, f64)> = Vec::new();
        for i in 0..1000{
            data_values.push((i as f64 , data_for_particle[i] as f64));
        }
        // We create our scatter plot from the data
        let s1 = Scatter::from_vec(data_values.as_slice())
            .style(scatter::Style::new().marker(scatter::Marker::Circle) // setting the marker to be a square
                .colour("#DD3355")); // and a custom colour



        // The 'view' describeswhat set of data is drawn
          // let mut cloned_v = v.clone();
         v = v.add(&s1).x_range(0.0f64, 1000f64).y_range(-1000f64, 1000f64);


    };

            // A plot with a single view is then saved to an SVG file
        Plot::single(&v).save("scatter.svg");
        */

    //for p in &particles{
    //    println!("Point: {}, {}, {}", p.coordinates.0, p.coordinates.1, p.coordinates.2);

    //}
    println!("************************************************************************************");
//    particles.sort_by_key(|co| abs(co.coordinates.0)  + abs(co.coordinates.1) + abs(co.coordinates.2));
//    for p in &particles{
//        println!("Point: {}, {}, {}", p.coordinates.0, p.coordinates.1, p.coordinates.2);
//    }
}


#[derive(Clone)]
struct Particle {
    coordinates: (i64, i64, i64),
    velocity: (i64, i64, i64),
    acceleration: (i64, i64, i64),
    p_index: i64
}

    fn update(p: Particle) -> Particle {
        return Particle{ coordinates: (p.coordinates.0 + p.velocity.0, p.coordinates.1 + p.velocity.1, p.coordinates.2 + p.velocity.2),
        velocity: (p.velocity.0 + p.acceleration.0, p.velocity.1 + p.acceleration.1, p.velocity.2 + p.acceleration.2),
        acceleration: (p.acceleration.0, p.acceleration.1, p.acceleration.2), p_index: p.p_index};
    }


impl Ord for Particle {
    fn cmp(&self, other: &Particle) -> Ordering {
        let self_sum = self.coordinates.0 + self.coordinates.1 + self.coordinates.2;
        let other_sum = other.coordinates.0 + other.coordinates.1 + other.coordinates.2;
        println!("Comparing.. {}, {}", self_sum, other_sum);
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
        return self.coordinates.0 == other.coordinates.0 &&
            self.coordinates.1 == other.coordinates.1
        && self.coordinates.2 == other.coordinates.2;

    }
}
impl Eq for Particle {
    //println!("In Eq");
//    fn eq(&self, other: &Particle) -> bool {
//        return self.coordinates.0 == other.coordinates.0 &&
//            self.coordinates.1 + other.coordinates.1
//        && self.coordinates.2 + other.coordinates.2;
//
//    }
}


impl Hash for Particle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.coordinates.0;
        //self.phone.hash(state);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    pub fn test_on_inp()
    {
        super::day_20();
    }
}
