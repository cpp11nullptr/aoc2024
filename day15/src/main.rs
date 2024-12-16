use std::fs::File;
use std::io::{BufRead, BufReader};

enum ReadMode {
    Map,
    Moves,
}

#[derive(Clone, Copy, Debug)]
enum Movement {
    Up,
    Right,
    Down,
    Left,
}

#[derive(PartialEq)]
enum Object {
    Wall,
    Box,
    Robot,
    Empty,
}

#[derive(Clone, Copy, PartialEq)]
enum ObjectWide {
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
    Empty,
}

fn main() {
    let file_path = "data/day15.txt";
    let file = File::open(file_path).expect("Cannot open file");
    let reader = BufReader::new(file);

    let mut read_mode = ReadMode::Map;

    let mut robot = (0, 0);
    let mut map = Vec::new();
    let mut movements = Vec::new();

    let lines = reader.lines();
    for (i, line) in lines.enumerate() {
        let line = line.expect("Cannot read line");
        if line.trim().is_empty() {
            match read_mode {
                ReadMode::Map => {
                    read_mode = ReadMode::Moves;
                    continue;
                }
                ReadMode::Moves => break,
            }
        }

        let line_bytes = line.as_bytes();

        match read_mode {
            ReadMode::Map => {
                let mut objects = Vec::new();

                for (j, &byte) in line_bytes.iter().enumerate() {
                    let object = match byte {
                        b'#' => Object::Wall,
                        b'O' => Object::Box,
                        b'@' => {
                            robot = (i, j);
                            Object::Robot
                        }
                        b'.' => Object::Empty,
                        value => panic!("Unexpected object: {}", value),
                    };

                    objects.push(object);
                }

                map.push(objects);
            }
            ReadMode::Moves => {
                for &byte in line_bytes {
                    let movement = match byte {
                        b'^' => Movement::Up,
                        b'>' => Movement::Right,
                        b'v' => Movement::Down,
                        b'<' => Movement::Left,
                        value => panic!("Unexpected movement: {}", value),
                    };

                    movements.push(movement);
                }
            }
        }
    }

    let mut robot2 = (0, 0);
    let mut map2 = map_wide(&map);

    for (i, line) in map2.iter().enumerate() {
        for (j, &object) in line.iter().enumerate() {
            if object == ObjectWide::Robot {
                robot2 = (i, j);
                break;
            }
        }
    }

    // part 1

    move_robot(&mut map, &movements, &mut robot);

    let mut result1 = 0;

    for (x, line) in map.iter().enumerate() {
        for (y, object) in line.iter().enumerate() {
            if *object == Object::Box {
                result1 += x * 100 + y;
            }
        }
    }

    println!("Part 1: {}", result1);

    // part 2

    move_robot_wide(&mut map2, &movements, &mut robot2);

    let mut result2 = 0;

    for (x, line) in map2.iter().enumerate() {
        for (y, object) in line.iter().enumerate() {
            if *object == ObjectWide::BoxLeft {
                result2 += x * 100 + y;
            }
        }
    }

    println!("Part 2: {}", result2);
}

#[allow(dead_code)]
fn print(map: &[Vec<Object>]) {
    for line in map {
        for object in line {
            match object {
                Object::Wall => print!("#"),
                Object::Box => print!("O"),
                Object::Robot => print!("@"),
                Object::Empty => print!("."),
            }
        }

        println!();
    }
}

#[allow(dead_code)]
fn print_wide(map: &[Vec<ObjectWide>]) {
    for line in map {
        for object in line {
            match object {
                ObjectWide::Wall => print!("#"),
                ObjectWide::BoxLeft => print!("["),
                ObjectWide::BoxRight => print!("]"),
                ObjectWide::Robot => print!("@"),
                ObjectWide::Empty => print!("."),
            }
        }

        println!();
    }
}

