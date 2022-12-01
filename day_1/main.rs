use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    let calories = part_1();
    part_2(calories);
}

fn part_2(mut calories: Vec<i32>) {
    calories.sort();
    calories.reverse();

    let total = calories[0] + calories[1] + calories[2];
    println!("Top three calories total: {}", total);
}

fn part_1() -> Vec<i32> {
    let mut calories: Vec<i32> = Vec::new();
    let mut max: i32 = 0;
    let mut count: i32 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(value) = line {
                if value.is_empty() {
                    calories.push(count);

                    if count > max {
                        max = count;
                    }

                    count = 0;
                } else {
                    count += value.parse::<i32>().unwrap();
                }
            }
        }

        println!("The max is {}", max); 
    }

    calories
}

fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
