use crate::utils;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day2/input.txt");
    for line in lines {
        let res = run_program(line, 12, 2);
        println!("{:?}", res);
    }
}

pub fn run_program(program: String, noun: usize, verb: usize) -> usize {
    let tokens: Vec<&str> = program.split(",").collect();
    let mut codes: Vec<usize> = vec!(0; tokens.len());
    for (index, token) in tokens.iter().enumerate() {
        codes[index] = token.parse::<usize>().unwrap();
    }
    codes[1] = noun;
    codes[2] = verb;
    let mut index = 0;
    loop {
        match codes[index] {
            1 => {
                let op1_loc = codes[index + 1];
                let op2_loc = codes[index + 2];
                let location = codes[index + 3];
                codes[location] = codes[op1_loc] + codes[op2_loc];
                index += 4;
            },
            2 => {
                let op1_loc = codes[index + 1];
                let op2_loc = codes[index + 2];
                let location = codes[index + 3];
                codes[location] = codes[op1_loc] * codes[op2_loc];
                index += 4;
            },
            99 => {
                return codes[0];
            },
            _ => panic!("unknown op"),
        }
    }
}