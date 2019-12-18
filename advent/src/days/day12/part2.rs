use crate::utils;
use crate::days::day12::part1::{Moon, timestep};
use std::collections::HashMap;
use num::integer::gcd;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day12/input.txt");
    let mut moons = lines.iter().map(|l| Moon::new(l)).into_iter().collect();
    run_simulation(&mut moons);
}

fn run_simulation(moons: &mut Vec<Moon>) {
    let mut axes = vec![AxisInfo::new(); 3];
    let mut steps = 0;
    while axes.iter().any(|m| m.result == 0) {
        for (i, info) in axes.iter_mut().enumerate() {
            if info.result != 0 {
                continue;
            }
            info.record_state(steps, &get_axis(&moons, i));
        }
        timestep(moons);
        steps += 1;
    }
    let val = lcm(axes[0].result, lcm(axes[1].result, axes[2].result));
    println!("{}", val);
}

fn get_axis(moons: &Vec<Moon>, i: usize) -> AxisResult {
    match i {
        0 => AxisResult{pos: moons.iter().map(|m| m.position.x).collect(), vel: moons.iter().map(|m| m.velocity.x).collect()},
        1 => AxisResult{pos: moons.iter().map(|m| m.position.y).collect(), vel: moons.iter().map(|m| m.velocity.y).collect()},
        2 => AxisResult{pos: moons.iter().map(|m| m.position.z).collect(), vel: moons.iter().map(|m| m.velocity.z).collect()},
        _ => panic!("{}", i)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}

#[derive(Eq, PartialEq, Hash, Clone, Debug)]
struct AxisResult {
    pos: Vec::<isize>,
    vel: Vec::<isize>,
}

#[derive(Clone)]
struct AxisInfo {
    states: HashMap::<AxisResult, usize>,
    result: usize,
}

impl AxisInfo{
    fn new() -> AxisInfo {
        AxisInfo{states: HashMap::<AxisResult, usize>::new(), result: 0}
    }

    fn record_state(&mut self, steps: usize, state: &AxisResult) {
        if self.states.contains_key(state) {
            println!("Found after {} steps (from {}): {:?}", steps, self.states[state], state);
            self.result = steps - self.states[state];
        } else {
            self.states.insert(state.clone(), steps);
        }
    }
}