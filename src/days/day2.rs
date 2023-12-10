use std::cmp::max;

use pest::Parser;
use pest_derive::Parser as DeriveParser;

#[derive(DeriveParser)]
#[grammar = "days/day2.pest"]
pub struct GamesParser;

fn parse(input: String, power: bool) -> Option<u32> {
    let mut pairs = GamesParser::parse(Rule::game, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();

    let id = pairs.next().unwrap().as_str().parse().unwrap();
    let mut red = 0;
    let mut green = 0;
    let mut blue = 0;

    for pair in pairs {
        if !power {
            red = 0;
            green = 0;
            blue = 0;
        }

        match pair.as_rule() {
            Rule::set => {
                let pairs = pair.into_inner();
                let mut number = 0;
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::number => number = pair.as_str().parse().unwrap(),
                        Rule::color => {
                            let color = pair.as_str();

                            if !power {
                                match color {
                                    "red" => red += number,
                                    "green" => green += number,
                                    "blue" => blue += number,
                                    _ => unreachable!(),
                                }
                                number = 0;
                            } else {
                                match color {
                                    "red" => red = max(red, number),
                                    "green" => green = max(green, number),
                                    "blue" => blue = max(blue, number),
                                    _ => unreachable!(),
                                }
                            }
                        },
                        _ => unreachable!(),
                    }
                }
            },
            Rule::EOI => (),
            _ => unreachable!(),
        };
        if !power && (red > 12 || green > 13 || blue > 14) {
            return None;
        }
    }

    if !power {
        Some(id)
    } else {
        Some(red * green * blue)
    }
}

pub fn solution1(input: Vec<String>) -> u32 {
    let mut output: Vec<u32> = Vec::new();

    for line in input {
        let id = parse(line, false);
        if let Some(id) = id {
            output.push(id);
        }
    }

    output.iter().sum()
}

pub fn solution2(input: Vec<String>) -> u32 {
    let mut output: Vec<u32> = Vec::new();

    for line in input {
        output.push(parse(line, true).unwrap());
    }

    output.iter().sum()
}
