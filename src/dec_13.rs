
use std::collections::HashMap;

use super::parse_utils;

fn day_13_solution(){
    // let stdin = io::stdin();
    let inp = parse_utils::read_line_by_line("inp13.txt");

    let mut heights = HashMap::<u32, u32>::new();

    for line in inp {
        //let line = line.unwrap();
        let split: Vec<_> = line.split_whitespace().collect();

        heights.insert(split[0].parse().unwrap(), split[1].parse().unwrap());
    }

    let severity: u32 = heights.iter()
        .filter(|&(&pos, &height)| pos % (2 * (height - 1)) == 0)
        .map(|(pos, height)| pos * height)
        .sum();

    println!("[Part 1] Severity is: {}", severity);

    let wait: u32 = (0..)
        .filter(|wait| !heights.iter()
            .any(|(&pos, &height)| (wait + pos) % (2 * (height - 1)) == 0))
        .next()
        .unwrap();

    println!("[Part 2] Wait time is: {}", wait);
}

fn day_13(delay: usize){
    let inp = parse_utils::read_line_by_line("inp13.txt");
    let mut hm: HashMap<usize, Snake> = HashMap::new();
    let mut iter:usize = 0;
    for line in inp{
        let mut sw = line.split_whitespace();
        let  first: usize = sw.next().unwrap().parse().unwrap();
        let le: usize =sw.next().unwrap().parse().unwrap();
        if first > iter{
            for i in iter..first {
                hm.insert(i, Snake{s: 0, l: 0, inc: false});
            }
        }
        hm.insert(first, Snake{s: 0, l: le, inc: false});
        iter = first + 1;
    }

    let mut s = 0;
    for s_x in 0..96{

        let sn: &mut Snake = hm.get_mut(&s_x).unwrap();
        if sn.l == 0 {
            continue;
        }
        let step = delay % (sn.l + sn.l -2);
        for st in 0..step{
            sn.up_s();

        }
    }
    let mut it_del = 0;
    for iter in 0..96 {
        {
            let x = hm.get(&iter).unwrap();

            if 0 == x.s  && x.l >0 {
                //println!("Caught in: {} with severity {}", iter, x.l);
                s = s + (iter * x.l);//break;
                it_del = it_del +1;
                break;

            }
        }
        // One iteration:
        for s_x in iter..96{
            let sn: &mut Snake = hm.get_mut(&s_x).unwrap();
            sn.up_s();
        }


    }
    if(it_del == 0){
        println!("iiiii is oooookkkk: {}", delay);
    }
    // println!("Severity: {}", s);
}


struct Snake {
    s: usize,
    l: usize,
    inc: bool
}

impl Snake{
    fn up_s(& mut self) {
        if self.l == 0 {
            return;
        }
        if self.s == 0 || self.s == self.l - 1{
            self.inc = !self.inc;
        }

        match self.inc {
            true => self.s+= 1,
            false => self.s-=1
        }
    }

}