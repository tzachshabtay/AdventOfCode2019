use crate::utils;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day12/input.txt");
    let mut moons = lines.iter().map(|l| Moon::new(l)).into_iter().collect();
    run_simulation(&mut moons, 1000);
}

fn run_simulation(moons: &mut Vec<Moon>, steps: usize) {
    for _i in 0..steps {
        timestep(moons);
    }
    let energy: isize = moons.iter().map(Moon::get_energy).sum();
    println!("{}", energy);
}

fn timestep(moons: &mut Vec<Moon>) {
    for i in 0..moons.len()-1 {
        for j in i+1..moons.len() {
            {
                let moon2 = moons[j].clone();
                let moon1 = &mut moons[i];
                moon1.update_velocity(&moon2);
            }
            {
                let moon1 = moons[i].clone();
                let moon2 = &mut moons[j];
                moon2.update_velocity(&moon1);
            }
        }
    }
    for moon in moons {
        moon.update_position();
    }
}

#[derive(Clone)]
struct Vec3{
    x: isize,
    y: isize,
    z: isize,
}

impl Vec3{
    fn set_x(&mut self, x: isize) {
        self.x = x;
    }

    fn set_y(&mut self, y: isize) {
        self.y = y;
    }

    fn set_z(&mut self, z: isize) {
        self.z = z;
    }

    fn sum_abs(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    }
}

#[derive(Clone)]
struct Moon{
    velocity: Vec3,
    position: Vec3,
}

impl Moon{
    fn new(line: &str) -> Moon {
        let line = &line.replace("<x=", "").replace("y=", "").replace("z=", "").replace(">", "").replace(" ", "");
        let tokens: Vec<&str> = line.split(",").collect();
        println!("{:?}", tokens);
        Moon{position: Vec3{x: tokens[0].parse::<isize>().unwrap(), y: tokens[1].parse::<isize>().unwrap(), z: tokens[2].parse::<isize>().unwrap()}, velocity: Vec3{x: 0, y: 0, z: 0}}
    }

    fn update_velocity(&mut self, other: &Moon) {
        if self.position.x > other.position.x {
            self.velocity.set_x(self.velocity.x - 1);
        } else if self.position.x < other.position.x {
            self.velocity.set_x(self.velocity.x + 1);
        }

        if self.position.y > other.position.y {
            self.velocity.set_y(self.velocity.y - 1);
        } else if self.position.y < other.position.y {
            self.velocity.set_y(self.velocity.y + 1);
        }

        if self.position.z > other.position.z {
            self.velocity.set_z(self.velocity.z - 1);
        } else if self.position.z < other.position.z {
            self.velocity.set_z(self.velocity.z + 1);
        }
    }

    fn update_position(&mut self) {
        self.position.set_x(self.position.x + self.velocity.x);
        self.position.set_y(self.position.y + self.velocity.y);
        self.position.set_z(self.position.z + self.velocity.z);
    }

    fn get_energy(&self) -> isize {
        self.position.sum_abs() * self.velocity.sum_abs()
    }
}
