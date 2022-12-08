use std::fs;

#[derive(Debug)]
struct Point {
    x: usize,
    y: usize,
    value: u32,
}

impl Point {
    fn new(x: usize, y: usize, value: u32) -> Point {
        Point {x: x, y: y, value: value}
    }

    fn geq(&self, point: &Point) -> bool {
        self.value >= point.value
    }
}

fn main() {
    //part_1();
    part_2();
}

fn part_2() {
    let input: Vec<Vec<u32>> = 
        read_lines("src/input.txt")
                                  .lines()
                                  .map(|line| line
                                                  .chars()
                                                  .map(|c| c.to_digit(10).unwrap())
                                                  .collect::<Vec<u32>>())
                                  .collect::<Vec<Vec<u32>>>();
    
    let columns: usize = input[0].len();
    let rows: usize = input.len();
    let mut max_score = 0;

    for x in 1..rows - 1 {
        for y in 1..columns - 1 {
            let point = Point::new(x, y, input[x][y]);
            
            let mut results: Vec<usize> = Vec::new();
            results.push(left_dist(&point, &input));
            results.push(right_dist(&point, &input));
            results.push(top_dist(&point, &input));
            results.push(down_dist(&point, &input));
            
            //println!("{:?}", point);
            //println!("{:?}", results);
            let scenic_score = results.iter().product();
            
            if scenic_score > max_score {
                max_score = scenic_score;
            }
            
        }
    }
    println!("{}", max_score);
}

fn part_1() {
    let input: Vec<Vec<u32>> = 
        read_lines("src/input.txt")
                                  .lines()
                                  .map(|line| line
                                                  .chars()
                                                  .map(|c| c.to_digit(10).unwrap())
                                                  .collect::<Vec<u32>>())
                                  .collect::<Vec<Vec<u32>>>();
    
    let columns: usize = input[0].len();
    let rows: usize = input.len();
    let mut visible_trees = 0;

    for x in 1..rows - 1 {
        for y in 1..columns - 1 {
            let point = Point::new(x, y, input[x][y]);

            if visible_left(&point, &input) 
                || visible_right(&point, &input)
                || visible_top(&point, &input)
                || visible_down(&point, &input) {
                visible_trees += 1;
            }
        }
    }
    visible_trees += (rows * 4) - 4;
    println!("{}", visible_trees);

    // Sum all the edges 99 * 4
}

fn visible_left(point: &Point, data: &Vec<Vec<u32>>) -> bool {
    for j in 0..point.y {
        let point_to_check = Point::new(point.x, j, data[point.x][j]);

        if point_to_check.geq(point) {
            return false;
        }
    }

    true
}

fn left_dist(point: &Point, data: &Vec<Vec<u32>>) -> usize {
    for j in (0..point.y).rev() {
        let point_to_check = Point::new(point.x, j, data[point.x][j]);

        if point_to_check.geq(&point) {
            return point.y - point_to_check.y 
        }
    }

    point.y - 0 
}

fn visible_right(point: &Point, data: &Vec<Vec<u32>>) -> bool {
    for j in point.y + 1..data[point.x].len() {
        let point_to_check = Point::new(point.x, j, data[point.x][j]);

        if point_to_check.geq(point) {
            return false;
        }
    }

    true
}

fn right_dist(point: &Point, data: &Vec<Vec<u32>>) -> usize {
    for j in point.y + 1..data[point.x].len() {
        let point_to_check = Point::new(point.x, j, data[point.x][j]);

        if point_to_check.geq(point) {
            return point_to_check.y - point.y 
        }
    }

    data[point.x].len() - 1  - point.y
}

fn visible_top(point: &Point, data: &Vec<Vec<u32>>) -> bool {
    for i in 0..point.x {
        let point_to_check = Point::new(i, point.y, data[i][point.y]);

        if point_to_check.geq(point) {
            return false;
        }
    }

    true
}

fn top_dist(point: &Point, data: &Vec<Vec<u32>>) -> usize {
    for i in (0..point.x).rev() {
        let point_to_check = Point::new(i, point.y, data[i][point.y]);

        if point_to_check.geq(point) {
            println!("{:?} and {:?}", point, point_to_check.x);
            return point.x - point_to_check.x;
        }
    }

    point.x - 0
}


fn visible_down(point: &Point, data: &Vec<Vec<u32>>) -> bool {
    for i in point.x + 1..data.len() {
        let point_to_check = Point::new(i, point.y, data[i][point.y]);

        if point_to_check.geq(point) {
            return false;
        }
    }

    true
}

fn down_dist(point: &Point, data: &Vec<Vec<u32>>) -> usize {
    for i in point.x + 1..data.len() {
        let point_to_check = Point::new(i, point.y, data[i][point.y]);

        if point_to_check.geq(point) {
            return point_to_check.x - point.x;
        }
    }

    data.len() - 1 - point.x
}

fn read_lines(filename: &str) -> String {
    let f = fs::read_to_string(filename);
    f.expect("could not open input file")
}
