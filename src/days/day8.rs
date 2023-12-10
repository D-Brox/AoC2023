use std::collections::HashMap;
use std::mem::swap;

use pest::Parser;
use pest_derive::Parser as DeriveParser;

#[derive(DeriveParser)]
#[grammar = "days/day8.pest"]
pub struct NetworkParser;

#[derive(Clone)]
enum Direction {
    Left,
    Right,
}

fn parse(input: String) -> (Vec<Direction>, HashMap<String, (String, String)>) {
    let mut pairs = NetworkParser::parse(Rule::network, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();
    let steps = pairs.next().unwrap().into_inner();
    let mut navigation = Vec::new();
    for s in steps {
        match s.as_str() {
            "L" => navigation.push(Direction::Left),
            "R" => navigation.push(Direction::Right),
            _ => unreachable!(),
        }
    }
    let mut network = HashMap::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::map => {
                let mut pairs = pair.into_inner();
                let current = pairs.next().unwrap().as_str().to_string();
                let left = pairs.next().unwrap().as_str().to_string();
                let right = pairs.next().unwrap().as_str().to_string();
                network.insert(current, (left, right));
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    (navigation, network)
}

pub fn solution1(input: Vec<String>) -> u64 {
    let input = input.join("\n");

    let (directions, network) = parse(input);

    let mut navigation = directions.iter();
    let mut start = "AAA".to_string();
    let end = "ZZZ".to_string();

    let mut steps = 0;
    while start != end {
        if navigation.len() == 0 {
            navigation = directions.iter();
        }

        steps += 1;

        match navigation.next().unwrap() {
            Direction::Left => start = network[&start].0.clone(),
            Direction::Right => start = network[&start].1.clone(),
        }
    }

    steps
}

fn lcm(first: u64, second: u64) -> u64 { first * second / gcd(first, second) }

fn gcd(first: u64, second: u64) -> u64 {
    let mut max = first;
    let mut min = second;

    if min > max {
        swap(&mut min, &mut max);
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

pub fn solution2(input: Vec<String>) -> u64 {
    let input = input.join("\n");

    let (directions, network) = parse(input);

    let start: Vec<String> = network
        .keys()
        .filter(|&k| k.ends_with('A'))
        .cloned()
        .collect();

    let mut steps = Vec::new();

    for mut s in start {
        let mut navigation = directions.iter();
        let mut step = 0;

        while !s.ends_with('Z') {
            if navigation.len() == 0 {
                navigation = directions.iter();
            }

            step += 1;

            match navigation.next().unwrap() {
                Direction::Left => s = network[&s].0.clone(),
                Direction::Right => s = network[&s].1.clone(),
            }
        }
        steps.push(step);
    }

    steps.iter().copied().reduce(lcm).unwrap()
}
