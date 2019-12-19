use crate::utils;
use crate::days::day5::part2::{Operator, Program};
use std::collections::HashMap;

pub fn run(){
    let mut lines = utils::lines_from_file("./src/days/day13/input.txt");
    let mut game = Game::new();
    game.play(&mut lines[0]);
}

#[derive(Hash, Eq, PartialEq, Debug, Clone)]
struct Point{
    x: isize,
    y: isize,
}

struct Game{
    ball: Point,
    paddle: Point,
    score: isize,
}

impl Game {
    fn new() -> Game {
        Game{ball: Point{x: 0, y: 0}, paddle: {Point{x: 0, y: 0}}, score: 0}
    }

    fn play(&mut self, code: &str) {
        let code = &mut code.replace("1,380", "2,380");
        let mut program = Program::new(code);
        let mut points = HashMap::<Point, isize>::new();
        let (mut step, mut x, mut y) = (0, 0, 0);
        while !program.halted {
            let output = program.run(self);
            match step % 3 {
                0 => { x = output; },
                1 => { y = output; },
                2 => {
                    if x == -1 && y == 0 {
                        println!("----- Score = {} -----", output);
                        self.score = output;
                    } else {
                        //println!("{} at {},{}", output, x, y);
                        points.insert(Point{x, y}, output);
                        match output {
                            3 => {
                                self.ball = Point{x, y};
                                println!("ball at {:?}", self.ball);
                            },
                            4 => {
                                self.paddle = Point{x, y};
                                println!("paddle at {:?}", self.paddle);
                            },
                            _ => {}
                        }
                    }
                },
                _ => panic!("Can't happen"),
            }
            step += 1;
        }
        println!("Score: {}", self.score);
    }
}

impl Operator for Game {
    fn next(&mut self) -> isize {
        if self.paddle.x < self.ball.x {
            println!("Moving right");
            return -1;
        }
        if self.paddle.x > self.ball.x {
            println!("Moving left");
            return 1;
        }
        println!("Not Moving");
        0
    }
}
