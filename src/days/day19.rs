use std::collections::HashMap;

use pest::Parser;
use pest_derive::Parser as DeriveParser;

#[derive(DeriveParser)]
#[grammar = "days/day19.pest"]
struct RatingsParser;

#[derive(Debug, Clone)]
enum Result {
    Accept,
    Reject,
    Rule(String),
}

impl From<&str> for Result {
    fn from(value: &str) -> Self {
        match value {
            "A" => Self::Accept,
            "R" => Self::Reject,
            _ => Self::Rule(value.to_string()),
        }
    }
}

#[derive(Debug, Clone)]
enum Step {
    Less(String, u64, Result),
    Greater(String, u64, Result),
    Final(Result),
}

impl From<&str> for Step {
    fn from(value: &str) -> Self {
        Self::Final(value.into())
    }
}

fn parse(input: String) -> (HashMap<String, Vec<Step>>, Vec<HashMap<String, u64>>) {
    let pairs = RatingsParser::parse(Rule::ratings, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();
    let mut workflows = HashMap::new();
    let mut parts = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::workflows => {
                let pairs = pair.into_inner(); // vec[rule]
                for pair in pairs {
                    let mut pairs = pair.into_inner(); // vec[step]
                    let mut steps: Vec<Step> = Vec::new();
                    let code = pairs.next().unwrap().as_str().to_string();
                    for pair in pairs {
                        match pair.as_rule() {
                            Rule::step => {
                                let mut pairs = pair.into_inner(); // (code condition number next)
                                let piece = pairs.next().unwrap().as_str().to_string();
                                let condition = pairs.next().unwrap().as_str();
                                let value: u64 = pairs.next().unwrap().as_str().parse().unwrap();
                                let next = pairs.next().unwrap().as_str();
                                match condition {
                                    "<" => steps.push(Step::Less(piece, value, next.into())),
                                    ">" => steps.push(Step::Greater(piece, value, next.into())),
                                    _ => unreachable!(),
                                }
                            },
                            Rule::code => steps.push(pair.as_str().into()),
                            _ => unreachable!(),
                        }
                    }
                    workflows.insert(code, steps);
                }
            },
            Rule::parts => {
                let pairs = pair.into_inner(); // vec[part]
                for pair in pairs {
                    let pairs = pair.into_inner(); // vec[piece]
                    let mut part = HashMap::new();
                    for pair in pairs {
                        let mut pairs = pair.into_inner(); // (code number)
                        let code = pairs.next().unwrap().as_str().to_string();
                        let number: u64 = pairs.next().unwrap().as_str().parse().unwrap();
                        part.insert(code, number);
                    }
                    parts.push(part);
                }
            },
            Rule::EOI => (),
            _ => unreachable!(),
        };
    }
    (workflows, parts)
}

pub fn solution1(input: Vec<String>) -> u64 {
    let (workflows, parts) = parse(input.join("\n"));
    let mut rating = 0;
    'outer: for part in parts {
        let mut workflow = "in".to_string();
        'middle: loop {
            let rule = &workflows[&workflow];
            'inner: for step in rule {
                let next_workflow;
                match step {
                    Step::Less(key, value, next) => {
                        if &part[key] < value {
                            next_workflow = next;
                        } else {
                            continue 'inner;
                        }
                    },
                    Step::Greater(key, value, next) => {
                        if &part[key] > value {
                            next_workflow = next;
                        } else {
                            continue 'inner;
                        }
                    },
                    Step::Final(next) => {
                        next_workflow = next;
                    },
                }
                match next_workflow {
                    Result::Accept => {
                        rating += part.values().sum::<u64>();
                        continue 'outer;
                    },
                    Result::Reject => {
                        continue 'outer;
                    },
                    Result::Rule(next) => {
                        workflow = next.to_string();
                        continue 'middle;
                    },
                }
            }
        }
    }
    rating
}

#[derive(Debug, Clone, Copy)]
struct PieceGroup {
    x: (u64, u64),
    m: (u64, u64),
    a: (u64, u64),
    s: (u64, u64),
}

fn split_range(range: (u64, u64), value: u64) -> (Option<(u64, u64)>, Option<(u64, u64)>) {
    if range.0 >= value {
        return (None, Some(range));
    }
    if range.1 < value {
        return (Some(range), None);
    }
    (Some((range.0, value - 1)), Some((value, range.1)))
}

impl PieceGroup {
    fn split(self, rating: String, value: u64) -> (Option<Self>, Option<Self>) {
        let Self { x, m, a, s } = self;
        match rating.as_str() {
            "x" => {
                let (x1, x2) = split_range(x, value);
                let s1 = if let Some(x) = x1 { Some(Self { x, m, a, s }) } else { None };
                let s2 = if let Some(x) = x2 { Some(Self { x, m, a, s }) } else { None };
                return (s1, s2);
            },
            "m" => {
                let (m1, m2) = split_range(m, value);
                let s1 = if let Some(m) = m1 { Some(Self { x, m, a, s }) } else { None };
                let s2 = if let Some(m) = m2 { Some(Self { x, m, a, s }) } else { None };
                return (s1, s2);
            },
            "a" => {
                let (a1, a2) = split_range(a, value);
                let s1 = if let Some(a) = a1 { Some(Self { x, m, a, s }) } else { None };
                let s2 = if let Some(a) = a2 { Some(Self { x, m, a, s }) } else { None };
                return (s1, s2);
            },
            "s" => {
                let (s1, s2) = split_range(s, value);
                let s1 = if let Some(s) = s1 { Some(Self { x, m, a, s }) } else { None };
                let s2 = if let Some(s) = s2 { Some(Self { x, m, a, s }) } else { None };
                return (s1, s2);
            },
            _ => unreachable!(),
        }
    }

    fn total(self) -> u64 {
        let Self { x, m, a, s } = self;
        (x.1 - x.0 + 1) * (m.1 - m.0 + 1) * (a.1 - a.0 + 1) * (s.1 - s.0 + 1)
    }
}

pub fn solution2(input: Vec<String>) -> u64 {
    let (workflows, _) = parse(input.join("\n"));
    let mut groups = vec![(
        PieceGroup {
            x: (1, 4000),
            m: (1, 4000),
            a: (1, 4000),
            s: (1, 4000),
        },
        "in".to_string(),
    )];
    let mut accepted = 0;
    while let Some((mut group, workflow)) = groups.pop() {
        let rule = &workflows[&workflow];
        for step in rule {
            match step {
                Step::Less(key, value, next) => {
                    let (left, right) = group.split(key.to_string(), *value);
                    if let Some(left) = left {
                        match next {
                            Result::Accept => accepted += left.total(),
                            Result::Reject => (),
                            Result::Rule(next) => groups.push((left, next.to_string())),
                        }
                    }
                    if let Some(right) = right {
                        group = right;
                    }
                },
                Step::Greater(key, value, next) => {
                    let (left, right) = group.split(key.to_string(), *value + 1);
                    if let Some(right) = right {
                        match next {
                            Result::Accept => accepted += right.total(),
                            Result::Reject => (),
                            Result::Rule(next) => groups.push((right, next.to_string())),
                        }
                    }
                    if let Some(left) = left {
                        group = left;
                    }
                },
                Step::Final(Result::Accept) => accepted += group.total(),
                Step::Final(Result::Reject) => (),
                Step::Final(Result::Rule(next)) => groups.push((group, next.to_string())),
            }
        }
    }
    accepted
}
