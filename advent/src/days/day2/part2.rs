use crate::utils;

use crate::days::day2::part1;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day2/input.txt");
    for line in lines {
        solve(line, 19690720);
    }
}

fn solve(program: String, target: usize) {
    for noun in 0..100 {
        for verb in 0..100 {
            let p = program.clone();
            let res = part1::run_program(p, noun, verb);
            if res == target {
                println!("{}", noun * 100 + verb);
                return;
            }
        }
    }
}