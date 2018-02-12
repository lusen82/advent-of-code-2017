
extern crate regex;
extern crate ascii;
use std::string::String;
use std::i32;
use std::convert::From;
use std::collections::HashSet;
use self::regex::Regex;
use std::collections::HashMap;
use std::fmt;
use super::parse_utils;


pub fn day_7() {

    // 7 a.
    let mut input_for_a: Vec<String> = parse_utils::read_line_by_line("inp7.txt");

    // Keep only parent nodes:
    input_for_a.retain(|r| r.find('>').is_some());

    let parent_children: Vec<(&str, Vec<&str>)> = input_for_a.iter().map(|par| create_parent_children_tp(par)).collect();
    let child_map: HashMap<&str, Vec<&str>> = parent_children.into_iter().collect();
    let mut children = HashSet::new();
    for child in child_map.values() {
        for each in child {
            children.insert(each.trim());
        }
    }


    let option = child_map.keys().find(|&p| !children.contains(p));

    println!("Result day 7: ");
    match option {
        None => println!("Failed!"),
        Some(root) => println!("{}", root),
    }


    // 7 b.
    let input_for_b: Vec<String> = parse_utils::read_line_by_line("inp7.txt");


    let list_of_prog: Vec<Prog> = input_for_b.iter().map(|par| create_tp(par, &child_map)).collect();

    let tuple_map: Vec<(String, Prog)> = list_of_prog.into_iter().map(|l| (String::from(l.name.as_str()), l)).collect();

    let mut value_map: HashMap<String, Prog> = tuple_map.into_iter().collect();


    // Check:
    println!("Calculate total values: ");
    loop {
        let  mut nothing_updated = true;
        for s_par in child_map.keys() {

             let mut this_total_weight = 0;
             let mut can_count = true;
             let children: &Vec<&str> = child_map.get(s_par).unwrap();
             for child in children{
                 let child_total_weight = &value_map.get(child.trim()).unwrap().total_weight;
                 if *child_total_weight == 0 {
                     can_count = false;
                     break;
                 }
                 this_total_weight += child_total_weight;
             }
             let pg: &mut Prog = value_map.get_mut(*s_par).unwrap();
             if pg.total_weight != 0 {
                 continue;  // weight is set
             }

             if can_count {
                 nothing_updated = false;
                 pg.up(this_total_weight);

             }
         }
        if nothing_updated {
            break;
        }
    }

    println!("Cacluation done. ");

    // Start from root:

    let root: &str = option.unwrap();

    println!("Problem root: ");
    recurse_value_check(&child_map, &value_map, root);


}

fn recurse_value_check(child_map: &HashMap<&str, Vec<&str>> , value_map: &HashMap<String, Prog>, root: &str){
    let x = child_map.get(root);

    if x.is_none() { return; }

    let children: &Vec<&str> = x.unwrap();
//    println!("Problem x: ");
//    println!("Problem vec: -{}-", value_map.keys().next().unwrap());
//    println!("Problem x: -{}-", children[0].trim());
    let first_val = value_map.get(children[0].trim()).unwrap().total_weight;
   // println!("Problem fv: ");
    for std in children {
        let ch_name = std.trim();
       // println!("Problem fv: -{}-", ch_name);
        let option: &Option<&Prog> = &value_map.get(ch_name);
     //   println!("Problem option: ");
        if option.unwrap().total_weight != first_val {
            println!("Node is not ok: {}", root);
            println!("Child diff is: {}", ch_name);
            println!("Value of child is: {}", option.unwrap().total_weight);
            println!("Value of other child is: {}", first_val);
        }
        recurse_value_check(&child_map, &value_map, ch_name);
    }
}

fn create_tp(input_row: &str, child_map: &HashMap<&str, Vec<&str>>) -> Prog
{
    let parent = input_row.split(" ").next().unwrap();

    let re: Regex = regex::Regex::new("\\d+").unwrap();
    let mat = re.find(input_row).unwrap().as_str();
    let single_weight = mat.parse::<i32>().unwrap();

    let  total_weight = match child_map.get(parent).is_none() || child_map.get(parent).unwrap().is_empty() {
        true => single_weight,
        false => 0
    };

    return Prog{name: String::from(parent), single_weight, total_weight };

}


fn create_parent_children_tp(inn: &str) -> (&str, Vec<&str>)
{
    let vector: Vec<&str> = inn.split(">").collect();
    let (parent_info, children) = (vector[0], vector[1]);

    let parent = parent_info.split_whitespace().next().unwrap();

    let children_vec: Vec<&str> = children.trim().split(",").collect();

    return  (parent, children_vec);
}

struct Prog {
    name: String,
    single_weight: i32,
    total_weight: i32

}

impl Prog {
    fn up(&mut self, upd: i32) {
        self.total_weight = upd + self.single_weight;
    }
}

impl fmt::Display for Prog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Name: {}, Single value: {}, Total weight: {}", self.name, self.single_weight, self.total_weight)
    }
}


mod tests {
    #[test]
    pub fn test_on_inp()
    {
        super::day_7();
    }
}
