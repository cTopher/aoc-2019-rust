use std::{fmt, fs};
use std::fmt::{Display, Formatter};
use std::time::SystemTime;

#[derive(Debug, Copy, Clone)]
struct Location {
    x: i32,
    y: i32,
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({};{})", self.x, self.y)
    }
}

impl Location {
    const CENTRAL_PORT: Location = Location { x: 0, y: 0 };

    fn add(&self, input: &str) -> Location {
        let mut chars = input.chars();
        let direction = chars.next().unwrap();
        let tail: String = chars.collect();
        let length: i32 = tail.parse().unwrap();
        match direction {
            'U' => Location { y: self.y + length, ..*self },
            'R' => Location { x: self.x + length, ..*self },
            'D' => Location { y: self.y - length, ..*self },
            'L' => Location { x: self.x - length, ..*self },
            _ => panic!("Could not parse location input: {}", input)
        }
    }
}

struct Wire {
    sections: Vec<Section>
}

enum Section {
    Horizontal { x1: i32, x2: i32, y: i32 },
    Vertical { x: i32, y1: i32, y2: i32 },
}

impl Section {
    fn new(start: Location, end: Location) -> Section {
        if start.x == end.x {
            let x = start.x;
            let (y1, y2) = if start.y <= end.y { (start.y, end.y) } else { (end.y, start.y) };
            Section::Vertical { x, y1, y2 }
        } else if start.y == end.y {
            let (x1, x2) = if start.x <= end.x { (start.x, end.x) } else { (end.x, start.x) };
            let y = start.y;
            Section::Horizontal { x1, x2, y }
        } else {
            panic!("Cannot create section from {} to {}", start, end)
        }
    }

    fn intersection_distance(a: &Section, b: &Section) -> Option<i32> {
        match (a, b) {
            (&Section::Horizontal { x1, x2, y }, &Section::Vertical { x, y1, y2 }) => Section::_intersection_distance(x, x1, x2, y, y1, y2),
            (&Section::Vertical { x, y1, y2 }, &Section::Horizontal { x1, x2, y }) => Section::_intersection_distance(x, x1, x2, y, y1, y2),
            _ => None
        }
    }

    fn _intersection_distance(x: i32, x1: i32, x2: i32, y: i32, y1: i32, y2: i32) -> Option<i32> {
        if x == 0 && y == 0 {
            None
        } else if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
            Some(x.abs() + y.abs())
        } else {
            None
        }
    }
}

impl Wire {
    fn from_string(input: &str) -> Wire {
        let sections = input.split(',')
            .scan(Location::CENTRAL_PORT, |acc, input| {
                let start = *acc;
                *acc = acc.add(input);
                Some(Section::new(start, *acc))
            })
            .collect();
        Wire { sections }
    }

    fn intersection_distance(a: &Wire, b: &Wire) -> Option<i32> {
          a.sections.iter()
            .flat_map(|s1| {
                b.sections.iter()
                    .flat_map(|s2| { Section::intersection_distance(s1, s2) })
                    .min()
            }).min()
    }
}

fn main() {
    let now = SystemTime::now();
    let wires = parse_input();
    let w1 = &wires[0];
    let w2 = &wires[1];
    for _ in 1..1000 {
        Wire::intersection_distance(w1, w2).unwrap();
    }
    println!("Answer 1 = {}", Wire::intersection_distance(w1, w2).unwrap());
    println!("{}", now.elapsed().unwrap().as_millis());
}

fn parse_input() -> Vec<Wire> {
    fs::read_to_string("src/day/3/input.txt").unwrap().lines()
        .map(|line| { Wire::from_string(line) })
        .collect()
}


#[cfg(test)]
mod tests {
    use crate::*;

    #[test]
    fn example_1() {
        let w1 = Wire::from_string("R75,D30,R83,U83,L12,D49,R71,U7,L72");
        let w2 = Wire::from_string("U62,R66,U55,R34,D71,R55,D58,R83");
        assert_eq!(Wire::intersection_distance(&w1, &w2), Some(159));
    }

    #[test]
    fn example_2() {
        let w1 = Wire::from_string("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51");
        let w2 = Wire::from_string("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7");
        assert_eq!(Wire::intersection_distance(&w1, &w2), Some(135));
    }
}