pub fn day_12() {
    let inp = read_line_by_line("inp12.txt");
    let tuples = inp.iter().map(|line| parse_line(line));
    let tree: HashMap<String, Vec<String>> = tuples.into_iter().collect();
    println!("tree size {}", tree.len());
    print_vector_str(tree.values().next().unwrap());
    let mut node:String = "0".to_string();
    let mut parsed = HashSet::new();
    //parsed.insert(node.to_string());
    let mut set = Vec::new();
    set.push(node);
    let mut count = 0;
    while let Some(child) = set.pop(){
        // println!("Child: {}", child);
        let x = child.as_str();
        if !parsed.contains(&String::from(x)){
            //println!("Counted child: {}", child);
            count = count +1;
            parsed.insert(child.to_string());
            let x: &Vec<String> = &tree[child.as_str()];
            for s in x {
                set.push(String::from(&*s.as_str()));

            }
            // in_program_node(&child, tree, already_parsed);

        }
    }

    let mut count_of_groups = 1;
    //while let Some(new_input) = tree.keys().filter(|k| !parsed.contains(&k.to_string())).next()
    while let Some(new_input) = tree.keys().filter(|k| !parsed.contains(&k.to_string())).next(){

        count_of_groups = count_of_groups + 1;
        let mut set2 = Vec::new();
        set2.push(String::from(&*new_input.as_str()));
        let mut count = 0;
        while let Some(child) = set2.pop() {
            // println!("Child: {}", child);
            let x = child.as_str();
            if !parsed.contains(&String::from(x)) {
                //println!("Counted child: {}", child);
                count = count + 1;
                parsed.insert(child.to_string());
                let x: &Vec<String> = &tree[child.as_str()];
                for s in x {
                    set2.push(String::from(&*s.as_str()));
                }
                // in_program_node(&child, tree, already_parsed);
            }

        }
        println!("Count {} ", count);
    }


    println!("Count of groups {} ", count_of_groups);
    // in_program_node(&"0", tree, &mut HashSet::new());
}