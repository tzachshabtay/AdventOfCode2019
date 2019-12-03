use crate::utils;

pub fn run() {
    let lines = utils::lines_from_file("./src/days/day1/input.txt");
    let mut ctr = 0;
    for line in lines {
        let fuel = line.parse::<i32>().unwrap();
        let res = fuel / 3 - 2;
        ctr = ctr + res
    }

    println!("{}", ctr);
}