fn move_robot(map: &mut Vec<Vec<Object>>, movements: &[Movement], robot: &mut (usize, usize)) {
    for &movement in movements {
        let x = robot.0;
        let y = robot.1;

        match movement {
            Movement::Up => {
                if x == 1 {
                    continue;
                }

                match map[x - 1][y] {
                    Object::Wall => continue,
                    Object::Box => {
                        if move_boxes(map, movement, x - 1, y) {
                            map[x - 1][y] = Object::Robot;
                            map[x][y] = Object::Empty;
                            *robot = (x - 1, y);
                        }
                    }
                    Object::Robot => panic!("Unexpected robot: ({}, {})", x - 1, y),
                    Object::Empty => {
                        map[x - 1][y] = Object::Robot;
                        map[x][y] = Object::Empty;
                        *robot = (x - 1, y);
                    }
                }
            }
            Movement::Right => {
                if y == map[0].len() - 2 {
                    continue;
                }

                match map[x][y + 1] {
                    Object::Wall => continue,
                    Object::Box => {
                        if move_boxes(map, movement, x, y + 1) {
                            map[x][y + 1] = Object::Robot;
                            map[x][y] = Object::Empty;
                            *robot = (x, y + 1);
                        }
                    }
                    Object::Robot => panic!("Unexpected robot: ({}, {})", x, y + 1),
                    Object::Empty => {
                        map[x][y + 1] = Object::Robot;
                        map[x][y] = Object::Empty;
                        *robot = (x, y + 1);
                    }
                }
            }
            Movement::Down => {
                if x == map.len() - 2 {
                    continue;
                }

                match map[x + 1][y] {
                    Object::Wall => continue,
                    Object::Box => {
                        if move_boxes(map, movement, x + 1, y) {
                            map[x + 1][y] = Object::Robot;
                            map[x][y] = Object::Empty;
                            *robot = (x + 1, y);
                        }
                    }
                    Object::Robot => panic!("Unexpected robot: ({}, {})", x, y + 1),
                    Object::Empty => {
                        map[x + 1][y] = Object::Robot;
                        map[x][y] = Object::Empty;
                        *robot = (x + 1, y);
                    }
                }
            }
            Movement::Left => {
                if y == 1 {
                    continue;
                }

                match map[x][y - 1] {
                    Object::Wall => continue,
                    Object::Box => {
                        if move_boxes(map, movement, x, y - 1) {
                            map[x][y - 1] = Object::Robot;
                            map[x][y] = Object::Empty;
                            *robot = (x, y - 1);
                        }
                    }
                    Object::Robot => panic!("Unexpected robot: ({}, {})", x, y - 1),
                    Object::Empty => {
                        map[x][y - 1] = Object::Robot;
                        map[x][y] = Object::Empty;
                        *robot = (x, y - 1);
                    }
                }
            }
        }
    }
}

