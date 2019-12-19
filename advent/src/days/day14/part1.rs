use crate::utils;
use std::collections::HashMap;

pub fn run() {
    let reactions = parse();
    let mut solver = Solver::new(reactions);
    let result = solver.solve(Chemical::new("1 FUEL"));
    println!("{}", result);
}

fn parse() -> HashMap::<String, Reaction> {
    let lines = utils::lines_from_file("./src/days/day14/input.txt");
    let mut reactions = HashMap::new();
    for line in lines {
        let reaction = Reaction::new(&line);
        reactions.insert((&reaction.output.name).to_string(), reaction);
    }
    reactions
}

struct Solver {
    reactions: HashMap::<String, Reaction>,
    inventory: HashMap::<String, usize>,
}

const ORE: &'static str = "ORE";

impl Solver{
    fn new(reactions: HashMap::<String, Reaction>) -> Solver {
        Solver{reactions, inventory: HashMap::new()}
    }

    fn solve(&mut self, needed: Chemical) -> usize {
        if needed.name == ORE {
            return needed.qty;
        }
        let mut remaining = 0;
        if self.inventory.contains_key(&needed.name) {
            remaining = self.inventory[&needed.name];
        }
        if remaining > needed.qty {
            self.inventory.insert(needed.name.to_string(), remaining - needed.qty);
            println!("{} {}- had it!", &needed.name, needed.qty);
            return 0;
        }
        if remaining > 0 {
            self.inventory.remove(&needed.name);
        }
        let reaction = self.reactions[&needed.name].clone();
        let mut sum = 0;
        let mut needed_qty = (needed.qty - remaining) as isize;
        while needed_qty > 0 {
            for input in &reaction.inputs {
                sum += self.solve(input.clone());
            }
            needed_qty -= reaction.output.qty as isize;
        }
        if needed_qty < 0 {
            self.inventory.insert(needed.name.to_string(), -needed_qty as usize);
        }
        println!("{} {}- {}! (reused {})", &needed.name, needed.qty, sum, remaining);
        sum
    }
}

#[derive(Clone, Debug)]
struct Chemical {
    name: String,
    qty: usize,
}

impl Chemical {
    fn new(line: &str) -> Chemical {
        let tokens: Vec<&str> = line.trim().split(" ").collect();
        Chemical{name: tokens[1].trim().to_string(), qty: tokens[0].trim().parse::<usize>().unwrap()}
    }
}

#[derive(Clone, Debug)]
struct Reaction {
    inputs: Vec<Chemical>,
    output: Chemical,
}

impl Reaction {
    fn new(line: &str) -> Reaction {
        let tokens : Vec<&str> = line.split("=>").collect();
        let (inputs, output) = (tokens[0].trim(), tokens[1].trim());
        Reaction{inputs: inputs.split(",").map(|i| Chemical::new(i)).collect(), output: Chemical::new(output)}
    }
}