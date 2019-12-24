use crate::utils;
use crate::days::day5::part2::{Operator, Program};
use std::collections::{HashMap};

pub fn run() {
    let code = &utils::lines_from_file("./src/days/day15/input.txt")[0];
    let mut solver = Solver::new(code);
    solver.run(1);
    println!("{}", solver.get_result());
}

struct Solver {
    program: Program,
    droid: Droid,
    points: HashMap::<Point, isize>,
    current_point: Point,
    target: Point,
}

impl Solver {
    fn new(code: &str) -> Solver {
        let mut points = HashMap::new();
        let current_point = Point{x: 0, y: 0};
        points.insert(current_point, 0);
        Solver{program: Program::new(code), droid: Droid::new(), points, current_point, target: Point{x: 0, y: 0}}
    }

    fn update(&mut self, command: isize, current_value: isize, should_continue: bool) -> Point {
        self.current_point = move_next(self.current_point, command);
        let point_last_value = self.points.entry(self.current_point).or_default();
        if *point_last_value == 0 || *point_last_value > current_value {
            self.points.insert(self.current_point, current_value);
            if should_continue {
                self.run(current_value + 1);
            }
        }
        self.droid.set_command(reverse(command));
        self.program.run(&mut self.droid);
        let return_point = self.current_point;
        self.current_point = move_next(self.current_point, reverse(command));
        return_point
    }

    fn run(&mut self, current_value: isize) {
        let commands = vec![1,2,3,4];
        for command in commands {
            self.droid.set_command(command);
            let output = self.program.run(&mut self.droid);

            match output {
                0 => {},
                1 => {
                    self.update(command, current_value, true);
                },
                2 => {
                    self.target = self.update(command, current_value, false);
                },
                _ => panic!("unknown output: {}", output),
            }
        }
    }

    fn get_result(self) -> isize {
        self.points[&self.target]
    }
}

fn move_next(point: Point, command: isize) -> Point {
    match command {
        1 => Point{x: point.x, y: point.y - 1},
        2 => Point{x: point.x, y: point.y + 1},
        3 => Point{x: point.x - 1, y: point.y},
        4 => Point{x: point.x + 1, y: point.y},
        _ => panic!("unknown command {}", command),
    }
}

fn reverse(command: isize) -> isize {
    match command {
        1 => 2,
        2 => 1,
        3 => 4,
        4 => 3,
        _ => panic!("unknown conmmand {}", command),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct Point {
    x: isize,
    y: isize,
}

struct Droid {
    command: isize,
}

impl Droid {
    fn new() -> Droid {
        Droid{command: 1}
    }

    fn set_command(&mut self, command: isize) {
        self.command = command;
    }
}

impl Operator for Droid {
    fn next(&mut self) -> isize {
        self.command
    }
}