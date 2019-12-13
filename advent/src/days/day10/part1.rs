use crate::utils;
use std::cmp::max;
use std::collections::HashSet;
extern crate num;
use num::integer::gcd;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day10/input.txt");
    let map = lines.clone();

    let mut max_astroids = 0;
    for (y, line) in lines.iter().enumerate() {
        for (x, cell) in line.chars().enumerate() {
            if cell == '.' {
                continue;
            }
            let num_astroids = get_num_astroids(x as isize, y as isize, &map);
            if num_astroids > max_astroids {
                max_astroids = num_astroids;
            }
        }
    }
    println!("Result: {}", max_astroids);
}

#[derive(PartialEq, Eq, Hash)]
struct FoundVec {
    step_x: isize,
    step_y: isize,
}

fn get_num_astroids(x: isize, y: isize, map: &Vec<String>) -> usize {
    let mut found = HashSet::<FoundVec>::new();
    for step_x in 0..=max(x, map[0].len() as isize - x) {
        for step_y in 0..=max(y, map.len() as isize - y) {
            if step_x == 0 && step_y == 0 {
                continue
            }
            check_astroid(x, y, map, step_x as isize, step_y as isize, &mut found);
            check_astroid(x, y, map, step_x as isize, -(step_y as isize), &mut found);
            check_astroid(x, y, map, -(step_x as isize), step_y as isize, &mut found);
            check_astroid(x, y, map, -(step_x as isize), -(step_y as isize), &mut found);
        }
    }
    println!("For {},{}: {}", x, y, found.len());
    found.len()
}

fn check_astroid(x: isize, y: isize, map: &Vec<String>, step_x: isize, step_y: isize, found: &mut HashSet<FoundVec>) {
    if x + step_x < 0 || y + step_y < 0 || x + step_x >= map[0].len() as isize || y + step_y >= map.len() as isize {
        return;
    }
    if map[(y + step_y) as usize].as_bytes()[(x + step_x) as usize] as char == '.' {
        return;
    }
    let target = FoundVec{step_x, step_y};
    for found_vec in found.iter() {
        if is_blocking(found_vec, &target) {
            return;
        }
    }
    //println!("{},{} can see {},{} ({})", x, y, target.step_x + x, target.step_y + y, map[(target.step_y + y) as usize].as_bytes()[(target.step_x + x) as usize] as char);
    found.insert(target);
}

fn is_blocking(blocker: &FoundVec, target: &FoundVec) -> bool {
    let FoundVec {step_x, step_y} = blocker;
    if step_x.signum() != target.step_x.signum() || step_y.signum() != target.step_y.signum() {
        return false;
    }
    let (step_x, step_y) = get_step(step_x.abs() as usize, step_y.abs() as usize);
    let (target_x, target_y) = (target.step_x.abs() as usize, target.step_y.abs() as usize);
    let (mut x, mut y) = (step_x, step_y);
    while x <= target_x && y <= target_y {
        x += step_x;
        y += step_y;
        if x == target_x && y == target_y {
            return true;
        }
    }
    false
}

fn get_step(step_x: usize, step_y: usize) -> (usize, usize) {
    if step_x == 0 {
        return (0,1);
    }
    if step_y == 0 {
        return (1,0);
    }
    let div = gcd(step_x, step_y);
    (step_x / div, step_y / div)
}
