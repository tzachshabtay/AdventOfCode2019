use crate::utils;
use crate::days::day5::part2::{Program};
use crate::days::day15::part1::{Droid, Point, move_next, reverse};
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    let code = &utils::lines_from_file("./src/days/day15/input.txt")[0];
    let mut solver = Solver::new(code);
    solver.reach_target(1);
    solver.spread_oxygen();
    println!("{}", solver.get_result());
}

struct Solver {
    program: Program,
    droid: Droid,
    target_points: HashMap::<Point, isize>,
    oxygen_points: HashMap::<Point, isize>,
    current_point: Point,
    reached_target: bool,
}

struct Command {
    cmd_type: isize,
    is_reverse: bool,
    value: isize,
    id: isize,
}

impl Command {
    fn new(cmd_type: isize, value: isize, id: isize) -> Command {
        Command{cmd_type, value, id, is_reverse: false}
    }

    fn reverse(cmd_type: isize, value: isize, id: isize) -> Command {
        Command{cmd_type: reverse(cmd_type), value, id, is_reverse: true}
    }
}


#[derive(Debug, Hash, Clone, Copy, Eq, PartialEq)]
struct CommandKey {
    cmd_type: isize,
    id: isize,
}

impl Solver {
    fn new(code: &str) -> Solver {
        let mut target_points = HashMap::new();
        let current_point = Point{x: 0, y: 0};
        target_points.insert(current_point, 0);
        Solver{program: Program::new(code), droid: Droid::new(), target_points, oxygen_points: HashMap::new(), current_point, reached_target: false}
    }

    fn reach_target_helper(&mut self, command: isize, current_value: isize) {
        if self.reached_target {
            return;
        }
        self.current_point = move_next(self.current_point, command);
        let point_last_value = self.target_points.entry(self.current_point).or_default();
        if *point_last_value == 0 || *point_last_value > current_value {
            self.target_points.insert(self.current_point, current_value);
            if self.reach_target(current_value + 1) {
                return;
            }
        }
        if self.reached_target {
            return;
        }
        self.droid.set_command(reverse(command));
        self.program.run(&mut self.droid);
        self.current_point = move_next(self.current_point, reverse(command));
    }

    fn reach_target(&mut self, current_value: isize) -> bool {
        if self.reached_target {
            return true;
        }
        let commands = vec![1,2,3,4];
        for command in commands {
            if self.reached_target {
                return true;
            }
            self.droid.set_command(command);
            let output = self.program.run(&mut self.droid);

            match output {
                0 => {},
                1 => {
                    self.reach_target_helper(command, current_value);
                },
                2 => {
                    self.current_point = move_next(self.current_point, command);
                    self.reached_target = true;
                    println!("reached target at {:?}", self.current_point);
                    return true;
                },
                _ => panic!("unknown output: {}", output),
            }
        }
        false
    }

    fn spread_oxygen(&mut self) {
        println!("spreading oxygen from {:?}", self.current_point);
        let mut stack = VecDeque::new();
        let mut id = 1;
        let mut done_commands = HashSet::new();
        let commands = vec![1,2,3,4];
        for command in &commands {
            &stack.push_back(Command::reverse(*command, 1, id));
            &stack.push_back(Command::new(*command, 1, id));
        }
        while !&stack.is_empty() {
            let command = &stack.pop_back().unwrap();
            if command.is_reverse {
                println!("Reverse ({}): {}, {}", command.id, command.cmd_type, command.value);
                if !done_commands.contains(&CommandKey{cmd_type: reverse(command.cmd_type), id: command.id}) {
                    println!("Skipping reverse");
                    continue;
                }
            } else {
                println!("Command ({}): {}, {}", command.id, command.cmd_type, command.value);
            }
            self.droid.set_command(command.cmd_type);
            let output = self.program.run(&mut self.droid);

            match output {
                0 => { println!("wall at {:?}", move_next(self.current_point, command.cmd_type))},
                1 => {
                    self.current_point = move_next(self.current_point, command.cmd_type);
                    if !command.is_reverse {
                        let point_last_value = self.oxygen_points.entry(self.current_point).or_default();
                        if *point_last_value == 0 || *point_last_value > command.value {
                            println!("{},{} -> {}", self.current_point.x, self.current_point.y, command.value);
                            self.oxygen_points.insert(self.current_point, command.value);
                            id += 1;
                            for cmd in &commands {
                                &stack.push_back(Command::reverse(*cmd, command.value + 1, id));
                                &stack.push_back(Command::new(*cmd, command.value + 1, id));
                            }
                            done_commands.insert(CommandKey{cmd_type: command.cmd_type, id: command.id});
                        } else {
                            println!("Been there already ({},{})", self.current_point.x, self.current_point.y);
                            self.rollback(&command);
                        }
                    } else {
                        println!("Reversed to {},{}", self.current_point.x, self.current_point.y);
                    }
                },
                2 => {
                    println!("back to start");
                    if !command.is_reverse {
                        self.rollback(&command);
                    } else {
                        done_commands.insert(CommandKey{cmd_type: command.cmd_type, id: command.id});
                    }
                },
                _ => panic!("unknown output: {}", output),
            }
        }
    }

    fn rollback(&mut self, command: &Command) {
        self.droid.set_command(reverse(command.cmd_type));
        self.program.run(&mut self.droid);
        self.current_point = move_next(self.current_point, reverse(command.cmd_type));
    }

    fn get_result(self) -> isize {
        *self.oxygen_points.values().max().unwrap()
    }
}
