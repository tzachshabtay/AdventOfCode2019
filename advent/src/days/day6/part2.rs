use crate::utils;
use std::collections::HashMap;

#[derive(Debug)]
struct Object {
    visited: bool,
    connections: Vec<String>,
}

impl Object {
    fn add_connection(&mut self, name: String) {
        self.connections.push(name);
    }

    fn set_visited(&mut self) {
        self.visited = true;
    }
}

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day6/input.txt");
    let mut objects = HashMap::new();
    let mut to_process: Vec<String> = vec![];
    let mut add_connections = vec![];
    let mut target = String::from("");
    for line in lines {
        let tokens: Vec<&str> = line.split(')').collect();
        let name = tokens[1];
        let parent = tokens[0];
        objects.entry(String::from(name)).or_insert(Object{visited: false, connections: vec![]});
        objects.entry(String::from(parent)).or_insert(Object{visited: false, connections: vec![]});

        if name == "SAN" {
            to_process.push(String::from(parent));
        } else if name == "YOU" {
            target = String::from(parent);
        }
        add_connections.push((String::from(parent), String::from(name)));
    }
    for (parent, name) in &add_connections {
        objects.get_mut(parent).unwrap().add_connection(String::from(name));
    }
    for (parent, name) in add_connections {
        objects.get_mut(&name).unwrap().add_connection(parent);
    }

    println!("{:?}", objects);

    let mut num_orbits = 0;
    while to_process.len() > 0 {
        let mut next_cycle = vec![];
        for name in to_process {
            if name == target {
                println!("{}", num_orbits);
                return;
            }
            let obj = objects.get_mut(&name).unwrap();
            if obj.visited {
                continue;
            }
            println!("Processing {} with num = {}", name, num_orbits);
            obj.set_visited();
            next_cycle.append(&mut obj.connections);
        }
        num_orbits += 1;
        to_process = next_cycle;
    }
}
