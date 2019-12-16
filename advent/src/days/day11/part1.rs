use crate::utils;
use crate::days::day5::part2;
use std::collections::HashMap;

pub fn run() {
    let mut grid = HashMap::<Point, Color>::new();
    run_program(&mut grid);
    println!("{}", grid.len());
}

pub fn run_program(grid: &mut HashMap::<Point, Color>) {
    let lines = utils::lines_from_file("./src/days/day11/input.txt");
    let mut halted = false;
    let mut program = part2::Program::new(&lines[0]);
    let mut robot = Robot::new();
    while !halted {
        let input = get_color_input(&grid, &robot.position);
        let mut operator = part2::FromListProvider::new(vec![input]);
        let output = program.run(&mut operator);
        if program.halted {
            halted = true;
        } else {
            let color = if output == 0 { Color::Black } else { Color::White };
            grid.insert(robot.position.clone(), color);
            let output = program.run(&mut operator);
            robot.turn_around(output as usize);
            robot.move_forward();
            if program.halted {
                halted = true;
            }
        }
    }
}

fn get_color_input(grid: &HashMap::<Point, Color>, point: &Point) -> isize {
    match grid.get(point) {
        None => 0,
        Some(Color::Black) => 0,
        Some(Color::White) => 1,
    }
}

enum Direction{
    Left,
    Right,
    Up,
    Down,
}

pub enum Color {
    Black,
    White,
}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct Point {
    pub x: isize,
    pub y: isize,
}

impl Point{
    fn new(x: isize, y: isize) -> Point {
        Point{x, y}
    }
}

struct Robot{
    direction: Direction,
    position: Point,
}

impl Robot{
    fn new() -> Robot {
        Robot{direction: Direction::Up, position: Point::new(0, 0)}
    }

    fn turn_around(&mut self, instruction: usize) {
        self.direction = self.get_direction(instruction);
    }

    fn move_forward(&mut self) {
        self.position = self.get_new_position();
    }

    fn get_direction(&self, instruction: usize) -> Direction {
        match self.direction {
            Direction::Down => if instruction == 0 { Direction::Right } else { Direction::Left },
            Direction::Up => if instruction == 0 { Direction::Left } else { Direction::Right },
            Direction::Left => if instruction == 0 { Direction::Down } else { Direction::Up },
            Direction::Right => if instruction == 0 { Direction::Up } else { Direction::Down },
        }
    }

    fn get_new_position(&self) -> Point {
        match self.direction {
            Direction::Down => Point::new(self.position.x, self.position.y - 1),
            Direction::Up => Point::new(self.position.x, self.position.y + 1),
            Direction::Left => Point::new(self.position.x - 1, self.position.y),
            Direction::Right => Point::new(self.position.x + 1, self.position.y),
        }
    }
}