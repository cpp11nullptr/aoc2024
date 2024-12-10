use std::fmt::{Debug, Formatter};
use std::fmt::{Result as FormatResult, Write};
use std::fs::File;
use std::io::{BufReader, Read};

#[derive(Clone, Copy)]
enum Block {
    File(i32),
    Free,
}

impl Debug for Block {
    fn fmt(&self, f: &mut Formatter<'_>) -> FormatResult {
        match &self {
            Block::File(index) => f.write_fmt(format_args!("{}", index)),
            Block::Free => f.write_char('.'),
        }
    }
}

#[derive(Debug)]
struct Metadata {
    position: usize,
    size: usize,
}

fn main() {
    let file_path = r"data\day9.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let mut reader = BufReader::new(file);

    let mut data = String::new();
    reader.read_to_string(&mut data).expect("Cannot read file");

    let data_bytes = data.as_bytes();

    let mut disk = Vec::new();
    let mut metadata = Vec::new();

    let mut file_index = 0;

    for (i, &ch) in data_bytes.iter().enumerate() {
        let n = ch - b'0';

        if i % 2 == 0 {
            for _ in 0..n {
                disk.push(Block::File(file_index));
            }

            metadata.push(Metadata {
                position: disk.len() - n as usize,
                size: n as usize,
            });

            file_index += 1;
        } else {
            for _ in 0..n {
                disk.push(Block::Free);
            }
        }
    }

    // part 1

    let mut disk_part1 = disk.clone();

    let mut left = 0;
    let mut right = disk_part1.len() - 1;
    while left <= right {
        while left <= right {
            match disk_part1[left] {
                Block::File(_) => left += 1,
                Block::Free => break,
            }
        }

        while left <= right {
            match disk_part1[right] {
                Block::File(_) => break,
                Block::Free => right -= 1,
            }
        }

        if left <= right {
            disk_part1[left] = disk_part1[right];
            disk_part1[right] = Block::Free;
        }
    }

    let mut result_part1 = 0;

    for (i, block) in disk_part1.iter().enumerate() {
        match block {
            Block::File(index) => result_part1 += i as i64 * (*index) as i64,
            Block::Free => continue,
        }
    }

    println!("Part 1: {}", result_part1);

    // part 2

    let mut disk_part2 = disk;

    for item in metadata.iter().rev() {
        let mut left = 0;
        let right = item.position;
        while left <= right {
            while left <= right {
                match disk_part2[left] {
                    Block::File(_) => left += 1,
                    Block::Free => break,
                }
            }

            if left >= right {
                break;
            }

            let free_block_size = free_block_count(&disk_part2, left, right);
            if free_block_size < item.size {
                left += free_block_size;
                continue;
            }

            for i in 0..item.size {
                disk_part2[left + i] = disk_part2[right + i];
                disk_part2[right + i] = Block::Free;
            }

            break;
        }
    }

    let mut result_part2 = 0;

    for (i, block) in disk_part2.iter().enumerate() {
        match block {
            Block::File(index) => result_part2 += i as i64 * (*index) as i64,
            Block::Free => continue,
        }
    }

    println!("Part 2: {}", result_part2);
}

fn free_block_count(disk: &[Block], mut left: usize, right: usize) -> usize {
    match disk[left] {
        Block::File(_) => 0,
        Block::Free => {
            let mut count = 1;
            left += 1;

            while let Block::Free = disk[left] {
                if left >= right {
                    break;
                }

                count += 1;
                left += 1;
            }

            count
        }
    }
}
