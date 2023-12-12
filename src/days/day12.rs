use pest::Parser;
use pest_derive::Parser as DeriveParser;

#[derive(DeriveParser)]
#[grammar = "days/day12.pest"]
pub struct SpringParser;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Spring {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for Spring {
    fn from(value: char) -> Self {
        match value {
            '.' => Spring::Operational,
            '#' => Spring::Damaged,
            '?' => Spring::Unknown,
            _ => unreachable!(),
        }
    }
}

fn parse(input: String) -> (Vec<Spring>, Vec<usize>) {
    let pairs = SpringParser::parse(Rule::springs, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();
    let mut springs = Vec::new();
    let mut counts = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::spring => springs.extend(pair.as_str().chars().map(|c| Spring::from(c))),
            Rule::number => counts.push(pair.as_str().parse().unwrap()),
            Rule::EOI => (),
            _ => unreachable!(),
        };
    }

    (springs, counts)
}

fn search_matches((mut row, counts): (Vec<Spring>, Vec<usize>)) -> usize {
    // Extend beginning in case it not starts with operational
    if row.first().unwrap() != &Spring::Operational {
        let mut row_extended = vec![Spring::Operational];
        row_extended.append(&mut row);
        row = row_extended;
    }

    // Extend end for result
    let mut total_possible = vec![0; row.len() + 1];
    let first = row
        .iter()
        .position(|&x| x == Spring::Damaged)
        .unwrap_or(row.len());
    for i in 0..(first + 1) {
        total_possible[i] = 1;
    }

    for count in counts {
        let mut next_total = vec![0; row.len() + 1];
        let mut group = 0;
        for (n, &s) in row.iter().enumerate() {
            match s {
                Spring::Operational => {
                    group = 0;
                    next_total[n + 1] += next_total[n]; // Pass matches forward
                },
                Spring::Damaged => group += 1,
                Spring::Unknown => {
                    group += 1;
                    next_total[n + 1] += next_total[n]; // Pass matches forward
                },
            }
            // If count fits in the group
            // Needs to check if the previous space isn't damaged
            // since it would become count+1
            if count <= group && row[n - count] != Spring::Damaged {
                next_total[n + 1] += total_possible[n - count] // Pass previous
                                                               // matches forward
            }
        }
        total_possible = next_total;
    }

    total_possible.last().unwrap().to_owned()
}

pub fn solution1(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|l| search_matches(parse(l.to_owned())))
        .into_iter()
        .sum()
}

pub fn solution2(input: Vec<String>) -> usize {
    input
        .iter()
        .map(|l| {
            let (row, counts) = l.split_once(' ').unwrap();
            let row = (0..5).map(|_| row).collect::<Vec<_>>();
            let counts = (0..5).map(|_| counts).collect::<Vec<_>>();
            let l = row.join("?") + " " + &counts.join(",");
            search_matches(parse(l))
        })
        .into_iter()
        .sum()
}
