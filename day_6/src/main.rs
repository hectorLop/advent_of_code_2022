use std::fs;
use itertools::Itertools;

fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let input = read_lines("src/input.txt").replace("\n", "");
    if let Some(marker) = get_marker(input, 14) {
        println!("The marker is {}", marker);
    } else {
        println!("No marker");
    }

}

fn part_1() {
    let input = read_lines("src/input.txt").replace("\n", "");
    if let Some(marker) = get_marker(input, 4) {
        println!("The marker is {}", marker);
    } else {
        println!("No marker");
    }

}

fn get_marker(input: String, number_distinct_chars: usize) -> Option<usize> {
    let mut slow: usize = 0;
    let mut fast: usize = number_distinct_chars;
    let characters: Vec<char> = input.chars().collect();
    
    while fast < characters.len() - 1 {
        let slice: Vec<&char> = characters[slow..fast].into_iter().unique().collect(); 

        if slice.len() == number_distinct_chars {
            return Some(fast)
        } else {
            slow += 1;
            fast += 1;
        }
    }

    None
}

fn read_lines(filename: &str) -> String {
    let f = fs::read_to_string(filename);
    f.expect("could not open input file")
}
