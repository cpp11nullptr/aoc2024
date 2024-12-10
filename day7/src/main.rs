use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
struct Data {
    value: i64,
    operands: Vec<i64>,
}

fn main() {
    let file_path = r"data/day7.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    let lines = reader.lines();
    for line in lines {
        let line = line.expect("Cannot read line");
        let line_parts = line.split(':').collect::<Vec<&str>>();

        if line_parts.len() != 2 {
            panic!("Not valid item: {}", line);
        }

        let value = line_parts[0]
            .parse::<i64>()
            .unwrap_or_else(|e| panic!("Cannot parse value: {}", e));

        let operands = line_parts[1]
            .trim()
            .split(' ')
            .collect::<Vec<&str>>()
            .into_iter()
            .map(|s| {
                s.parse::<i64>()
                    .unwrap_or_else(|e| panic!("Cannot parse operand: {}", e))
            })
            .collect::<Vec<i64>>();

        data.push(Data { value, operands });
    }

    let mut result = 0;

    for item in data.iter() {
        let operands = &item.operands;
        let ops_list = generate_sequences(&[b'+', b'*', b'|'], operands.len() - 1);
        for ops in ops_list {
            let mut x = operands[0];
            for i in 1..operands.len() {
                match ops[i - 1] {
                    b'+' => x += operands[i],
                    b'*' => x *= operands[i],
                    b'|' => {
                        let mut tmp = operands[i];
                        while tmp > 0 {
                            x *= 10;
                            tmp /= 10;
                        }
                        x += operands[i];
                    }
                    _ => {}
                }
            }

            if x == item.value {
                result += item.value;
                break;
            }
        }
    }

    println!("Part 2: {}", result);
}

fn generate_sequences(v: &[u8], n: usize) -> Vec<Vec<u8>> {
    if n == 1 {
        return v.iter().map(|&c| vec![c]).collect();
    } else {
        let mut sequences = Vec::new();

        for &char in v {
            for seq in generate_sequences(v, n - 1) {
                let mut new_seq = vec![char];
                new_seq.extend(seq);
                sequences.push(new_seq);
            }
        }

        sequences
    }
}
