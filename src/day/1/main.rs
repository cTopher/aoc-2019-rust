use std::{
    fs::File,
    io::{BufRead, BufReader},
    path::Path,
};

fn main() {
    println!("Answer 1 = {}", part_1());
    println!("Answer 2 = {}", part_2());
}

fn part_1() -> u32 {
    parse_lines("src/day/1/input.txt.txt")
        .map(calculate_direct_fuel)
        .sum()
}

fn part_2() -> u32 {
    parse_lines("src/day/1/input.txt.txt")
        .map(calculate_total_fuel)
        .sum()
}

fn calculate_direct_fuel(mass: u32) -> u32 {
    let x = mass / 3;
    if x > 2 { x - 2 } else { 0 }
}

fn calculate_total_fuel(mass: u32) -> u32 {
    let mut mass = mass;
    let mut total_fuel: u32 = 0;
    loop {
        let additional_fuel = calculate_direct_fuel(mass);
        if additional_fuel == 0 {
            return total_fuel;
        } else {
            total_fuel += additional_fuel;
            mass = additional_fuel;
        }
    }
}

//TODO: move to utils module
fn parse_lines<P: AsRef<Path>>(path: P) -> impl Iterator<Item=u32> {
    let file = File::open(path).unwrap();
    BufReader::new(file).lines()
        .map(Result::unwrap)
        .map(|line| line.parse::<u32>().unwrap())
}

#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn direct_fuel() {
        assert_eq!(calculate_direct_fuel(12), 2);
        assert_eq!(calculate_direct_fuel(14), 2);
        assert_eq!(calculate_direct_fuel(1969), 654);
        assert_eq!(calculate_direct_fuel(100756), 33583);
    }

    #[test]
    fn answer_1() {
        assert_eq!(part_1(), 3198599);
    }

    #[test]
    fn total_fuel() {
        assert_eq!(calculate_total_fuel(14), 2);
        assert_eq!(calculate_total_fuel(1969), 966);
        assert_eq!(calculate_total_fuel(100756), 50346);
    }

    #[test]
    fn answer_2() {
        assert_eq!(part_2(), 4795042);
    }

}