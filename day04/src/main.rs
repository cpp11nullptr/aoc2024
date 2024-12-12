use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = r"data/day04.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut data = Vec::new();

    let lines = reader.lines();
    for line in lines {
        let line = line.expect("Cannot read line");
        let line_bytes = line.into_bytes();
        data.push(line_bytes);
    }

    let mut count = 0;

    let vmax = data.len();
    let hmax = data[0].len();

    // part 1

    for i in 0..vmax {
        for j in 0..hmax {
            if i >= 3 {
                // ..^..
                // ..|..
                // ..x..
                // .....
                // .....
                if data[i][j] == b'X'
                    && data[i - 1][j] == b'M'
                    && data[i - 2][j] == b'A'
                    && data[i - 3][j] == b'S'
                {
                    count += 1;
                }
            }

            if i + 3 < vmax {
                // .....
                // .....
                // ..x..
                // ..|..
                // ..v..
                if data[i][j] == b'X'
                    && data[i + 1][j] == b'M'
                    && data[i + 2][j] == b'A'
                    && data[i + 3][j] == b'S'
                {
                    count += 1;
                }
            }

            if j >= 3 {
                // .....
                // .....
                // <-x..
                // .....
                // .....
                if data[i][j] == b'X'
                    && data[i][j - 1] == b'M'
                    && data[i][j - 2] == b'A'
                    && data[i][j - 3] == b'S'
                {
                    count += 1;
                }

                if i + 3 < vmax {
                    // .....
                    // .....
                    // ..x..
                    // .-...
                    // <....
                    if data[i][j] == b'X'
                        && data[i + 1][j - 1] == b'M'
                        && data[i + 2][j - 2] == b'A'
                        && data[i + 3][j - 3] == b'S'
                    {
                        count += 1;
                    }
                }

                if i >= 3 {
                    // <....
                    // .-...
                    // ..x..
                    // .....
                    // .....
                    if data[i][j] == b'X'
                        && data[i - 1][j - 1] == b'M'
                        && data[i - 2][j - 2] == b'A'
                        && data[i - 3][j - 3] == b'S'
                    {
                        count += 1;
                    }
                }
            }

            if j + 3 < hmax {
                // .....
                // .....
                // ..x->
                // .....
                // .....
                if data[i][j] == b'X'
                    && data[i][j + 1] == b'M'
                    && data[i][j + 2] == b'A'
                    && data[i][j + 3] == b'S'
                {
                    count += 1;
                }

                if i + 3 < vmax {
                    // .....
                    // .....
                    // ..x..
                    // ...-.
                    // ....>
                    if data[i][j] == b'X'
                        && data[i + 1][j + 1] == b'M'
                        && data[i + 2][j + 2] == b'A'
                        && data[i + 3][j + 3] == b'S'
                    {
                        count += 1;
                    }
                }

                if i >= 3 {
                    // ....>
                    // ...-.
                    // ..x..
                    // .....
                    // .....
                    if data[i][j] == b'X'
                        && data[i - 1][j + 1] == b'M'
                        && data[i - 2][j + 2] == b'A'
                        && data[i - 3][j + 3] == b'S'
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    println!("Part 1: {}", count);

    // part 2

    let mut count = 0;

    for i in 1..vmax - 1 {
        for j in 1..hmax - 1 {
            if data[i][j] == b'A' {
                let is_word1 = data[i - 1][j - 1] == b'M' && data[i + 1][j + 1] == b'S'
                    || data[i - 1][j - 1] == b'S' && data[i + 1][j + 1] == b'M';
                let is_word2 = data[i + 1][j - 1] == b'M' && data[i - 1][j + 1] == b'S'
                    || data[i + 1][j - 1] == b'S' && data[i - 1][j + 1] == b'M';
                if is_word1 && is_word2 {
                    count += 1;
                }
            }
        }
    }

    println!("Part 2: {}", count);
}
