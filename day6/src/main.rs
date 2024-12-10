use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, Mutex};
use std::thread;

#[derive(Clone, Eq, Hash, PartialEq)]
enum Direction {
    North,
    East,
    South,
    West,
}

fn main() {
    let file_path = r"data/day6.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut data = Vec::new();
    let mut x = usize::MAX;
    let mut y = usize::MAX;

    let lines = reader.lines();
    for (i, line) in lines.enumerate() {
        let line = line.expect("Cannot read line");
        let line_bytes = line.into_bytes();

        let guard_pos = line_bytes.iter().position(|n| *n == b'^');
        if let Some(j) = guard_pos {
            x = i;
            y = j;
        }

        data.push(line_bytes);
    }

    if x == usize::MAX || y == usize::MAX {
        panic!("Guard isn't found");
    }

    let data = Arc::new(data);
    let result = Arc::new(Mutex::new(0));

    let mut threads = Vec::new();

    for i in 0..data.len() {
        let data_copy = Arc::clone(&data);
        let result_copy = Arc::clone(&result);
        let handle = thread::spawn(move || {
            for j in 0..data_copy[0].len() {
                let is_loop = map_loop(x, y, i, j, &data_copy);
                if is_loop {
                    let mut result = result_copy.lock().expect("Cannot acquire result");
                    *result += 1;
                }
            }
        });

        threads.push(handle);
    }

    for handle in threads {
        handle.join().expect("Spawned thread panics");
    }

    let result = result.lock().expect("Cannot acquire result");
    
    println!("Part 2: {}", *result);
}

fn map_loop(mut x: usize, mut y: usize, bx: usize, by: usize, data: &[Vec<u8>]) -> bool {
    let vmax = data.len();
    let hmax = data[0].len();

    let mut direction = Direction::North;
    let mut visited = HashSet::new();

    loop {
        if visited.contains(&(x, y, direction.clone())) {
            return true;
        }

        visited.insert((x, y, direction.clone()));

        match direction {
            Direction::North => {
                if x == 0 {
                    break;
                }

                if (x - 1 == bx && y == by) || (data[x - 1][y] == b'#') {
                    direction = Direction::East;
                } else {
                    x -= 1;
                }
            }
            Direction::East => {
                if y + 1 == hmax {
                    break;
                }

                if (x == bx && y + 1 == by) || (data[x][y + 1] == b'#') {
                    direction = Direction::South;
                } else {
                    y += 1;
                }
            }
            Direction::South => {
                if x + 1 == vmax {
                    break;
                }

                if (x + 1 == bx && y == by) || (data[x + 1][y] == b'#') {
                    direction = Direction::West;
                } else {
                    x += 1;
                }
            }
            Direction::West => {
                if y == 0 {
                    break;
                }

                if (x == bx && y - 1 == by) || (data[x][y - 1] == b'#') {
                    direction = Direction::North;
                } else {
                    y -= 1;
                }
            }
        }
    }

    false
}
