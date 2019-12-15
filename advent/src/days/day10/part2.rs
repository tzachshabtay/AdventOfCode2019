use crate::utils;
use crate::days::day10::part1::{FoundVec, get_visible_astroids, find_monitoring_station};
use std::collections::HashSet;

pub fn run(){
    let mut lines = utils::lines_from_file("./src/days/day10/input.txt");
    let lines_tmp = lines.clone();
    let station = find_monitoring_station(lines_tmp);
    println!("Found station at {:?}", station);
    find_from(station.0 as isize, station.1 as isize, 200 - 1, &mut lines);
}

fn get_angle(vec: &FoundVec) -> f64 {
    let (x, y) = (vec.step_x, -vec.step_y);
    let (a, b) = (x as f64, y as f64);
    let degrees = (a / b).atan().to_degrees();
    if x >= 0 && y >= 0 { //top-right quadrant
        return degrees;
    }
    if y < 0 { //bottom quadrant
        return 180.0 + degrees;
    }
    360.0 + degrees //top-left quadrant
}

//8,3
fn find_from(x: isize, y: isize, mut index: usize, map: &mut Vec<String>) {
    let mut visible = get_visible_astroids(x, y, map);
    while index > visible.len() && visible.len() > 0 {
        destroy_astroids(x, y, map, &visible);
        visible = get_visible_astroids(x, y, map);
        index -= visible.len();
    }
    let mut sorted = visible.iter().collect::<Vec<&FoundVec>>();
    sorted.sort_by_key(|c| get_angle(c) as isize);
    println!("Result is: {}", (sorted[index].step_x + x) * 100 + sorted[index].step_y + y);
}

fn destroy_astroids(x: isize, y: isize, map: &mut Vec<String>, astroids: &HashSet<FoundVec>) {
    for astroid in astroids {
        let row = destroy_astroid_at(&map[(astroid.step_y + y) as usize], (astroid.step_x + x) as usize);
        map[(astroid.step_y + y) as usize] = row;
    }
}

fn destroy_astroid_at(line: &str, index: usize) -> String {
    line.chars().enumerate()
        .map(|(i, c)| if i == index { '.' } else { c })
        .collect()
}
