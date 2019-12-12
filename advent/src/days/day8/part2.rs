use crate::utils;
use std::str;

pub fn run() {
    scan(25, 6, &utils::lines_from_file("./src/days/day8/input.txt")[0])
}

fn scan(width: usize, height: usize, input: &str) {
    let layer_size = width * height;
    let layers = input.as_bytes()
        .chunks(layer_size)
        .map(|layer| layer
            .chunks(width)
            .map(str::from_utf8)
            .collect::<Result<Vec<&str>, _>>()
            .unwrap())
        .collect::<Vec<Vec<&str>>>();

        for y in 0..height {
            for x in 0..width {
                if is_black(&layers, x, y) { print!("*"); } else { print!(" "); }
            }
            println!();
        }
}

fn is_black(layers: &Vec<Vec<&str>>, x: usize, y: usize) -> bool {
    for layer in layers {
        let pixel = layer[y].as_bytes()[x];
        match pixel {
            48 => { return true; },
            49 => { return false; },
            50 => { continue; }
            _ => { panic!("Unknown pixel color {} at {},{}", pixel, x, y) }
        }
    }
    panic!("Fully transparent at {},{}", x, y)
}