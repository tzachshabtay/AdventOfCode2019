use crate::utils;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day1/input.txt");
    let mut ctr = 0;
    for line in lines {
        let mass = line.parse::<i32>().unwrap();
        let res = get_fuel(mass);
        ctr = ctr + res
    }

    println!("{}", ctr);
}

fn get_fuel(mass: i32) -> i32 {
    let fuel = mass / 3 - 2;
    if fuel <= 0 {
        return 0;
    }
    return fuel + get_fuel(fuel);
}