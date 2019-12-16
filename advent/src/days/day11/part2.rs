use crate::days::day11::part1;
use std::collections::HashMap;

pub fn run() {
    let mut grid = HashMap::<part1::Point, part1::Color>::new();
    grid.insert(part1::Point{x: 0, y: 0}, part1::Color::White);
    part1::run_program(&mut grid);
    let min_x = grid.keys().min_by_key(|p| p.x).unwrap().x;
    let min_y = grid.keys().min_by_key(|p| p.y).unwrap().y;
    let max_x = grid.keys().max_by_key(|p| p.x).unwrap().x;
    let max_y = grid.keys().max_by_key(|p| p.y).unwrap().y;
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            print!("{}", match grid.get(&part1::Point{x, y}) {
                Some(part1::Color::White) => ".",
                Some(part1::Color::Black) => "#",
                None => "#",
            });
        }
        println!("");
    }
}