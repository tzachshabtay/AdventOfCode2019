use crate::utils;

const INPUT: isize = 1;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day5/input.txt");
    run_program(lines[0].clone());
}

type Program = Vec<isize>;

enum ParamMode {
    Position,
    Immediate,
}

struct Param {
    mode: ParamMode,
    value: isize,
}

fn new_param(mode: char, value: isize) -> Param {
    match mode {
        '0' => Param{mode: ParamMode::Position, value: value},
        '1' => Param{mode: ParamMode::Immediate, value: value},
        _ => panic!("Unknown param: {}", mode)
    }
}

impl Param {
    fn get(&self, program: &Program) -> isize {
        match self.mode {
            ParamMode::Immediate => self.value,
            ParamMode::Position => program[self.value as usize],
        }
    }
}

trait OpCode {
    fn name(&self) -> &'static str;
    fn length(&self) -> usize;
    fn exec(&self, program: &mut Program, params: Vec<Param>);
}

struct Output{cursor: usize}
struct Input{}
struct Add{}
struct Multiply{}

impl OpCode for Output {
    fn name(&self) -> &'static str { "output" }
    fn length(&self) -> usize { 1 }

    fn exec(&self, program: &mut Program, params: Vec<Param>) {
        println!("At {}: {}", self.cursor, params[0].get(&program));
    }
}

impl OpCode for Input {
    fn name(&self) -> &'static str { "input" }
    fn length(&self) -> usize { 1 }

    fn exec(&self, program: &mut Program, params: Vec<Param>) {
        println!("\tputting {} in {}", INPUT, params[0].value);
        program[params[0].value as usize] = INPUT;
    }
}

impl OpCode for Add {
    fn name(&self) -> &'static str { "add" }
    fn length(&self) -> usize { 3 }

    fn exec(&self, program: &mut Program, params: Vec<Param>) {
        let (op1, op2, location) = (params[0].get(&program), params[1].get(&program), params[2].value);
        println!("\tputting {} + {} in {}", op1, op2, location);
        program[location as usize] = op1 + op2;
    }
}

impl OpCode for Multiply {
    fn name(&self) -> &'static str { "multiply" }
    fn length(&self) -> usize { 3 }

    fn exec(&self, program: &mut Program, params: Vec<Param>) {
        let (op1, op2, location) = (params[0].get(&program), params[1].get(&program), params[2].value);
        println!("\tputting {} * {} in {}", op1, op2, location);
        program[location as usize] = op1 * op2;
    }
}

fn get_params(cursor: usize, program: &Program, num: usize) -> Vec<Param> {
    let instruction = format!("{:0>6}", program[cursor].to_string());
    instruction[(4 - num)..4].chars().rev().enumerate().map(|(i, c)|
        new_param(c, program[cursor + i + 1])).collect()
}

fn run_instruction<T: OpCode>(code: T, cursor: usize, program: &mut Program) -> usize {
    println!("Running instruction {:?} at {} ({})", code.name(), cursor, program[cursor]);
    let params = get_params(cursor, &program, code.length());
    code.exec(program, params);
    cursor + code.length() + 1
}

pub fn run_program(program: String) {
    let tokens = program.split(",").map(|c| c.parse::<isize>().unwrap());
    let program: &mut Program = &mut tokens.collect();
    let mut cursor = 0;
    loop {
        let instruction = format!("{:0>6}", program[cursor].to_string());
        match &instruction[4..=5] {
            "99" => return,
            "01" => cursor = run_instruction(Add{}, cursor, program),
            "02" => cursor = run_instruction(Multiply{}, cursor, program),
            "03" => cursor = run_instruction(Input{}, cursor, program),
            "04" => cursor = run_instruction(Output{cursor}, cursor, program),
            _ => panic!("unknown op {} at {}", program[cursor], cursor),
        }
    }
}