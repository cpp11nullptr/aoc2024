use std::fs::File;
use std::io::{BufReader, Read};

enum State {
    None,
    OpMul,
    OpMulOpenBracket,
    OpMulFirstNumber,
    OpMulComma,
    OpMulSecondNumber,
    OpMulCloseBracket,
}

fn main() {
    let file_path = r"data/day3.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let mut reader = BufReader::new(file);

    let mut data = String::new();
    reader.read_to_string(&mut data).expect("Cannot read file");

    let data = data.as_bytes();

    let mut i = 0;
    let mut state = State::None;

    let mut x = 0;
    let mut y = 0;
    let mut result = 0;

    let mut is_enabled = true;

    while i < data.len() {
        match state {
            State::OpMulCloseBracket => {}
            _ => {
                if try_enabled(data, &mut i, &mut is_enabled) {
                    continue;
                }
            }
        }

        match state {
            State::None => {
                if (i + 3) >= data.len() {
                    break;
                }

                if &data[i..i + 3] != b"mul" {
                    i += 1;
                    continue;
                }

                i += 3;
                state = State::OpMul;
            }
            State::OpMul => {
                if data[i] != b'(' {
                    i += 1;
                    state = State::None;
                    continue;
                }

                i += 1;
                state = State::OpMulOpenBracket;
            }
            State::OpMulOpenBracket => {
                if !data[i].is_ascii_digit() {
                    i += 1;
                    state = State::None;
                    continue;
                }

                x = data[i] as u32 - '0' as u32;

                i += 1;
                state = State::OpMulFirstNumber;

                for _ in 0..3 {
                    if data[i].is_ascii_digit() {
                        x *= 10;
                        x += data[i] as u32 - '0' as u32;
                        i += 1;
                    }
                }
            }
            State::OpMulFirstNumber => {
                if data[i] != b',' {
                    i += 1;
                    state = State::None;
                    continue;
                }

                i += 1;
                state = State::OpMulComma;
            }
            State::OpMulComma => {
                if !data[i].is_ascii_digit() {
                    i += 1;
                    state = State::None;
                    continue;
                }

                y = data[i] as u32 - '0' as u32;

                i += 1;
                state = State::OpMulSecondNumber;

                for _ in 0..3 {
                    if data[i].is_ascii_digit() {
                        y *= 10;
                        y += data[i] as u32 - '0' as u32;
                        i += 1;
                    }
                }
            }
            State::OpMulSecondNumber => {
                if data[i] != b')' {
                    i += 1;
                    state = State::None;
                    continue;
                }

                i += 1;
                state = State::OpMulCloseBracket;
            }
            State::OpMulCloseBracket => {
                if is_enabled {
                    result += x * y;
                }

                state = State::None;
            }
        }
    }

    println!("Part 2: {}", result);
}

fn try_enabled(data: &[u8], i: &mut usize, is_enabled: &mut bool) -> bool {
    if (*i + 4) < data.len() && &data[*i..*i + 4] == b"do()" {
        *i += 4;
        *is_enabled = true;
        return true;
    }

    if (*i + 7) < data.len() && &data[*i..*i + 7] == b"don't()" {
        *i += 7;
        *is_enabled = false;
        return true;
    }

    false
}
