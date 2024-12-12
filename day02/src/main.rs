use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = r"data/day02.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut safe_count1 = 0;
    let mut safe_count2 = 0;

    let lines = reader.lines();
    for line in lines {
        let line = line.expect("Cannot read line");
        let line_parts = line.split_whitespace().collect::<Vec<&str>>();

        let line_numbers = line_parts
            .iter()
            .map(|v| v.parse::<i32>().unwrap())
            .collect::<Vec<i32>>();

        // part 1

        if is_level_safe(&line_numbers) {
            safe_count1 += 1;
            safe_count2 += 1;
            continue;
        }

        // part 2

        for i in 0..line_numbers.len() {
            let mut numbers = line_numbers.clone();
            numbers.remove(i);

            if is_level_safe(&numbers) {
                safe_count2 += 1;
                break;
            }
        }
    }

    println!("Part 1: {}", safe_count1);
    println!("Part 2: {}", safe_count2);
}

fn is_level_safe(numbers: &[i32]) -> bool {
    if numbers.len() < 2 {
        return true;
    }

    let mut x = numbers[0];
    let mut y = numbers[1];

    let mut diff = y - x;
    if diff.abs() < 1 || diff.abs() > 3 {
        return false;
    }

    let is_increasing = diff > 0;

    for n in numbers.iter().skip(2) {
        x = y;
        y = *n;

        diff = y - x;
        if diff.abs() < 1 || diff.abs() > 3 {
            return false;
        }

        if is_increasing != ((y - x) > 0) {
            return false;
        }
    }

    return true;
}
