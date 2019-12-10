use crate::utils;
use radix_fmt::radix;
use itertools::Itertools;
use crate::days::day5::part2;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day7/input.txt");
    let mut max_output = -1 as isize;
    for i in 0..=2930 {
        let formatted = format!("{:0>5}", radix(i, 5).to_string());
        if !is_valid_config(&formatted) {
            continue;
        }
        let amplifiers: Vec<Amplifier> = formatted.chars().map(|x| Amplifier::new(&lines[0], x.to_string().parse::<isize>().unwrap() + 5)).collect();
        print!("{}->", formatted);
        //print!("{:?} ->", amplifiers);
        let last_output = run_feedback_loop(amplifiers);
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

#[derive(Debug)]
struct Amplifier {
    operator: AmplifierOperator,
    program: part2::Program,
}

impl Amplifier {
    fn new(program: &str, phase: isize) -> Amplifier {
        let program = part2::Program::new(program);
        let operator = AmplifierOperator{phase, read_phase: false, input: 0};
        Amplifier{operator, program}
    }
}

#[derive(Debug)]
struct AmplifierOperator{
    phase: isize,
    input: isize,
    read_phase: bool,
}

impl AmplifierOperator {
    fn set_input(&mut self, input: isize) {
        self.input = input;
    }

    fn get_phase(&mut self) -> isize {
        self.read_phase = true;
        self.phase
    }
}

impl part2::Operator for AmplifierOperator {
    fn next(&mut self) -> isize {
        if self.read_phase { self.input } else { self.get_phase() }
    }
}

fn run_feedback_loop(mut amplifiers: Vec<Amplifier>) -> isize {
    let mut input = 0;
    let mut halted = false;
    while !halted {
        for amplifier in &mut amplifiers {
            amplifier.operator.set_input(input);
            input = amplifier.program.run(&mut amplifier.operator);
            if amplifier.program.halted {
                halted = true;
            }
        }
    }
    input
}
