use pest::Parser;
use pest_derive::Parser as DeriveParser;

#[derive(DeriveParser)]
#[grammar = "days/day13.pest"]
pub struct LavaParser;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Lava {
    Ash,
    Rock,
}

fn parse(input: String) -> Vec<Vec<Vec<Lava>>> {
    let pairs = LavaParser::parse(Rule::lava, &input)
        .unwrap()
        .next()
        .unwrap()
        .into_inner();
    let mut patterns = Vec::new();
    for pair in pairs {
        match pair.as_rule() {
            Rule::pattern => {
                let pairs = pair.into_inner();
                let mut lines = Vec::new();
                for pair in pairs {
                    let pairs = pair.into_inner();
                    let mut line = Vec::new();
                    for pair in pairs {
                        match pair.as_rule() {
                            Rule::ash => line.push(Lava::Ash),
                            Rule::rock => line.push(Lava::Rock),
                            _ => unreachable!(),
                        }
                    }
                    lines.push(line);
                }
                patterns.push(lines);
            },
            Rule::EOI => (),
            _ => unreachable!(),
        };
    }

    patterns
}

fn transpose<T>(mut v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    for inner in &mut v {
        inner.reverse();
    }
    (0..v[0].len())
        .map(|_| {
            v.iter_mut()
                .map(|inner| inner.pop().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn find_mirror(pattern: Vec<Vec<Lava>>, smudge: bool) -> usize {
    let mut horizontal = 0;
    'outer: for i in 0..(pattern.len() - 1) {
        let mut allow = smudge;
        let mut a = i;
        let mut b = i + 1;
        loop {
            for j in 0..pattern[0].len() {
                if pattern[a][j] != pattern[b][j] {
                    if !allow {
                        continue 'outer;
                    } else {
                        allow = false;
                    }
                }
            }
            if a == 0 || b == pattern.len() - 1 {
                break;
            }
            a -= 1;
            b += 1;
        }
        if !allow {
            horizontal = i + 1;
            break;
        }
    }
    horizontal
}

pub fn solution1(input: Vec<String>) -> usize {
    let patterns = parse(input.join("\n"));
    patterns
        .iter()
        .map(|p| {
            let horizontal = find_mirror(p.to_vec(), false);
            if horizontal != 0 {
                100 * horizontal
            } else {
                let p = transpose(p.to_vec());
                find_mirror(p.to_vec(), false)
            }
        })
        .sum()
}

pub fn solution2(input: Vec<String>) -> usize {
    let patterns = parse(input.join("\n"));
    patterns
        .iter()
        .map(|p| {
            let horizontal = find_mirror(p.to_vec(), true);
            if horizontal != 0 {
                100 * horizontal
            } else {
                let p = transpose(p.to_vec());
                find_mirror(p.to_vec(), true)
            }
        })
        .map(|x| {
            println!("{x}");
            x
        })
        .sum()
}
