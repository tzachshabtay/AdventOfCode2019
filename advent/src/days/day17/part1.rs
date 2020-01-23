use crate::utils;
use std::collections::HashSet;
use crate::days::day5::part2::{FromListProvider, Program};

pub fn run() {
    let code = &utils::lines_from_file("./src/days/day17/input.txt")[0];
    let mut program = Program::new(code);
    let mut operator = FromListProvider::new(vec![]);
    let (mut x, mut y) = (0, 0);
    let mut points = HashSet::new();
    while !program.halted {
        let output = program.run(&mut operator);
        let c = output as u8 as char;
        print!("{}", c);
        match c {
            '\n' => { y += 1; x = 0; },
            '#' => { points.insert(Point{x,y}); x += 1; },
            _ => { x += 1; },
        }
    }
    let mut sum = 0;
    for point in points.iter() {
        if is_intersection(point, &points) {
            sum += point.x * point.y;
        }
    }
    println!("{}", sum);
}

fn is_intersection(point: &Point, points: &HashSet::<Point>) -> bool {
    if !points.contains(&Point{x: point.x+1, y: point.y}) { return false; }
    if !points.contains(&Point{x: point.x-1, y: point.y}) { return false; }
    if !points.contains(&Point{x: point.x, y: point.y+1}) { return false; }
    if !points.contains(&Point{x: point.x, y: point.y-1}) { return false; }
    true
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}