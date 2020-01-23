use crate::utils;

pub fn run() {
    let input = &utils::lines_from_file("./src/days/day16/input.txt")[0];
    let offset: usize = input[..7].parse().unwrap();
    let input = &(input.repeat(10000))[offset..];
    let mut input: Vec<_> = input.chars().map(|c| c.to_digit(10).unwrap()).collect();
    run_phases(&mut input, 100);
    println!("{:?}", &input[..8]);
}

fn run_phases(input: &mut Vec<u32>, phases: usize) {
    for _ in 0..phases {
        run_phase(input);
    }
}

fn run_phase(input: &mut Vec<u32>) {
    let mut result = 0;
    for index in (0..input.len()).rev() {
        let digit = input[index] as u64;
        result = (result + digit) % 10;
        input[index] = result as u32;
    }
}