use crate::utils;
use std::ops::RangeInclusive;

type Grid = Vec<Vec<i32>>;
type Point = (usize, usize);
const GRID_SIZE: usize = 100000;
const START_POINT: Point = (GRID_SIZE/2, GRID_SIZE/2);

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day3/input.txt");
    let wire1 = lines[0].clone();
    let wire2 = lines[1].clone();
    let grid = fill_grid(wire1);
    match_grid(wire2, grid);
}

fn fill_grid(wire: String) -> Grid {
    let instructions = get_instructions(wire);
    let mut grid = vec![vec![0; GRID_SIZE]; GRID_SIZE];
    let mut cursor = START_POINT;

    let mut steps = 0;
    for inst in instructions {
        for n in get_range(inst.x) {
            let p = get_point(cursor, &Instruction{x: n, y: 0});
            grid[p.0][p.1] = steps + n.abs();
        }
        for n in get_range(inst.y) {
            let p = get_point(cursor, &Instruction{x: 0, y: n});
            grid[p.0][p.1] = steps + n.abs();
        }
        cursor = get_point(cursor, &inst);
        steps += inst.x.abs() + inst.y.abs();
    }

    grid
}

fn match_grid(wire: String, grid: Grid) {
    let instructions = get_instructions(wire);
    let mut cursor = START_POINT;
    let mut closest_dist = std::i32::MAX;
    let mut steps = 0;
    for (index, inst) in instructions.iter().enumerate() {
        for n in get_range(inst.x) {
            let p = get_point(cursor, &Instruction{x: n, y: 0});
            if grid[p.0][p.1] > 0 && (index != 0 || p != START_POINT) {
                let dist = steps + n.abs() + grid[p.0][p.1];
                if dist < closest_dist {
                    closest_dist = dist;
                }
            }
        }
        for n in get_range(inst.y) {
            let p = get_point(cursor, &Instruction{x: 0, y: n});
            if grid[p.0][p.1] > 0 && (index != 0 || p != START_POINT) {
                let dist = steps + n.abs() + grid[p.0][p.1];
                if dist < closest_dist {
                    closest_dist = dist;
                }
            }
        }
        cursor = get_point(cursor, &inst);
        steps += inst.x.abs() + inst.y.abs();
    }
    println!("{}", closest_dist);
}

fn get_instructions(wire: String) -> Vec<Instruction> {
    wire.split(",").map(|x| new_instruction(x)).collect()
}

fn get_point(cursor: Point, inst: &Instruction) -> Point {
    ((cursor.0 as i32 + inst.x) as usize, (cursor.1 as i32 + inst.y) as usize)
}

struct Instruction {
    x: i32,
    y: i32,
}

fn get_range(x: i32) -> RangeInclusive::<i32> {
    if x > 0 {
        (0..=x)
    } else {
        (x..=0)
    }
}

fn new_instruction(code: &str) -> Instruction {
    match &code[..1] {
        "D" => Instruction{x: 0, y: code[1..].parse::<i32>().unwrap() * (-1)},
        "U" => Instruction{x: 0, y: code[1..].parse::<i32>().unwrap()},
        "L" => Instruction{y: 0, x: code[1..].parse::<i32>().unwrap() * (-1)},
        "R" => Instruction{y: 0, x: code[1..].parse::<i32>().unwrap()},
        _ => panic!(code.to_ascii_uppercase())
    }
}