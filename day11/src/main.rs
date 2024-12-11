use std::collections::HashMap;
use std::fs::File;
use std::io::{BufReader, Read};

fn main() {
    let file_path = "data/day11.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let mut reader = BufReader::new(file);

    let mut data = String::new();
    reader.read_to_string(&mut data).expect("Cannot read file");

    let stones_vec = data
        .split(' ')
        .map(|x| {
            x.parse::<u64>()
                .unwrap_or_else(|_| panic!("Cannot parse number: {}", x))
        })
        .collect::<Vec<u64>>();

    let mut stones = HashMap::new();

    for stone in stones_vec {
        let entry = stones.entry(stone).or_insert(0_u64);
        *entry += 1;
    }

    let count1 = stone_count(stones.clone(), 25);
    let count2 = stone_count(stones, 75);

    println!("Part 1: {}", count1);
    println!("Part 2: {}", count2);
}

fn stone_count(mut stones: HashMap<u64, u64>, n: usize) -> u64 {
    for _ in 0..n {
        let mut new_stones = HashMap::new();

        for stone in stones {
            if stone.0 == 0 {
                let entry = new_stones.entry(1).or_insert(0_u64);
                *entry += stone.1;
            } else {
                let mut stone_str = stone.0.to_string();
                if stone_str.len() > 1 && stone_str.len() % 2 == 0 {
                    let index = stone_str.len() / 2;
                    let stone_str0 = stone_str.split_off(index);

                    let value1 = stone_str0
                        .parse::<u64>()
                        .unwrap_or_else(|_| panic!("Cannot parse first number: {}", stone_str0));

                    let value2 = stone_str
                        .trim_start_matches('0')
                        .parse::<u64>()
                        .unwrap_or_else(|_| panic!("Cannot parse second number: {}", stone_str));

                    let entry1 = new_stones.entry(value1).or_insert(0);
                    *entry1 += stone.1;

                    let entry2 = new_stones.entry(value2).or_insert(0);
                    *entry2 += stone.1;
                } else {
                    let value = stone.0 * 2024;
                    let entry = new_stones.entry(value).or_insert(0);
                    *entry += stone.1;
                }
            }
        }

        stones = new_stones;
    }

    stones.values().sum::<u64>()
}
