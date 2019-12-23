use crate::utils;
use std::collections::HashMap;
use std::cmp;

pub fn run() {
    let reactions = parse();
    let mut solver = Solver::new(HashMap::new());
    let result = solver.solve(&Chemical::new("FUEL".to_string(), 1), &reactions);
    println!("{}", result);
}

pub fn parse() -> HashMap::<String, Reaction> {
    let lines = utils::lines_from_file("./src/days/day14/input.txt");
    let mut reactions = HashMap::new();
    for line in lines {
        let reaction = Reaction::new(&line);
        reactions.insert((&reaction.output.name).to_string(), reaction);
    }
    reactions
}

pub struct Solver {
    pub inventory: HashMap::<String, usize>,
}

const ORE: &'static str = "ORE";

impl Solver{
    pub fn new(inventory: HashMap::<String, usize>) -> Solver {
        Solver{inventory}
    }

    pub fn solve(&mut self, needed: &Chemical, reactions: &HashMap::<String, Reaction>) -> usize {
        if needed.name == ORE {
            return needed.qty;
        }
        let (mut in_inventory, mut needed_qty) = (0, needed.qty);
        if self.inventory.contains_key(&needed.name) {
            in_inventory = self.inventory[&needed.name];
        }
        if in_inventory > 0 {
            let used = cmp::min(in_inventory, needed_qty);
            self.inventory.insert(needed.name.to_string(), in_inventory - used);
            needed_qty -= used;
        }
        if needed_qty == 0 {
            return 0;
        }
        let reaction = &reactions[&needed.name];
        let mut sum = 0;
        let mut factor = 1;
        let out_qty = reaction.output.qty;
        if needed_qty > out_qty {
            if needed_qty % out_qty == 0 {
                factor = needed_qty / out_qty;
            } else {
                factor = (needed_qty / out_qty) + 1;
            }
        }
        self.inventory.insert(needed.name.to_string(), (out_qty * factor - needed_qty) as usize);
        for input in &reaction.inputs {
            sum += self.solve(&Chemical::new(input.name.to_string(), input.qty * factor), reactions);
        }
        /*while needed_qty > 0 {
            for input in &reaction.inputs {
                sum += self.solve(input, reactions);
            }
            needed_qty -= reaction.output.qty as isize;
        }
        if needed_qty < 0 {
            self.inventory.insert(needed.name.to_string(), -needed_qty as usize);
        }*/
        //println!("{} {}- {}! (reused {})", &needed.name, needed.qty, sum, remaining);
        sum
    }
}

#[derive(Clone, Debug)]
pub struct Chemical {
    name: String,
    qty: usize,
}

impl Chemical {
    pub fn parse(line: &str) -> Chemical {
        let tokens: Vec<&str> = line.trim().split(" ").collect();
        Chemical{name: tokens[1].trim().to_string(), qty: tokens[0].trim().parse::<usize>().unwrap()}
    }

    pub fn new(name: String, qty: usize) -> Chemical {
        Chemical{name, qty}
    }
}

#[derive(Clone, Debug)]
pub struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

impl Reaction {
    fn new(line: &str) -> Reaction {
        let tokens : Vec<&str> = line.split("=>").collect();
        let (inputs, output) = (tokens[0].trim(), tokens[1].trim());
        Reaction{inputs: inputs.split(",").map(|i| Chemical::parse(i)).collect(), output: Chemical::parse(output)}
    }
}