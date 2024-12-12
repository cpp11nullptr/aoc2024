use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = r"data/day08.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut antennas: HashMap<u8, Vec<(usize, usize)>> = HashMap::new();

    let mut vmax: usize = 0;
    let mut hmax: usize = 0;

    let mut antinodes_part1: HashSet<(usize, usize)> = HashSet::new();
    let mut antinodes_part2: HashSet<(usize, usize)> = HashSet::new();

    let lines = reader.lines();
    for (i, line) in lines.into_iter().enumerate() {
        let line = line.expect("Cannot read line");
        let line_bytes = line.into_bytes();
        for (j, &ch) in line_bytes.iter().enumerate() {
            if ch != b'.' {
                let p = (i, j);
                let item = antennas.entry(ch).or_default();
                item.push(p);
                antinodes_part1.insert((i, j));
                antinodes_part2.insert((i, j));
            }

            if j + 1 > hmax {
                hmax = j + 1;
            }
        }

        if i + 1 > vmax {
            vmax = i + 1;
        }
    }

    for item in antennas {
        let pts = item.1;
        for (i, p1) in pts.iter().enumerate() {
            for p2 in pts.iter().skip(i + 1) {
                // part 1

                {
                    let x = if p1.0 < p2.0 {
                        let diff = p2.0 as i32 - p1.0 as i32;
                        (p1.0 as i32 - diff, p2.0 as i32 + diff)
                    } else {
                        let diff = p1.0 as i32 - p2.0 as i32;
                        (p1.0 as i32 + diff, p2.0 as i32 - diff)
                    };

                    let y = if p1.1 < p2.1 {
                        let diff = p2.1 as i32 - p1.1 as i32;
                        (p1.1 as i32 - diff, p2.1 as i32 + diff)
                    } else {
                        let diff = p1.1 as i32 - p2.1 as i32;
                        (p1.1 as i32 + diff, p2.1 as i32 - diff)
                    };

                    if x.0 >= 0 && x.0 < vmax as i32 && y.0 >= 0 && y.0 < hmax as i32 {
                        antinodes_part1.insert((x.0 as usize, y.0 as usize));
                    }

                    if x.1 >= 0 && x.1 < vmax as i32 && y.1 >= 0 && y.1 < hmax as i32 {
                        antinodes_part1.insert((x.1 as usize, y.1 as usize));
                    }
                }

                // part 2

                if p1.0 < p2.0 {
                    let diff0 = p2.0 as i32 - p1.0 as i32;
                    if p1.1 < p2.1 {
                        let diff1 = p2.1 as i32 - p1.1 as i32;
                        let mut x = (p1.0 as i32 - diff0, p1.1 as i32 - diff1);
                        while x.0 >= 0 && x.1 >= 0 {
                            antinodes_part2.insert((x.0 as usize, x.1 as usize));
                            x = (x.0 - diff0, x.1 - diff1);
                        }
                        let mut x = (p2.0 as i32 + diff0, p2.1 as i32 + diff1);
                        while x.0 < vmax as i32 && x.1 < hmax as i32 {
                            antinodes_part2.insert((x.0 as usize, x.1 as usize));
                            x = (x.0 + diff0, x.1 + diff1);
                        }
                    } else {
                        let diff1 = p1.1 as i32 - p2.1 as i32;
                        let mut x = (p1.0 as i32 - diff0, p1.1 as i32 + diff1);
                        while x.0 >= 0 && x.1 < hmax as i32 {
                            antinodes_part2.insert((x.0 as usize, x.1 as usize));
                            x = (x.0 - diff0, x.1 + diff1);
                        }
                        let mut x = (p2.0 as i32 + diff0, p2.1 as i32 - diff1);
                        while x.0 < vmax as i32 && x.1 >= 0 {
                            antinodes_part2.insert((x.0 as usize, x.1 as usize));
                            x = (x.0 + diff0, x.1 - diff1);
                        }
                    }
                } else {
                    let diff0 = p1.0 as i32 - p2.0 as i32;
                    if p1.1 < p2.1 {
                        let diff1 = p2.1 as i32 - p1.1 as i32;
                        let mut x = (p1.0 as i32 + diff0, p1.1 as i32 - diff1);
                        while x.0 < vmax as i32 && x.1 >= 0 {
                            antinodes_part2.insert((x.0 as usize, x.1 as usize));
                            x = (x.0 + diff0, x.1 - diff1);
                        }
                        let mut x = (p2.0 as i32 - diff0, p2.1 as i32 + diff1);
                        while x.0 >= 0 && x.1 < hmax as i32 {
                            antinodes_part2.insert((x.0 as usize, x.1 as usize));
                            x = (x.0 - diff0, x.1 + diff1);
                        }
                    } else {
                        let diff1 = p1.1 as i32 - p2.1 as i32;
                        let mut x = (p1.0 as i32 + diff0, p1.1 as i32 + diff1);
                        while x.0 < vmax as i32 && x.1 < hmax as i32 {
                            antinodes_part2.insert((x.0 as usize, x.1 as usize));
                            x = (x.0 + diff0, x.1 + diff1);
                        }
                        let mut x = (p2.0 as i32 - diff0, p2.1 as i32 - diff1);
                        while x.0 >= 0 && x.1 >= 0 {
                            antinodes_part2.insert((x.0 as usize, x.1 as usize));
                            x = (x.0 - diff0, x.1 - diff1);
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: {}", antinodes_part1.len());
    println!("Part 2: {}", antinodes_part2.len());
}
