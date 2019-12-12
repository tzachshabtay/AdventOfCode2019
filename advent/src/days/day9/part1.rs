use crate::utils;
use crate::days::day5::part2;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day9/input.txt");
    part2::run_program(&lines[0], part2::FromListProvider::new(vec![1]));
}