use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn main() {
    let file_path = "data/day12.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut map = Vec::new();

    let lines = reader.lines();
    for line in lines {
        let line = line.expect("Cannot read line");
        let line_bytes = line.as_bytes().to_vec();
        map.push(line_bytes);
    }

    let mut regions: Vec<HashSet<(usize, usize)>> = Vec::new();

    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    for (i, row) in map.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            if visited.contains(&(i, j)) {
                continue;
            }

            let mut region_visited = HashSet::new();

            let region = connected(&map, i, j, &mut region_visited);

            regions.push(region);

            visited.extend(region_visited);
        }
    }

    let vmax = map.len();
    let hmax = map[0].len();

    // part 1

    let mut result1 = 0;

    for region in &regions {
        let area = region.len();
        let mut total_perimeter = 0;

        for garden in region {
            let x = garden.0;
            let y = garden.1;

            let mut perimeter = 4;

            if x > 0 && region.contains(&(x - 1, y)) {
                perimeter -= 1;
            }

            if x + 1 < vmax && region.contains(&(x + 1, y)) {
                perimeter -= 1;
            }

            if y > 0 && region.contains(&(x, y - 1)) {
                perimeter -= 1;
            }

            if y + 1 < hmax && region.contains(&(x, y + 1)) {
                perimeter -= 1;
            }

            total_perimeter += perimeter;
        }

        result1 += area * total_perimeter;
    }

    println!("Part 1: {}", result1);

    // part 2

    let mut result2 = 0;

    for region in &regions {
        let area = region.len();

        let mut corners = 0;

        for garden in region {
            let x = garden.0;
            let y = garden.1;

            // NW (outer)
            if (x == 0 || !region.contains(&(x - 1, y)))
                && (y == 0 || !region.contains(&(x, y - 1)))
            {
                corners += 1;
            }

            // NE (outer)
            if (x == 0 || !region.contains(&(x - 1, y)))
                && ((y + 1) == hmax || !region.contains(&(x, y + 1)))
            {
                corners += 1;
            }

            // SE (outer)
            if ((x + 1) == vmax || !region.contains(&(x + 1, y)))
                && ((y + 1) == hmax || !region.contains(&(x, y + 1)))
            {
                corners += 1;
            }

            // SW (outer)
            if ((x + 1) == vmax || !region.contains(&(x + 1, y)))
                && (y == 0 || !region.contains(&(x, y - 1)))
            {
                corners += 1;
            }

            // NW (inner)
            if x + 1 < vmax
                && region.contains(&(x + 1, y))
                && y + 1 < hmax
                && region.contains(&(x, y + 1))
                && !region.contains(&(x + 1, y + 1))
            {
                corners += 1;
            }

            // NE (inner)
            if x + 1 < vmax
                && region.contains(&(x + 1, y))
                && y > 0
                && region.contains(&(x, y - 1))
                && !region.contains(&(x + 1, y - 1))
            {
                corners += 1;
            }

            // SE (inner)
            if x > 0
                && region.contains(&(x - 1, y))
                && y > 0
                && region.contains(&(x, y - 1))
                && !region.contains(&(x - 1, y - 1))
            {
                corners += 1;
            }

            // SW (inner)
            if x > 0
                && region.contains(&(x - 1, y))
                && y + 1 < hmax
                && region.contains(&(x, y + 1))
                && !region.contains(&(x - 1, y + 1))
            {
                corners += 1;
            }
        }

        result2 += area * corners;
    }

    println!("Part 2: {}", result2);
}

fn connected(
    map: &[Vec<u8>],
    x: usize,
    y: usize,
    visited: &mut HashSet<(usize, usize)>,
) -> HashSet<(usize, usize)> {
    let garden = map[x][y];

    let mut result = HashSet::new();

    result.insert((x, y));

    visited.insert((x, y));

    if x + 1 < map.len() && map[x + 1][y] == garden && !visited.contains(&(x + 1, y)) {
        let link = connected(map, x + 1, y, visited);
        result.extend(link);
    }

    if x > 0 && map[x - 1][y] == garden && !visited.contains(&(x - 1, y)) {
        let link = connected(map, x - 1, y, visited);
        result.extend(link);
    }

    if y + 1 < map[x].len() && map[x][y + 1] == garden && !visited.contains(&(x, y + 1)) {
        let link = connected(map, x, y + 1, visited);
        result.extend(link);
    }

    if y > 0 && map[x][y - 1] == garden && !visited.contains(&(x, y - 1)) {
        let link = connected(map, x, y - 1, visited);
        result.extend(link);
    }

    result
}
