use std::cmp::{max, min};

use itertools::Itertools;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Space {
    Void(usize),
    Galaxy,
}

impl Space {
    fn is_void(self) -> bool {
        match self {
            Space::Void(_) => true,
            Space::Galaxy => false,
        }
    }
}

impl From<char> for Space {
    fn from(value: char) -> Self {
        match value {
            '.' => Space::Void(1),
            '#' => Space::Galaxy,
            _ => unreachable!(),
        }
    }
}

fn expand(input: Vec<String>, distance: usize) -> usize {
    let mut space: Vec<Vec<Space>> = Vec::new();

    for line in input {
        let l = line.chars().map(Space::from).collect();
        space.push(l);
    }

    let h = space[0].len();

    for (i, l) in space.clone().iter().cloned().enumerate().rev() {
        if l.iter().all(|&x| x.is_void()) {
            space[i] = vec![Space::Void(distance); h];
        }
    }

    for j in (0..h).rev() {
        if space.iter().all(|l| l[j].is_void()) {
            for void in space.iter_mut() {
                void[j] = Space::Void(distance);
            }
        }
    }

    let mut galaxies = Vec::new();

    for (i, l) in space.iter().enumerate() {
        for (j, &s) in l.iter().enumerate() {
            if s == Space::Galaxy {
                galaxies.push((i, j));
            }
        }
    }

    galaxies
        .iter()
        .combinations(2)
        .map(|pair| {
            let first = pair[0];
            let second = pair[1];
            let mut dist = 0;
            for line in space
                .iter()
                .take(max(first.0, second.0))
                .skip(min(first.0, second.0))
            {
                match line[first.1] {
                    Space::Galaxy => dist += 1,
                    Space::Void(d) => dist += d,
                }
            }
            for s in space[first.0]
                .iter()
                .take(max(first.1, second.1))
                .skip(min(first.1, second.1))
            {
                match s {
                    Space::Galaxy => dist += 1,
                    Space::Void(d) => dist += d,
                }
            }
            dist
        })
        .sum()
}

pub fn solution1(input: Vec<String>) -> usize { expand(input, 2) }

pub fn solution2(input: Vec<String>) -> usize { expand(input, 1000000) }
