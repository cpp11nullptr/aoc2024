use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "data/day10.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut trailmap = Vec::new();
    let mut trailheads = Vec::new();

    let lines = reader.lines();
    for (i, line) in lines.enumerate() {
        let line = line.expect("Cannot read line");

        let line_bytes = line
            .into_bytes()
            .iter()
            .map(|x| x - b'0')
            .collect::<Vec<u8>>();

        for (j, &item) in line_bytes.iter().enumerate() {
            if item == 0 {
                trailheads.push((i, j));
            }
        }

        trailmap.push(line_bytes);
    }

    let mut result: Vec<Vec<(usize, usize)>> = Vec::new();

    for (x, y) in trailheads {
        let route = vec![(x, y)];
        if let Some(paths) = trailpaths(&trailmap, route) {
            result.extend(paths);
        }
    }

    let mut trail_points = HashSet::new();

    for steps in &result {
        let head = steps.first().copied().expect("Trail head is empty");
        let tail = steps.last().copied().expect("Trail tail is empty");
        trail_points.insert((head, tail));
    }

    println!("Part 1: {}", trail_points.len());

    println!("Part 2: {}", result.len());
}

fn trailpaths(map: &[Vec<u8>], route: Vec<(usize, usize)>) -> Option<Vec<Vec<(usize, usize)>>> {
    let (x, y) = route.last().copied().expect("Route is empty");

    let h = map[x][y];
    if h == 9 {
        return Some(vec![route]);
    }

    let mut result = Vec::new();

    // north
    if x > 0 && map[x - 1][y] == h + 1 {
        let mut route = route.clone();
        route.push((x - 1, y));
        if let Some(paths) = trailpaths(map, route) {
            result.extend(paths);
        }
    }

    // east
    if y + 1 < map[x].len() && map[x][y + 1] == h + 1 {
        let mut route = route.clone();
        route.push((x, y + 1));
        if let Some(paths) = trailpaths(map, route) {
            result.extend(paths);
        }
    }

    // south
    if x + 1 < map.len() && map[x + 1][y] == h + 1 {
        let mut route = route.clone();
        route.push((x + 1, y));
        if let Some(paths) = trailpaths(map, route) {
            result.extend(paths);
        }
    }

    // west
    if y > 0 && map[x][y - 1] == h + 1 {
        let mut route = route.clone();
        route.push((x, y - 1));
        if let Some(paths) = trailpaths(map, route) {
            result.extend(paths);
        }
    }

    if result.is_empty() {
        return None;
    }

    Some(result)
}
