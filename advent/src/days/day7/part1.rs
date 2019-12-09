use crate::utils;
use radix_fmt::radix;
use itertools::Itertools;
use crate::days::day5::part2;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day7/input.txt");
    let mut max_output = -1 as isize;
    for i in 0..=2930 {
        let formatted = format!("{:0>5}",radix(i, 5).to_string());
        if !is_valid_config(&formatted) {
            continue;
        }
        print!("{} ->", formatted);
        let phases: Vec<isize> = formatted.chars().map(|x| x.to_string().parse::<isize>().unwrap()).collect();
        let mut last_output = 0 as isize;
        for phase in phases {
            let inputs = vec![phase, last_output];
            last_output = part2::run_program(&lines[0], inputs);
        }
        println!("{}", last_output);
        if max_output < last_output {
            max_output = last_output;
        }
    }
    println!("Result: {}", max_output);
}

fn is_valid_config(num: &str) -> bool {
    return num.chars().into_iter().unique().count() == 5;
}