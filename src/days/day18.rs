use itertools::Itertools;
use pest::Parser;
use pest_derive::Parser as DeriveParser;

#[derive(DeriveParser)]
#[grammar = "days/day18.pest"]
pub struct TrenchParser;

#[derive(Debug, Copy, Clone)]
enum Horizontal {
    Left,
    Right,
    None,
}

#[derive(Debug, Copy, Clone)]
enum Vertical {
    Up,
    Down,
    None,
}

#[derive(Debug, Copy, Clone)]

struct Direction(Horizontal, Vertical);

impl From<&str> for Direction {
    fn from(value: &str) -> Self {
        match value {
            "R" | "0" => Direction(Horizontal::Right, Vertical::None),
            "D" | "1" => Direction(Horizontal::None, Vertical::Down),
            "L" | "2" => Direction(Horizontal::Left, Vertical::None),
            "U" | "3" => Direction(Horizontal::None, Vertical::Up),
            _ => unreachable!(),
        }
    }
}

fn dig(instructions: Vec<(Direction, i64)>) -> usize {
    let mut start = (0, 0);
    let mut vertices = vec![(0, 0)];
    let mut perimeter = 0;

    for (direction, distance) in instructions {
        match direction {
            Direction(_, Vertical::Up) => {
                vertices.push((start.0 - distance, start.1));
                start = (start.0 - distance, start.1);
            },
            Direction(_, Vertical::Down) => {
                vertices.push((start.0 + distance, start.1));
                start = (start.0 + distance, start.1);
            },
            Direction(Horizontal::Left, _) => {
                vertices.push((start.0, start.1 - distance));
                start = (start.0, start.1 - distance);
            },
            Direction(Horizontal::Right, _) => {
                vertices.push((start.0, start.1 + distance));
                start = (start.0, start.1 + distance);
            },
            _ => unreachable!(),
        }
        perimeter += distance as usize;
    }

    let area: usize = vertices
        .iter()
        .tuple_windows()
        .map(|((xa, ya), (xb, yb))| (xa * yb - xb * ya))
        .sum::<i64>()
        .unsigned_abs() as usize
        >> 1;

    area + perimeter / 2 + 1
}

fn parse(input: String, invert: bool) -> (Direction, i64) {
    let mut pairs = TrenchParser::parse(Rule::trench, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();
    if !invert {
        let direction: Direction = pairs.next().unwrap().as_str().into();
        let distance: i64 = pairs.next().unwrap().as_str().parse().unwrap();
        (direction, distance)
    } else {
        let hexcode = pairs.nth(2).unwrap().as_str();
        let direction = hexcode[7..8].into();
        let distance = i64::from_str_radix(&hexcode[2..7], 16).unwrap();
        (direction, distance)
    }
}

pub fn solution1(input: Vec<String>) -> usize {
    let mut instructions = Vec::new();

    for line in input {
        let line = parse(line, false);
        instructions.push(line);
    }

    dig(instructions)
}

pub fn solution2(input: Vec<String>) -> usize {
    let mut instructions = Vec::new();

    for line in input {
        let line = parse(line, true);
        instructions.push(line);
    }

    dig(instructions)
}
