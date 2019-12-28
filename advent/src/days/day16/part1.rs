use crate::utils;

pub fn run() {
    let input = &utils::lines_from_file("./src/days/day16/input.txt")[0];
    let result = run_phases(input, &vec![0,1,0,-1], 100);
    println!("{:?}", result);
}

fn run_phases(input: &str, pattern: &Vec<isize>, phases: usize) -> String {
    let mut result = input.to_string();
    for _ in 0..phases {
        result = run_phase(&result, pattern);
    }
    result
}

fn run_phase(input: &str, pattern: &Vec<isize>) -> String {
    let mut result = String::with_capacity(input.len());
    for index in 1..=input.len() {
        let c = run_index(input, pattern, index);
        result.push_str(&c.to_string());
    }
    result
}

fn run_index(input: &str, pattern: &Vec<isize>, index: usize) -> u32 {
    let pattern = scale_pattern(pattern, index);
    let mut index = 1;
    let mut result = 0;
    for c in input.chars() {
        result += c.to_digit(10).unwrap() as i32 * pattern[index] as i32;
        index = (index + 1) % pattern.len();
    }
    result.to_string().chars().last().unwrap().to_digit(10).unwrap()
}

fn scale_pattern(pattern: &Vec<isize>, index: usize) -> Vec<isize> {
    let mut result = Vec::with_capacity(pattern.len() * index);
    for c in pattern {
        for _ in 0..index {
            result.push(*c);
        }
    }
    result
}