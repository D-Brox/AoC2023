use itertools::izip;
use pest::Parser;
use pest_derive::Parser as DeriveParser;

#[derive(DeriveParser)]
#[grammar = "days/day6.pest"]
pub struct CompetitionParser;

fn parse(input: String, kerning: bool) -> Vec<(u64, u64)> {
    let pairs = CompetitionParser::parse(Rule::competition, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();
    let mut times = Vec::new();
    let mut distances = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::times => {
                let pairs = pair.into_inner();
                let mut number = String::new();
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::number => {
                            if !kerning {
                                let number = pair.as_str().parse().unwrap();
                                times.push(number);
                            } else {
                                number += pair.as_str();
                            }
                        },
                        _ => unreachable!(),
                    }
                }
                if kerning {
                    times.push(number.parse().unwrap());
                }
            },
            Rule::distances => {
                let pairs = pair.into_inner();
                let mut number = String::new();
                for pair in pairs {
                    match pair.as_rule() {
                        Rule::number => {
                            if !kerning {
                                let number = pair.as_str().parse().unwrap();
                                distances.push(number);
                            } else {
                                number += pair.as_str();
                            }
                        },
                        _ => unreachable!(),
                    }
                }
                if kerning {
                    distances.push(number.parse().unwrap());
                }
            },
            Rule::EOI => (),
            _ => unreachable!(),
        }
    }
    izip!(times, distances).collect()
}

pub fn solution1(input: Vec<String>) -> u64 {
    let mut output = Vec::new();

    let input = input.join("\n");
    let pairs = parse(input, false);

    for (t, d) in pairs {
        let delta = f64::sqrt((t * t - 4 * d) as f64);
        let first = (((t as f64) - delta) / 2.0).ceil() as u64;
        let last = (((t as f64) + delta) / 2.0).floor() as u64;
        output.push(last - first + 1);
    }

    output.iter().copied().reduce(|a, b| a * b).unwrap()
}

pub fn solution2(input: Vec<String>) -> u64 {
    let input = input.join("\n");
    let (t, d) = parse(input, true).first().unwrap().to_owned();

    let delta = f64::sqrt((t * t - 4 * d) as f64);
    let first = (((t as f64) - delta) / 2.0).ceil() as u64;
    let last = (((t as f64) + delta) / 2.0).floor() as u64;

    last - first + 1
}
