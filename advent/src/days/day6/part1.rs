use crate::utils;
use std::collections::HashMap;

struct Object {
    num_orbits: usize,
    orbiting: Vec<String>,
}

impl Object {
    fn add_orbiting(&mut self, name: String) {
        self.orbiting.push(name);
    }

    fn set_orbits(&mut self, num: usize) {
        self.num_orbits = num;
    }
}

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day6/input.txt");
    let mut objects = HashMap::new();
    let mut to_process: Vec<String> = vec![];
    let mut add_orbiters = vec![];
    for line in lines {
        let tokens: Vec<&str> = line.split(')').collect();
        let name = tokens[1];
        let parent = tokens[0];
        let obj = Object{num_orbits: 0, orbiting: vec![]};
        objects.insert(String::from(name), obj);

        if parent == "COM" {
            to_process.push(String::from(name));
            continue;
        }
        add_orbiters.push((String::from(parent), String::from(name)));
    }
    for (parent, name) in add_orbiters {
        objects.get_mut(&parent).unwrap().add_orbiting(name);
    }

    let mut num_orbits = 1;
    while to_process.len() > 0 {
        let mut next_cycle = vec![];
        for name in to_process {
            let obj = objects.get_mut(&name).unwrap();
            obj.set_orbits(num_orbits);
            next_cycle.append(&mut obj.orbiting);
        }
        num_orbits += 1;
        to_process = next_cycle;
    }

    println!("{}", objects.values().map(|o| o.num_orbits).sum::<usize>());
}
