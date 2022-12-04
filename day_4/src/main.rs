use std::fs;
use itertools::Itertools;


fn main() {
    part_1();
    part_2();
}

fn part_2() {
    let input = read_lines("src/input.txt");
    let lines: usize = input
                    .lines()
                    .map(|line| get_overlapping(line))
                    .filter(|result| *result == true)
                    .count(); 
    println!("{:?}", lines);
}

fn part_1() {
    let input = read_lines("src/input.txt");
    let lines: usize = input
                    .lines()
                    .map(|line| contains_range(line))
                    .filter(|result| *result == true)
                    .count();
    println!("{:?}", lines);
}

fn get_overlapping(line: &str) -> bool {
    let data: Vec<&str> = line.split(",").collect();

    let range_1: Vec<i32> = data[0]
                                .split("-")
                                .map(|x| x.parse::<i32>().unwrap())
                                .collect();
    let range_2: Vec<i32> = data[1]
                                .split("-")
                                .map(|x| x.parse::<i32>().unwrap())
                                .collect();
    
    // This is the only possible NON overlap conditions, so if neither
    // of them are true, that means the ranges overlap
    if range_1[1] < range_2[0] || range_2[1] < range_1[0] {
        return false;
    } else {
        return true; // Overlapped range
    }
}

fn contains_range(line: &str) -> bool {
    let data: Vec<&str> = line.split(",").collect();

    let range_1: Vec<i32> = data[0]
                                .split("-")
                                .map(|x| x.parse::<i32>().unwrap())
                                .collect();
    let range_2: Vec<i32> = data[1]
                                .split("-")
                                .map(|x| x.parse::<i32>().unwrap())
                                .collect();

    
    is_range_contained(range_1, range_2)
}

fn is_range_contained(range_1: Vec<i32>, range_2: Vec<i32>) -> bool {
    if range_1[0] <= range_2[0] && range_1[1] >= range_2[1] {
        return true;
    } else if range_2[0] <= range_1[0] && range_2[1] >= range_1[1] {
        return true;
    }

    false
}


fn read_lines(filename: &str) -> String {
    let f = fs::read_to_string(filename);
    f.expect("could not open input file")
}
