use std::{fmt, fs};
use std::fmt::{Display, Formatter};
use Direction::{Horizontal, Vertical};

#[derive(Debug, Copy, Clone, PartialEq)]
struct Location {
    x: i32,
    y: i32,
}

#[derive(Debug, Copy, Clone, PartialEq)]
struct Range {
    min: i32,
    max: i32,
}

impl Range {
    fn new(a: i32, b: i32) -> Range {
        if a <= b {
            Range { min: a, max: b }
        } else { ;.l/
            Range { min: b, max: a }
        }
    }
}

impl Display for Location {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "({};{})", self.x, self.y)
    }
}

impl Location {
    const CENTRAL_PORT: Location = Location { x: 0, y: 0 };
}

struct Wire {
    sections: Vec<Section>
}

#[derive(PartialEq, Debug)]
enum Direction {
    Horizontal,
    Vertical,
}

#[derive(PartialEq, Debug)]
struct Section {
    start: Location,
    direction: Direction,
    size: i32,
    steps: i32,
    range: Range,
}

impl Section {
    fn new(start: Location, steps: i32, input: &str) -> Section {
        let (direction, size) = Section::parse_input(input);
        let var = match direction {
            Horizontal => start.x,
            Vertical => start.y
        };
        let range = Range::new(var, var + size);
        Section { start, direction, size, steps, range }
    }

    fn end(&self) -> Location {
        match self.direction {
            Horizontal => Location { x: self.start.x + self.size, ..self.start },
            Vertical => Location { y: self.start.y + self.size, ..self.start }
        }
    }

    fn parse_input(input: &str) -> (Direction, i32) {
        let mut chars = input.chars();
        let d = chars.next().unwrap();
        let tail: String = chars.collect();
        let length: i32 = tail.parse().unwrap();
        match d {
            'U' => (Vertical, length),
            'R' => (Horizontal, length),
            'D' => (Vertical, -length),
            'L' => (Horizontal, -length),
            _ => panic!("Could not parse location input: {}", input)
        }
    }

    fn intersection(a: &Section, b: &Section) -> Option<Location> {
        let (horizontal, vertical) = match (&a.direction, &b.direction) {
            (Horizontal, Vertical) => (a, b),
            (Vertical, Horizontal) => (b, a),
            _ => return None
        };
        let x = vertical.start.x;
        let y = horizontal.start.y;
        if x == 0 && y == 0 { return None; }
        let Range { min: x1, max: x2 } = horizontal.range;
        let Range { min: y1, max: y2 } = vertical.range;
        if x1 <= x && x <= x2 && y1 <= y && y <= y2 {
            Some(Location { x, y })
        } else {
            None
        }
    }

    fn intersection_distance(a: &Section, b: &Section) -> Option<i32> {
        Section::intersection(a, b).map(|Location { x, y }| {
            x.abs() + y.abs()
        })
    }

    fn intersection_steps(a: &Section, b: &Section) -> Option<i32> {
        Section::intersection(a, b).map(|_| {
            let dx = (a.start.x - b.start.x).abs();
            let dy = (a.start.y - b.start.y).abs();
            a.steps + b.steps + dx + dy
        })
    }
}

impl Wire {
    fn from_string(input: &str) -> Wire {
        let sections = input.split(',')
            .scan((Location::CENTRAL_PORT, 0), |(location, steps), input| {
                let new = Section::new(*location, *steps, input);
                *location = new.end();
                *steps += new.size.abs();
                Some(new)
            })
            .collect();
        Wire { sections }
    }

    fn intersection_distance(a: &Wire, b: &Wire) -> Option<i32> {
        Wire::min_by(a,b,Section::intersection_distance)
    }

    fn intersection_steps(a: &Wire, b: &Wire) -> Option<i32> {
        Wire::min_by(a,b,Section::intersection_steps)
    }

    fn min_by(a: &Wire, b: &Wire, check: fn(&Section, &Section) -> Option<i32>)-> Option<i32> {
        a.sections.iter()
            .flat_map(|s1| {
                b.sections.iter()
                    .flat_map(|s2| {
                        check(s1, s2)
                    })
                    .min()
            }).min()
    }
}

fn main() {
    let wires = parse_input();
    let w1 = &wires[0];
    let w2 = &wires[1];
    println!("Answer 1 = {}", Wire::intersection_distance(w1, w2).unwrap());
    println!("Answer 2 = {}", Wire::intersection_steps(w1, w2).unwrap());
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
    fn new_section() {
        let start = Location { x: 5, y: 7 };
        let section = Section::new(start, 10, "R75");
        assert_eq!(section, Section {
            start,
            direction: Horizontal,
            size: 75,
            steps: 10,
            range: Range { min: 5, max: 80 },
        });
    }

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