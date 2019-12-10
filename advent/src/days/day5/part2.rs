use crate::utils;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day5/input.txt");
    run_program(&lines[0], FromListProvider{ inputs: vec![5], cursor: 0});
}

pub fn run_program<T: Operator>(code: &str, mut operator: T) -> isize {
    let mut program = Program::new(code);
    let mut output = 0;
    while !program.halted {
        output = program.run(&mut operator);
    }
    output
}

pub trait Operator {
    fn next(&mut self) -> isize;
}

pub struct FromListProvider {
    inputs: Vec<isize>,
    cursor: usize,
}

impl FromListProvider {
    pub fn new(inputs: Vec<isize>) -> FromListProvider {
        FromListProvider{inputs, cursor: 0}
    }
}

impl Operator for FromListProvider {
    fn next(&mut self) -> isize {
        let result = self.inputs[self.cursor];
        self.cursor += 1;
        return result;
    }
}

type ProgramCode = Vec<isize>;

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
    fn get(&self, program: &ProgramCode) -> isize {
        match self.mode {
            ParamMode::Immediate => self.value,
            ParamMode::Position => program[self.value as usize],
        }
    }
}

trait OpCode {
    fn name(&self) -> &'static str;
    fn length(&self) -> usize;
    fn move_cursor(&self, cursor: usize) -> usize { cursor + self.length() + 1 }
    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>);
}

struct Output{output: isize}
struct Input{input: isize}
struct Add{}
struct Multiply{}
#[derive(Default)]
struct JumpIfTrue{override_cursor: Option<usize>}
#[derive(Default)]
struct JumpIfFalse{override_cursor: Option<usize>}
struct LessThan{}
struct Equals{}

impl OpCode for JumpIfTrue {
    fn name(&self) -> &'static str { "JumpIfTrue" }
    fn length(&self) -> usize { 2 }
    fn move_cursor(&self, cursor: usize) -> usize {
        match self.override_cursor {
            None => cursor + self.length() + 1,
            Some(new_cursor) => new_cursor,
        }
    }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        if params[0].get(&program) != 0 {
            let val = params[1].get(&program);
            //println!("\tcursor jumping to {}", val);
            self.override_cursor = Some(val as usize);
        }
    }
}

impl OpCode for JumpIfFalse {
    fn name(&self) -> &'static str { "JumpIfFalse" }
    fn length(&self) -> usize { 2 }
    fn move_cursor(&self, cursor: usize) -> usize {
        match self.override_cursor {
            None => cursor + self.length() + 1,
            Some(new_cursor) => new_cursor,
        }
    }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        if params[0].get(&program) == 0 {
            let val = params[1].get(&program);
            //println!("\tcursor jumping to {}", val);
            self.override_cursor = Some(val as usize);
        }
    }
}

impl OpCode for LessThan {
    fn name(&self) -> &'static str { "LessThan" }
    fn length(&self) -> usize { 3 }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        if params[0].get(&program) < params[1].get(&program) {
            program[params[2].value as usize] = 1;
        } else {
            program[params[2].value as usize] = 0;
        }
        //println!("Put {} in {}", program[params[2].value as usize], params[2].value as usize);
    }
}

impl OpCode for Equals {
    fn name(&self) -> &'static str { "LessThan" }
    fn length(&self) -> usize { 3 }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        if params[0].get(&program) == params[1].get(&program) {
            program[params[2].value as usize] = 1;
        } else {
            program[params[2].value as usize] = 0;
        }
        //println!("Put {} in {}", program[params[2].value as usize], params[2].value as usize);
    }
}

impl OpCode for Output {
    fn name(&self) -> &'static str { "output" }
    fn length(&self) -> usize { 1 }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        //println!("At {}: {}", self.cursor, params[0].get(&program));
        self.output = params[0].get(&program);
    }
}

impl OpCode for Input {
    fn name(&self) -> &'static str { "input" }
    fn length(&self) -> usize { 1 }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        //println!("\tputting {} in {}", self.input, params[0].value);
        program[params[0].value as usize] = self.input;
    }
}

impl OpCode for Add {
    fn name(&self) -> &'static str { "add" }
    fn length(&self) -> usize { 3 }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        let (op1, op2, location) = (params[0].get(&program), params[1].get(&program), params[2].value);
        //println!("\tputting {} + {} in {}", op1, op2, location);
        program[location as usize] = op1 + op2;
    }
}

impl OpCode for Multiply {
    fn name(&self) -> &'static str { "multiply" }
    fn length(&self) -> usize { 3 }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        let (op1, op2, location) = (params[0].get(&program), params[1].get(&program), params[2].value);
        //println!("\tputting {} * {} in {}", op1, op2, location);
        program[location as usize] = op1 * op2;
    }
}

fn get_params(cursor: usize, program: &ProgramCode, num: usize) -> Vec<Param> {
    let instruction = format!("{:0>6}", program[cursor].to_string());
    instruction[(4 - num)..4].chars().rev().enumerate().map(|(i, c)|
        new_param(c, program[cursor + i + 1])).collect()
}

fn run_instruction<T: OpCode>(code: &mut T, cursor: usize, program: &mut ProgramCode) -> usize {
    //println!("Running instruction {:?} at {} ({})", code.name(), cursor, program[cursor]);
    let params = get_params(cursor, &program, code.length());
    code.exec(program, params);
    code.move_cursor(cursor)
}

#[derive(Debug)]
pub struct Program {
    code: ProgramCode,
    pub halted: bool,
    result: isize,
    cursor: usize,
}

impl Program {
    pub fn new(program: &str) -> Program {
        let tokens = program.split(",").map(|c| c.parse::<isize>().unwrap());
        let program: ProgramCode = tokens.collect();
        Program{code: program, halted: false, result: 0, cursor: 0}
    }

    pub fn run<T: Operator>(&mut self, operator: &mut T) -> isize {
        loop {
            let instruction = format!("{:0>6}", self.code[self.cursor].to_string());
            match &instruction[4..=5] {
                "99" => {
                    self.halted = true;
                    return self.result;
                },
                "01" => self.cursor = run_instruction(&mut Add{}, self.cursor, &mut self.code),
                "02" => self.cursor = run_instruction(&mut Multiply{}, self.cursor, &mut self.code),
                "03" => self.cursor = run_instruction(&mut Input{input: operator.next()}, self.cursor, &mut self.code),
                "04" => {
                    let mut output = Output{output: 0};
                    self.cursor = run_instruction(&mut output, self.cursor, &mut self.code);
                    self.result = output.output;
                    return self.result;
                },
                "05" => self.cursor = run_instruction(&mut JumpIfTrue{..Default::default()}, self.cursor, &mut self.code),
                "06" => self.cursor = run_instruction(&mut JumpIfFalse{..Default::default()}, self.cursor, &mut self.code),
                "07" => self.cursor = run_instruction(&mut LessThan{}, self.cursor, &mut self.code),
                "08" => self.cursor = run_instruction(&mut Equals{}, self.cursor, &mut self.code),
                _ => panic!("unknown op {} at {}", self.code[self.cursor], self.cursor),
            }
        }
    }
}