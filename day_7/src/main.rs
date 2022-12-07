use std::fs;
use std::collections::HashMap;

fn main() {
    let counter = part_1();
    part_2(counter);
}

fn part_2(counter: HashMap<String, i32>) -> () {
    let dir_space = counter.get("/-").unwrap();
    let unused_space = 70000000 - dir_space;
    
    let mut min_folder_size = 70000000;
    for value in counter.into_values() {
        if unused_space + value >= 30000000 {
            if value < min_folder_size {
                min_folder_size = value;
            }
        }

    }
    println!("Part 2 final: {}", min_folder_size);
}

fn part_1() -> HashMap<String, i32> {
    let input = read_lines("src/input.txt");
    
    let mut path = "".to_string();
    let mut counter: HashMap<String, i32> = HashMap::new();
    for line in input.lines() {
       let result = decide_action(line.to_string(), path, counter);
       path = result.0;
       counter = result.1;
    }
    
    let mut count = 0;
    for value in counter.clone().into_values() {
        if value <= 100000 {
            count += value;
        }
    }
    println!("Final {}", count);

    counter
}

fn decide_action(line: String, path: String, counter: HashMap<String, i32>) -> (String, HashMap<String, i32>) {
    let line_data: Vec<&str> = line.split_whitespace().collect();
    let value = line_data[0];

    match value {
        "$" => {
            let result = handle_command(line_data, path, counter);
            (result.0, result.1)
        },
        _ => { 
            let result = handle_info(value, path, counter);
            (result.0, result.1)
        }
    }
}

fn handle_command(data: Vec<&str>, path: String, mut counter: HashMap<String, i32>) -> (String, HashMap<String, i32>){
    match data[1] {
        "cd" => {
            match data[2] {
                ".." => {
                    let mut tmp_path: Vec<String> = path.split("-").map(|s| s.to_string()).collect();
                    tmp_path.pop();
                    tmp_path.pop();

                    return (tmp_path.join("-") + "-", counter);
                },
                string => {
                    let tmp_path = path.to_owned() + string + "-";
                    counter.entry(tmp_path.to_string()).or_insert(0);
                    return (tmp_path, counter);
                }
            }
        },
        _ => ()
    };

    (path, counter)
}

fn handle_info(data: &str, path: String, mut counter: HashMap<String, i32>) -> (String, HashMap<String, i32>){
    match data.parse::<i32>() {
        Ok(val) => {
            //let mut tmp_path: Vec<String> = path.split("-").map(|s| s.to_string()).collect();
            let mut p = "".to_string();
            for x in path.split("-") {
                if !x.is_empty() {
                    p = p + x + "-";
                    println!("{}", p);

                    *counter.entry(p.clone()).or_insert(0) += val;
                }
            }

        },
        Err(_) => {}
    }
    (path, counter)
}

fn read_lines(filename: &str) -> String {
    let f = fs::read_to_string(filename);
    f.expect("could not open input file")
}
