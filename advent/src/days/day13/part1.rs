use crate::utils;
use crate::days::day5::part2::{FromListProvider, Program};
use std::collections::HashMap;

pub fn run(){
    let lines = utils::lines_from_file("./src/days/day13/input.txt");
    let mut program = Program::new(&lines[0]);
    let mut points = HashMap::<Point, isize>::new();
    let (mut step, mut x, mut y) = (0, 0, 0);
    while !program.halted {
        let output = program.run(&mut FromListProvider::new(vec![]));
        match step % 3 {
            0 => { x = output; },
            1 => { y = output; },
            2 => { points.insert(Point{x, y}, output); },
            _ => panic!("Can't happen"),
        }
        step += 1;
    }
    let blocks = points.values().filter(|x| **x == 2).count();
    println!("{}", blocks);
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point{
    x: isize,
    y: isize,
}