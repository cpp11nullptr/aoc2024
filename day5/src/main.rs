use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = r"data/day5.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut orders = Vec::new();
    let mut updates = Vec::new();

    let mut is_order = true;
    let lines = reader.lines();
    for line in lines {
        let line = line.expect("Cannot read line");
        if is_order {
            if line.is_empty() {
                is_order = false;
                continue;
            }

            let line_parts = line.split('|').collect::<Vec<&str>>();

            let page1 = line_parts[0]
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Cannot parse first page: {}", line));

            let page2 = line_parts[1]
                .parse::<i32>()
                .unwrap_or_else(|_| panic!("Cannot parse second page: {}", line));

            let order = (page1, page2);

            orders.push(order);
        } else {
            let line_parts = line.split(',').collect::<Vec<&str>>();

            let pages = line_parts
                .iter()
                .map(|v| {
                    v.parse::<i32>()
                        .unwrap_or_else(|_| panic!("Cannot parse second page: {}", v))
                })
                .collect::<Vec<i32>>();

            updates.push(pages);
        }
    }

    let mut not_correct = HashSet::new();

    let mut i = 0;
    while i < updates.len() {
        let update = &mut updates[i];
        let mut is_correct = true;
        for i in 0..update.len() {
            let page = update[i];
            for j in i + 1..update.len() {
                let next_page = update[j];
                for order in &orders {
                    if order.0 == next_page && order.1 == page {
                        is_correct = false;
                        update.swap(i, j);
                        break;
                    }
                }

                if !is_correct {
                    break;
                }
            }

            if !is_correct {
                break;
            }
        }

        if is_correct {
            i += 1;
        } else {
            not_correct.insert(i);
        }
    }

    let mut result = 0;

    for i in not_correct {
        let update = &updates[i];
        result += update[update.len() / 2];
    }

    println!("Part 2: {}", result);
}
