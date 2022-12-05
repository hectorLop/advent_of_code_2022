use std::fs;
use std::collections::VecDeque;
use itertools::{Itertools, Tuples};

#[derive(Debug)]
struct Movement {
    quantity: i32,
    from_stack: usize,
    to_stack: usize,
}

fn main() {
    part_1();
    part_2();
}

fn part_2() {
   let input = read_lines("src/input.txt");
   let data = split_stack_and_moves(input);

   let final_stack = do_movements_2(data.0, data.1);
   println!("{:?}", final_stack);
}

fn part_1() {
   let input = read_lines("src/input.txt");
   let data = split_stack_and_moves(input);

   let final_stack = do_movements(data.0, data.1);
   println!("{:?}", final_stack);
}

fn do_movements_2(mut stacks: Vec<VecDeque<char>>, movements: Vec<Movement>) -> Vec<VecDeque<char>> {
    for movement in movements.into_iter() {

        let mut tmp_stack: VecDeque<char> = VecDeque::new();

        for i in 0..movement.quantity {

            let element: char = stacks[movement.from_stack - 1].pop_back().unwrap();
            tmp_stack.push_front(element);
        }

        for character in tmp_stack.into_iter() {
            stacks[movement.to_stack - 1].push_back(character)
        }
    }

    stacks
}

fn do_movements(mut stacks: Vec<VecDeque<char>>, movements: Vec<Movement>) -> Vec<VecDeque<char>> {
    for movement in movements.into_iter() {
        for i in 0..movement.quantity {
            let element: char = stacks[movement.from_stack - 1].pop_back().unwrap();
            stacks[movement.to_stack - 1].push_back(element);
        }
    }

    stacks
}

fn split_stack_and_moves(input: String) -> (Vec<VecDeque<char>>, Vec<Movement>) {
    let mut data = input.split("\n\n");
    let stack_data = data.next().unwrap().to_string();
    let moves_data = data.next().unwrap().to_string();

    let stack = get_stack(&stack_data);
    let movements = get_movements(&moves_data);

    return (stack, movements);
}

fn get_movements(input: &String) -> Vec<Movement> {
    let movements: Vec<Movement> = input
                        .lines()
                        .map(|line| line.split_whitespace().collect::<Vec<&str>>())
                        .map(|strings: Vec<&str>| {
                            Movement {
                                quantity: strings[1].parse::<i32>().unwrap(),
                                from_stack: strings[3].parse::<usize>().unwrap(),
                                to_stack: strings[5].parse::<usize>().unwrap(),
                            }
                        })
                        .collect();
 
    movements
}

fn get_stack(input: &String) -> Vec<VecDeque<char>>{
   let columns: Vec<Vec<char>> = input
                                .lines()
                                .map(|line| line.chars().collect::<Vec<char>>())
                                .map(|chars: Vec<char>| {
                                    let mut bucket: Vec<char> = Vec::new();
                                    
                                    for i in (1..chars.len()).step_by(4) {
                                        bucket.push(chars[i]);
                                    }

                                    bucket
                                })
                                .collect();

    let mut stacks: Vec<VecDeque<char>> = Vec::new();

    for (i, row) in columns.into_iter().enumerate() {
        for (j, character) in row.into_iter().enumerate() {
            if stacks.len() < j+1 {
                stacks.push(VecDeque::new());
            }
            
            if !character.is_numeric() && character != ' ' {
                stacks[j].push_front(character)
            }
        }
    }

    stacks
}

fn read_lines(filename: &str) -> String {
    let f = fs::read_to_string(filename);
    f.expect("could not open input file")
}
