use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = r"data/day01.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut numbers1 = Vec::new();
    let mut numbers2 = Vec::new();

    let lines = reader.lines();
    for line in lines {
        let line = line.expect("Cannot read line");
        let line_parts = line.split_whitespace().collect::<Vec<&str>>();
        if line_parts.len() != 2 {
            panic!("Unexpected line: {}", line);
        }

        let number1 = line_parts[0]
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Cannot parse first number: {}", line));

        numbers1.push(number1);

        let number2 = line_parts[1]
            .parse::<i32>()
            .unwrap_or_else(|_| panic!("Cannot parse second number: {}", line));

        numbers2.push(number2);
    }

    numbers1.sort();
    numbers2.sort();

    // part 1

    let mut diff: i32 = 0;

    for i in 0..numbers1.len() {
        let x = numbers1[i];
        let y = numbers2[i];
        diff += (x - y).abs();
    }

    println!("Part 1: {}", diff);

    // part 2

    let mut sim: i32 = 0;

    for x in &numbers1 {
        let mut count = 0;
        for y in &numbers2 {
            if x == y {
                count += 1;
            }
        }

        sim += x * count;
    }

    println!("Part 2: {}", sim);
}
