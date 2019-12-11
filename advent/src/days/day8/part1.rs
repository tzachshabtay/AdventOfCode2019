use crate::utils;
use std::str;

pub fn run() {
    scan(25, 6, &utils::lines_from_file("./src/days/day8/input.txt")[0])
}

fn scan(width: usize, height: usize, input: &str) {
    let layer_size = width * height;
    let layers = input.as_bytes()
        .chunks(layer_size)
        .map(str::from_utf8)
        .collect::<Result<Vec<&str>, _>>()
        .unwrap();

    let min_layer = *(layers.iter().min_by_key(|layer| (**layer).matches("0").count()).unwrap());
    println!("{:?}", min_layer.matches("1").count() * min_layer.matches("2").count());
}