fn move_robot_wide(
    map: &mut Vec<Vec<ObjectWide>>,
    movements: &[Movement],
    robot: &mut (usize, usize),
) {
    for &movement in movements {
        let x = robot.0;
        let y = robot.1;

        match movement {
            Movement::Up => {
                if x == 1 {
                    continue;
                }

                match map[x - 1][y] {
                    ObjectWide::Wall => continue,
                    ObjectWide::BoxLeft => {
                        if move_boxes_wide(map, movement, x - 1, y, true)
                            && move_boxes_wide(map, movement, x - 1, y + 1, true)
                        {
                            move_boxes_wide(map, movement, x - 1, y, false);
                            move_boxes_wide(map, movement, x - 1, y + 1, false);

                            map[x - 1][y] = ObjectWide::Robot;
                            map[x][y] = ObjectWide::Empty;
                            *robot = (x - 1, y);
                        }
                    }
                    ObjectWide::BoxRight => {
                        if move_boxes_wide(map, movement, x - 1, y, true)
                            && move_boxes_wide(map, movement, x - 1, y - 1, true)
                        {
                            move_boxes_wide(map, movement, x - 1, y, false);
                            move_boxes_wide(map, movement, x - 1, y - 1, false);

                            map[x - 1][y] = ObjectWide::Robot;
                            map[x][y] = ObjectWide::Empty;
                            *robot = (x - 1, y);
                        }
                    }
                    ObjectWide::Robot => panic!("Unexpected robot: ({}, {})", x - 1, y),
                    ObjectWide::Empty => {
                        map[x - 1][y] = ObjectWide::Robot;
                        map[x][y] = ObjectWide::Empty;
                        *robot = (x - 1, y);
                    }
                }
            }
            Movement::Right => {
                if y == map[0].len() - 2 {
                    continue;
                }

                match map[x][y + 1] {
                    ObjectWide::Wall => continue,
                    ObjectWide::BoxLeft => {
                        if move_boxes_wide(map, movement, x, y + 1, true) {
                            move_boxes_wide(map, movement, x, y + 1, false);

                            map[x][y + 1] = ObjectWide::Robot;
                            map[x][y] = ObjectWide::Empty;
                            *robot = (x, y + 1);
                        }
                    }
                    ObjectWide::BoxRight => panic!("Unexpected box right: ({}, {})", x, y + 1),
                    ObjectWide::Robot => panic!("Unexpected robot: ({}, {})", x, y + 1),
                    ObjectWide::Empty => {
                        map[x][y + 1] = ObjectWide::Robot;
                        map[x][y] = ObjectWide::Empty;
                        *robot = (x, y + 1);
                    }
                }
            }
            Movement::Down => {
                if x == map.len() - 2 {
                    continue;
                }

                match map[x + 1][y] {
                    ObjectWide::Wall => continue,
                    ObjectWide::BoxLeft => {
                        if move_boxes_wide(map, movement, x + 1, y, true)
                            && move_boxes_wide(map, movement, x + 1, y + 1, true)
                        {
                            move_boxes_wide(map, movement, x + 1, y, false);
                            move_boxes_wide(map, movement, x + 1, y + 1, false);

                            map[x + 1][y] = ObjectWide::Robot;
                            map[x][y] = ObjectWide::Empty;
                            *robot = (x + 1, y);
                        }
                    }
                    ObjectWide::BoxRight => {
                        if move_boxes_wide(map, movement, x + 1, y, true)
                            && move_boxes_wide(map, movement, x + 1, y - 1, true)
                        {
                            move_boxes_wide(map, movement, x + 1, y, false);
                            move_boxes_wide(map, movement, x + 1, y - 1, false);

                            map[x + 1][y] = ObjectWide::Robot;
                            map[x][y] = ObjectWide::Empty;
                            *robot = (x + 1, y);
                        }
                    }
                    ObjectWide::Robot => panic!("Unexpected robot: ({}, {})", x, y + 1),
                    ObjectWide::Empty => {
                        map[x + 1][y] = ObjectWide::Robot;
                        map[x][y] = ObjectWide::Empty;
                        *robot = (x + 1, y);
                    }
                }
            }
            Movement::Left => {
                if y == 1 {
                    continue;
                }

                match map[x][y - 1] {
                    ObjectWide::Wall => continue,
                    ObjectWide::BoxLeft => panic!("Unexpected box left: ({}, {})", x, y + 1),
                    ObjectWide::BoxRight => {
                        if move_boxes_wide(map, movement, x, y - 1, true) {
                            move_boxes_wide(map, movement, x, y - 1, false);

                            map[x][y - 1] = ObjectWide::Robot;
                            map[x][y] = ObjectWide::Empty;
                            *robot = (x, y - 1);
                        }
                    }
                    ObjectWide::Robot => panic!("Unexpected robot: ({}, {})", x, y - 1),
                    ObjectWide::Empty => {
                        map[x][y - 1] = ObjectWide::Robot;
                        map[x][y] = ObjectWide::Empty;
                        *robot = (x, y - 1);
                    }
                }
            }
        }
    }
}

