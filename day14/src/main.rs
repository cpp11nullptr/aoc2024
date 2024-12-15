use image::{ImageBuffer, Rgb};
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, BufWriter, Write};

#[allow(dead_code)]
#[derive(Debug)]
struct Robot {
    pos: (i32, i32),
    vel: (i32, i32),
}

fn main() {
    let file_path = "data/day14.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut robots = Vec::new();

    let lines = reader.lines();
    for line in lines {
        let line = line.expect("Cannot read line");
        let line_parts = line.split(' ').collect::<Vec<&str>>();

        let pos = parse_xy(line_parts[0]);
        let vel = parse_xy(line_parts[1]);

        let robot = Robot { pos, vel };

        robots.push(robot);
    }

    const XMAX: i32 = 101;
    const YMAX: i32 = 103;

    // part 1

    const TIME: i32 = 100;

    let mut map = HashMap::new();

    for robot in &robots {
        let pos = (
            (calc_value(robot.pos.0, robot.vel.0, TIME, XMAX)),
            (calc_value(robot.pos.1, robot.vel.1, TIME, YMAX)),
        );

        let entry = map.entry(pos).or_insert(0);
        *entry += 1;
    }

    let q1 = quad_sum(&map, 0, XMAX / 2, 0, YMAX / 2);
    let q2 = quad_sum(&map, XMAX / 2 + 1, XMAX, 0, YMAX / 2);
    let q3 = quad_sum(&map, XMAX / 2 + 1, XMAX, YMAX / 2 + 1, YMAX);
    let q4 = quad_sum(&map, 0, XMAX / 2, YMAX / 2 + 1, YMAX);

    let result1 = q1 * q2 * q3 * q4;

    println!("Part 1: {}", result1);

    // part 2

    let mut result2 = 0;

    for t in 0..100000 {
        let mut map = HashMap::new();

        for robot in &robots {
            let pos = (
                (calc_value(robot.pos.0, robot.vel.0, t, XMAX)),
                (calc_value(robot.pos.1, robot.vel.1, t, YMAX)),
            );

            let entry = map.entry(pos).or_insert(0);
            *entry += 1;
        }

        if check_pattern(&map, XMAX, YMAX) {
            save_png(&map, XMAX, YMAX, t);
            result2 = t;
            break;
        }
    }

    println!("Part 2: {}", result2);
}

fn check_pattern(map: &HashMap<(i32, i32), i32>, xmax: i32, ymax: i32) -> bool {
    for x in 2..xmax {
        for y in 2..ymax {
            if map.get(&(x, y)).unwrap_or(&0) > &0
                && map.get(&(x + 1, y)).unwrap_or(&0) > &0
                && map.get(&(x + 2, y)).unwrap_or(&0) > &0
                && map.get(&(x - 1, y)).unwrap_or(&0) > &0
                && map.get(&(x - 2, y)).unwrap_or(&0) > &0
                && map.get(&(x, y - 1)).unwrap_or(&0) > &0
                && map.get(&(x + 1, y - 1)).unwrap_or(&0) > &0
                && map.get(&(x + 2, y - 1)).unwrap_or(&0) > &0
                && map.get(&(x - 1, y - 1)).unwrap_or(&0) > &0
                && map.get(&(x - 2, y - 1)).unwrap_or(&0) > &0
                && map.get(&(x + 1, y - 2)).unwrap_or(&0) > &0
                && map.get(&(x + 2, y - 2)).unwrap_or(&0) > &0
                && map.get(&(x - 1, y - 2)).unwrap_or(&0) > &0
                && map.get(&(x - 2, y - 2)).unwrap_or(&0) > &0
            {
                return true;
            }
        }
    }

    false
}

fn quad_sum(map: &HashMap<(i32, i32), i32>, xmin: i32, xmax: i32, ymin: i32, ymax: i32) -> i32 {
    let mut result = 0;

    for y in ymin..ymax {
        for x in xmin..xmax {
            result += match map.get(&(x, y)) {
                Some(&value) => value,
                None => 0,
            }
        }
    }

    result
}

#[allow(dead_code)]
fn print(map: &HashMap<(i32, i32), i32>, xmax: i32, ymax: i32) {
    for y in 0..ymax {
        for x in 0..xmax {
            match map.get(&(x, y)) {
                Some(value) => print!("{}", value),
                None => print!("."),
            }
        }

        println!();
    }
}

fn save_png(map: &HashMap<(i32, i32), i32>, xmax: i32, ymax: i32, t: i32) {
    let width = xmax as u32;
    let height = ymax as u32;
    let mut imgbuf = ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let colour: [u8; 3] = if map.get(&(x as i32, y as i32)).is_some() {
            [255, 255, 255]
        } else {
            [0, 0, 0]
        };
        *pixel = Rgb(colour);
    }

    let file_path = format!("data/day14out/{}.png", t);
    imgbuf
        .save(file_path)
        .unwrap_or_else(|_| panic!("Cannot save image: {}", t));
}

#[allow(dead_code)]
fn save_txt(map: &HashMap<(i32, i32), i32>, xmax: i32, ymax: i32) {
    let file_path = "data/day14output.txt";
    let file = File::create(file_path).expect("Cannot create file");
    let mut writer = BufWriter::new(file);
    for y in 0..ymax {
        for x in 0..xmax {
            match map.get(&(x, y)) {
                Some(&value) => {
                    let ch = value as u8 + b'0';
                    writer.write(&[ch])
                }
                None => writer.write(&[b'.']),
            }
            .expect("Cannot write data");
        }

        writer.write_all(&[b'\n']).expect("Cannot write new line");
    }
}

fn calc_value(x: i32, v: i32, t: i32, m: i32) -> i32 {
    match v.cmp(&0) {
        Ordering::Equal => x,
        Ordering::Greater => (x + t * v) % m,
        Ordering::Less => {
            let a = x + t * v;
            if a >= 0 {
                a
            } else {
                (m + a % m) % m
            }
        }
    }
}

fn parse_xy(value: &str) -> (i32, i32) {
    let point = value.split('=').collect::<Vec<&str>>();
    let point_parts = point[1].split(',').collect::<Vec<&str>>();

    let x = point_parts[0]
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("Cannot parse number: {}", point_parts[0]));

    let y = point_parts[1]
        .parse::<i32>()
        .unwrap_or_else(|_| panic!("Cannot parse number: {}", point_parts[1]));

    (x, y)
}
