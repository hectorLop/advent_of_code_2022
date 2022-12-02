use std::fs::File;
use std::io::{self, BufRead};

fn main() {
   // part_1();
    part_2();
}

fn part_1() {
    let mut player1 = 0;
    let mut player2 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(value) = line {
                let normalized_value = normalize_values(value);
                let results: Vec<&str> = normalized_value.split_whitespace().collect();
                
                let score = decide_winner((results[0], results[1]));
                println!("{:?} and {:?}", results, score);
                //println!("{:?} and {:?}", results, score);
                player1 += get_action_value(results[0]) + score.0;
                player2 += get_action_value(results[1]) + score.1;
            }        
        }
    }

    println!("{}", player1);
    println!("{}", player2);
    println!("Total {}", player1+player2);
}

fn part_2() {
    let mut player1 = 0;
    let mut player2 = 0;

    if let Ok(lines) = read_lines("input.txt") {
        for line in lines {
            if let Ok(value) = line {
                let results: Vec<&str> = value.split_whitespace().collect();
                
                let score = decide_winner_2((results[0], results[1]));
                
                println!("{:?} and {:?}", results, score);
                player1 += score.0;
                player2 += score.1;
            }        
        }
    }

    println!("{}", player1);
    println!("{}", player2);
    println!("Total {}", player1+player2);
}

fn normalize_values(mut actions: String) -> String {
    actions = actions.replace("X", "A");
    actions = actions.replace("Y", "B");
    actions = actions.replace("Z", "C");
    actions
}

fn get_action_value(action: &str) -> i32 {
    match action {
        "A" => 1,
        "B" => 2,
        "C" => 3,
        _other => panic!("Panicked"),
    }
}

fn decide_winner_2(game: (&str, &str)) -> (i32, i32) {
    match game {
        ("A", "X") => (7, 3),
        ("A", "Y") => (4, 4),
        ("A", "Z") => (1, 8),
        ("B", "X") => (8, 1),
        ("B", "Y") => (5, 5),
        ("B", "Z") => (2, 9),
        ("C", "X") => (9, 2),
        ("C", "Y") => (6, 6),
        ("C", "Z") => (3, 7),
        _other => panic!("Panic")
    }
}

fn decide_winner(game: (&str, &str)) -> (i32, i32) {
    match game {
        ("A", "B") => (0, 6),
        ("B", "A") => (6, 0),
        ("A", "C") => (6, 0),
        ("C", "A") => (0, 6),
        ("B", "C") => (0, 6),
        ("C", "B") => (6, 0),
        _other => (3, 3),
    }
}


fn read_lines(filename: &str) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