fn move_boxes(map: &mut Vec<Vec<Object>>, movement: Movement, x: usize, y: usize) -> bool {
    match movement {
        Movement::Up => match map[x - 1][y] {
            Object::Wall => false,
            Object::Box => {
                if move_boxes(map, movement, x - 1, y) {
                    map[x - 1][y] = Object::Box;
                    map[x][y] = Object::Empty;
                    true
                } else {
                    false
                }
            }
            Object::Robot => panic!("Unexpected robot: ({}, {})", x - 1, y),
            Object::Empty => {
                map[x - 1][y] = Object::Box;
                map[x][y] = Object::Empty;
                true
            }
        },
        Movement::Right => match map[x][y + 1] {
            Object::Wall => false,
            Object::Box => {
                if move_boxes(map, movement, x, y + 1) {
                    map[x][y + 1] = Object::Box;
                    map[x][y] = Object::Empty;
                    true
                } else {
                    false
                }
            }
            Object::Robot => panic!("Unexpected robot: ({}, {})", x, y + 1),
            Object::Empty => {
                map[x][y + 1] = Object::Box;
                map[x][y] = Object::Empty;
                true
            }
        },
        Movement::Down => match map[x + 1][y] {
            Object::Wall => false,
            Object::Box => {
                if move_boxes(map, movement, x + 1, y) {
                    map[x + 1][y] = Object::Box;
                    map[x][y] = Object::Empty;
                    true
                } else {
                    false
                }
            }
            Object::Robot => panic!("Unexpected robot: ({}, {})", x + 1, y),
            Object::Empty => {
                map[x + 1][y] = Object::Box;
                map[x][y] = Object::Empty;
                true
            }
        },
        Movement::Left => match map[x][y - 1] {
            Object::Wall => false,
            Object::Box => {
                if move_boxes(map, movement, x, y - 1) {
                    map[x][y - 1] = Object::Box;
                    map[x][y] = Object::Empty;
                    true
                } else {
                    false
                }
            }
            Object::Robot => panic!("Unexpected robot: ({}, {})", x, y - 1),
            Object::Empty => {
                map[x][y - 1] = Object::Box;
                map[x][y] = Object::Empty;
                true
            }
        },
    }
}

fn map_wide(map: &[Vec<Object>]) -> Vec<Vec<ObjectWide>> {
    let mut result = Vec::new();

    for line in map {
        let mut line_wide = Vec::new();

        for object in line {
            match object {
                Object::Wall => {
                    line_wide.push(ObjectWide::Wall);
                    line_wide.push(ObjectWide::Wall);
                }
                Object::Box => {
                    line_wide.push(ObjectWide::BoxLeft);
                    line_wide.push(ObjectWide::BoxRight);
                }
                Object::Robot => {
                    line_wide.push(ObjectWide::Robot);
                    line_wide.push(ObjectWide::Empty);
                }
                Object::Empty => {
                    line_wide.push(ObjectWide::Empty);
                    line_wide.push(ObjectWide::Empty);
                }
            }
        }

        result.push(line_wide);
    }

    result
}

