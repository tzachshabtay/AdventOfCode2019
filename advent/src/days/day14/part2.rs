use crate::days::day14::part1::{Solver, parse, Chemical};
use std::collections::{HashMap};

pub fn run(){
    let reactions = parse();
    let target = 1000000000000 as usize;
    let (mut min, mut max) = (0, target);
    while min < max {
        let mid = (min + max) / 2;
        let mut solver = Solver::new(HashMap::new());
        let ore_per_fuel = solver.solve(&Chemical::new("FUEL".to_string(), mid), &reactions);
        if ore_per_fuel == target {
            min = mid;
            max = mid;
        } else if ore_per_fuel < target {
            min = mid + 1;
        } else {
            max = mid - 1;
        }
    }

    let mut solver = Solver::new(HashMap::new());
    let ore_per_fuel = solver.solve(&Chemical::new("FUEL".to_string(), min), &reactions);
    if ore_per_fuel < target {
        println!("{}", min);
    } else {
        println!("{}", min - 1);
    }

}
