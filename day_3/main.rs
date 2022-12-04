use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};
use itertools::Itertools;

fn main() {
    part_2();
}

fn part_2() {
    let alphabet: Vec<char> = [('a' ..= 'z').collect::<Vec<_>>(), ('A' ..= 'Z').collect::<Vec<_>>()].concat();
    let mut sum: usize = 0;
    let reader = read_lines("input.txt");
    let lines = reader.lines().collect::<Result<Vec<String>, _>>().unwrap();
    for (a, b, c) in lines.iter().tuples() {
        let backpack_1: Vec<char> = a.chars().unique().collect::<Vec<char>>(); 
        let backpack_2: Vec<char> = b.chars().unique().collect::<Vec<char>>(); 
        let backpack_3: Vec<char> = c.chars().unique().collect::<Vec<char>>(); 

        let letter_counts = items_to_hash_map(&backpack_1);

        if let Ok(common_item) = get_common_item_2(letter_counts, &backpack_2, &backpack_3) {
            println!("{}", common_item);
            for (i, letter) in alphabet.iter().enumerate() {
                if common_item == *letter {
                    sum += i + 1;
                }
            }
        }
    }

    println!("The total is: {}", sum);
}

fn part_1() {
    let alphabet: Vec<char> = [('a' ..= 'z').collect::<Vec<_>>(), ('A' ..= 'Z').collect::<Vec<_>>()].concat();
    let mut sum: usize = 0;
    if let lines = read_lines("input.txt").lines() {
        for line in lines {
            if let Ok(value) = line {
                
                let length = value.len();
                let chars: Vec<char> = value.chars().collect();
                let backpack_1 = &chars[0..length/2];
                let backpack_2 = &chars[length/2..];
                let letter_counts = items_to_hash_map(backpack_1);

                if let Ok(common_item) = get_common_item(letter_counts, backpack_2) {
                    for (i, letter) in alphabet.iter().enumerate() {
                        if common_item == *letter {
                            sum += i + 1;
                        }
                    }
                }

            }
        }
    }
    println!("The total is: {}", sum);
}

fn items_to_hash_map(items: &[char]) -> HashMap<char, i32> {
    let mut letter_counts: HashMap<char, i32> = HashMap::new();
    for c in items {
        *letter_counts.entry(*c).or_insert(0) += 1;
    }

    letter_counts
}

fn get_common_item_2(mut letter_counts: HashMap<char, i32>, items2: &[char], items3: &[char]) -> Result<char, String> { 
    for c in [items2, items3].concat() {
        *letter_counts.entry(c).or_insert(0) += 1;

        if *letter_counts.get(&c).unwrap() == 3 {
            return Ok(c);
        }
    
    }
    Err("No common items".to_string())
}

fn get_common_item(item_count: HashMap<char, i32>, items: &[char]) -> Result<char, &str> {
    for c in items {
        if item_count.contains_key(c) {
            return Ok(*c);
        }
    }

    Err("No common items")
}

fn read_lines(filename: &str) -> io::BufReader<File> {
    let file = File::open(filename).unwrap();
    io::BufReader::new(file)
}