fn move_boxes_wide(
    map: &mut Vec<Vec<ObjectWide>>,
    movement: Movement,
    x: usize,
    y: usize,
    is_lookup: bool,
) -> bool {
    match movement {
        Movement::Up => match map[x - 1][y] {
            ObjectWide::Wall => false,
            ObjectWide::BoxLeft => {
                if move_boxes_wide(map, movement, x - 1, y, true)
                    && move_boxes_wide(map, movement, x - 1, y + 1, true)
                {
                    move_boxes_wide(map, movement, x - 1, y, is_lookup);
                    move_boxes_wide(map, movement, x - 1, y + 1, is_lookup);

                    if !is_lookup {
                        map[x - 1][y] = map[x][y];
                        map[x][y] = ObjectWide::Empty;
                    }

                    true
                } else {
                    false
                }
            }
            ObjectWide::BoxRight => {
                if move_boxes_wide(map, movement, x - 1, y, true)
                    && move_boxes_wide(map, movement, x - 1, y - 1, true)
                {
                    move_boxes_wide(map, movement, x - 1, y, is_lookup);
                    move_boxes_wide(map, movement, x - 1, y - 1, is_lookup);

                    if !is_lookup {
                        map[x - 1][y] = map[x][y];
                        map[x][y] = ObjectWide::Empty;
                    }

                    true
                } else {
                    false
                }
            }
            ObjectWide::Robot => panic!("Unexpected robot: ({}, {})", x - 1, y),
            ObjectWide::Empty => {
                if !is_lookup {
                    map[x - 1][y] = map[x][y];
                    map[x][y] = ObjectWide::Empty;
                }

                true
            }
        },
        Movement::Right => match map[x][y + 1] {
            ObjectWide::Wall => false,
            ObjectWide::BoxLeft | ObjectWide::BoxRight => {
                if move_boxes_wide(map, movement, x, y + 1, true) {
                    move_boxes_wide(map, movement, x, y + 1, is_lookup);

                    if !is_lookup {
                        map[x][y + 1] = map[x][y];
                        map[x][y] = ObjectWide::Empty;
                    }

                    true
                } else {
                    false
                }
            }
            ObjectWide::Robot => panic!("Unexpected robot: ({}, {})", x, y + 1),
            ObjectWide::Empty => {
                if !is_lookup {
                    map[x][y + 1] = map[x][y];
                    map[x][y] = ObjectWide::Empty;
                }

                true
            }
        },
        Movement::Down => match map[x + 1][y] {
            ObjectWide::Wall => false,
            ObjectWide::BoxLeft => {
                if move_boxes_wide(map, movement, x + 1, y, true)
                    && move_boxes_wide(map, movement, x + 1, y + 1, true)
                {
                    move_boxes_wide(map, movement, x + 1, y, is_lookup);
                    move_boxes_wide(map, movement, x + 1, y + 1, is_lookup);

                    if !is_lookup {
                        map[x + 1][y] = map[x][y];
                        map[x][y] = ObjectWide::Empty;
                    }

                    true
                } else {
                    false
                }
            }
            ObjectWide::BoxRight => {
                if move_boxes_wide(map, movement, x + 1, y, true)
                    && move_boxes_wide(map, movement, x + 1, y - 1, true)
                {
                    move_boxes_wide(map, movement, x + 1, y, is_lookup);
                    move_boxes_wide(map, movement, x + 1, y - 1, is_lookup);

                    if !is_lookup {
                        map[x + 1][y] = map[x][y];
                        map[x][y] = ObjectWide::Empty;
                    }

                    true
                } else {
                    false
                }
            }
            ObjectWide::Robot => panic!("Unexpected robot: ({}, {})", x + 1, y),
            ObjectWide::Empty => {
                if !is_lookup {
                    map[x + 1][y] = map[x][y];
                    map[x][y] = ObjectWide::Empty;
                }

                true
            }
        },
        Movement::Left => match map[x][y - 1] {
            ObjectWide::Wall => false,
            ObjectWide::BoxLeft | ObjectWide::BoxRight => {
                if move_boxes_wide(map, movement, x, y - 1, true) {
                    move_boxes_wide(map, movement, x, y - 1, is_lookup);

                    if !is_lookup {
                        map[x][y - 1] = map[x][y];
                        map[x][y] = ObjectWide::Empty;
                    }

                    true
                } else {
                    false
                }
            }
            ObjectWide::Robot => panic!("Unexpected robot: ({}, {})", x, y - 1),
            ObjectWide::Empty => {
                if !is_lookup {
                    map[x][y - 1] = map[x][y];
                    map[x][y] = ObjectWide::Empty;
                }

                true
            }
        },
    }
}
