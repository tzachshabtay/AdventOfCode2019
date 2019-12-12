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

#[derive(Debug)]
enum ParamMode {
    Position,
    Immediate,
    Relative,
}

struct Param {
    mode: ParamMode,
    value: isize,
    relative_base: isize,
}

fn new_param(mode: char, value: isize, relative_base: isize) -> Param {
    let p = match mode {
        '0' => Param{mode: ParamMode::Position, value, relative_base},
        '1' => Param{mode: ParamMode::Immediate, value, relative_base},
        '2' => Param{mode: ParamMode::Relative, value, relative_base},
        _ => panic!("Unknown param: {}", mode)
    };
    //println!("Param: {:?}, {}, {}", p.mode, p.value, p.relative_base);
    p
}

impl Param {
    fn get(&self, program: &ProgramCode) -> isize {
        match self.mode {
            ParamMode::Immediate => self.value,
            ParamMode::Position => program[self.value as usize],
            ParamMode::Relative => program[(self.value as isize + self.relative_base as isize) as usize]
        }
    }

    fn set(&self, program: &mut ProgramCode, value: isize) {
        match self.mode {
            ParamMode::Position => { program[self.value as usize] = value; },
            ParamMode::Relative => { program[(self.value as isize + self.relative_base as isize) as usize] = value;},
            _ => panic!("Did not expect immediate mode for output")
        }
    }
}

trait OpCode {
    fn name(&self) -> &'static str;
    fn length(&self) -> usize;
    fn move_cursor(&self, cursor: usize) -> usize { cursor + self.length() + 1 }
    fn get_relative_base(&self, relative_base: isize) -> isize { relative_base }
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
#[derive(Default)]
struct RelativeOffset{offset: isize}

impl OpCode for RelativeOffset {
    fn name(&self) -> &'static str { "RelativeOffset" }
    fn length(&self) -> usize { 1 }
    fn get_relative_base(&self, relative_base: isize) -> isize { relative_base + self.offset }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        self.offset = params[0].get(&program);
        //println!("Set relative offset to {}", self.offset);
    }
}

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
            params[2].set(program, 1);
        } else {
            params[2].set(program, 0);
        }
        //println!("Put {} in {}", program[params[2].value as usize], params[2].value as usize);
    }
}

impl OpCode for Equals {
    fn name(&self) -> &'static str { "LessThan" }
    fn length(&self) -> usize { 3 }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        if params[0].get(&program) == params[1].get(&program) {
            params[2].set(program, 1);
        } else {
            params[2].set(program, 0);
        }
        //println!("Put {} in {}", program[params[2].value as usize], params[2].value as usize);
    }
}

impl OpCode for Output {
    fn name(&self) -> &'static str { "output" }
    fn length(&self) -> usize { 1 }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        println!("Output: {}", params[0].get(&program));
        self.output = params[0].get(&program);
    }
}

impl OpCode for Input {
    fn name(&self) -> &'static str { "input" }
    fn length(&self) -> usize { 1 }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        params[0].set(program, self.input);
    }
}

impl OpCode for Add {
    fn name(&self) -> &'static str { "add" }
    fn length(&self) -> usize { 3 }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        let (op1, op2) = (params[0].get(&program), params[1].get(&program));
        //println!("\tputting {} + {} in {}", op1, op2, location);
        params[2].set(program, op1 + op2);
    }
}

impl OpCode for Multiply {
    fn name(&self) -> &'static str { "multiply" }
    fn length(&self) -> usize { 3 }

    fn exec(&mut self, program: &mut ProgramCode, params: Vec<Param>) {
        let (op1, op2) = (params[0].get(&program), params[1].get(&program));
        //println!("\tputting {} * {} in {}", op1, op2, location);
        params[2].set(program, op1 * op2);
    }
}

#[derive(Debug)]
pub struct Program {
    code: ProgramCode,
    pub halted: bool,
    result: isize,
    cursor: usize,
    relative_base: isize,
}

impl Program {
    pub fn new(program: &str) -> Program {
        let tokens = program.split(",").map(|c| c.parse::<isize>().unwrap());
        let mut program: ProgramCode = tokens.collect();
        program.extend(vec![0; 10000]);
        Program{code: program, halted: false, result: 0, cursor: 0, relative_base: 0}
    }

    fn get_params(&self, num: usize) -> Vec<Param> {
        let instruction = format!("{:0>6}", self.code[self.cursor].to_string());
        instruction[(4 - num)..4].chars().rev().enumerate().map(|(i, c)|
            new_param(c, self.code[self.cursor + i + 1], self.relative_base)).collect()
    }

    fn run_instruction<T: OpCode>(&mut self, code: &mut T) -> usize {
        //println!("Running instruction {:?} at {} ({})", code.name(), self.cursor, self.code[self.cursor]);
        let params = self.get_params(code.length());
        code.exec(&mut self.code, params);
        self.relative_base = code.get_relative_base(self.relative_base);
        code.move_cursor(self.cursor)
    }

    pub fn run<T: Operator>(&mut self, operator: &mut T) -> isize {
        loop {
            let instruction = format!("{:0>6}", self.code[self.cursor].to_string());
            match &instruction[4..=5] {
                "99" => {
                    self.halted = true;
                    return self.result;
                },
                "01" => self.cursor = self.run_instruction(&mut Add{}),
                "02" => self.cursor = self.run_instruction(&mut Multiply{}),
                "03" => self.cursor = self.run_instruction(&mut Input{input: operator.next()}),
                "04" => {
                    let mut output = Output{output: 0};
                    self.cursor = self.run_instruction(&mut output);
                    self.result = output.output;
                    return self.result;
                },
                "05" => self.cursor = self.run_instruction(&mut JumpIfTrue{..Default::default()}),
                "06" => self.cursor = self.run_instruction(&mut JumpIfFalse{..Default::default()}),
                "07" => self.cursor = self.run_instruction(&mut LessThan{}),
                "08" => self.cursor = self.run_instruction(&mut Equals{}),
                "09" => self.cursor = self.run_instruction(&mut RelativeOffset{..Default::default()}),
                _ => panic!("unknown op {} at {}", self.code[self.cursor], self.cursor),
            }
        }
    }
}