use std::collections::VecDeque;
use std::ops::Not;

#[derive(Debug, PartialEq, Clone, Copy)]
enum MirrorSplit {
    Positive,
    Negative,
    Horizontal,
    Vertical,
    Empty,
}

impl From<char> for MirrorSplit {
    fn from(value: char) -> Self {
        match value {
            '.' => MirrorSplit::Empty,
            '|' => MirrorSplit::Vertical,
            '-' => MirrorSplit::Horizontal,
            '/' => MirrorSplit::Negative,
            '\\' => MirrorSplit::Positive,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Direction {
    Positive,
    Negative,
}

impl Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Positive => Direction::Negative,
            Direction::Negative => Direction::Positive,
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]

enum Ray {
    Horizontal(Direction),
    Vertical(Direction),
}

impl Ray {
    fn next(self, x: usize, y: usize, v: usize, h: usize) -> Option<(usize, usize)> {
        match self {
            Ray::Vertical(Direction::Negative) => {
                if x > 0 {
                    Some((x - 1, y))
                } else {
                    None
                }
            },
            Ray::Vertical(Direction::Positive) => {
                if x + 1 < v {
                    Some((x + 1, y))
                } else {
                    None
                }
            },
            Ray::Horizontal(Direction::Negative) => {
                if y > 0 {
                    Some((x, y - 1))
                } else {
                    None
                }
            },
            Ray::Horizontal(Direction::Positive) => {
                if y + 1 < h {
                    Some((x, y + 1))
                } else {
                    None
                }
            },
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Energized {
    energy: bool,
    left: bool,
    right: bool,
    up: bool,
    down: bool,
}

impl Energized {
    fn new() -> Self {
        Self {
            energy: false,
            left: false,
            right: false,
            up: false,
            down: false,
        }
    }

    fn set_direction(&mut self, ray: Ray) -> bool {
        match ray {
            Ray::Horizontal(Direction::Negative) => {
                if !self.energy {
                    self.left = true;
                    self.energy = true;
                    false
                } else if !self.left {
                    self.left = true;
                    false
                } else {
                    true
                }
            },
            Ray::Horizontal(Direction::Positive) => {
                if !self.energy {
                    self.energy = true;
                    self.right = true;
                    false
                } else if !self.right {
                    self.right = true;
                    false
                } else {
                    true
                }
            },
            Ray::Vertical(Direction::Negative) => {
                if !self.energy {
                    self.energy = true;
                    self.up = true;
                    false
                } else if !self.up {
                    self.up = true;
                    false
                } else {
                    true
                }
            },
            Ray::Vertical(Direction::Positive) => {
                if !self.energy {
                    self.energy = true;
                    self.down = true;
                    false
                } else if !self.down {
                    self.down = true;
                    false
                } else {
                    true
                }
            },
        }
    }
}

fn search(grid: &Vec<Vec<MirrorSplit>>, mut queue: VecDeque<(usize, usize, Ray)>) -> usize {
    let h = grid[0].len();
    let v = grid.len();

    let mut energized = vec![vec![Energized::new(); h]; v];
    while !queue.is_empty() {
        let (x, y, ray) = queue.pop_front().unwrap();
        if !energized[x][y].set_direction(ray) {
            match (grid[x][y], ray) {
                (MirrorSplit::Positive, Ray::Horizontal(d)) => {
                    if let Some((xn, yn)) = Ray::Vertical(d).next(x, y, v, h) {
                        queue.push_back((xn, yn, Ray::Vertical(d)));
                    }
                },
                (MirrorSplit::Positive, Ray::Vertical(d)) => {
                    if let Some((xn, yn)) = Ray::Horizontal(d).next(x, y, v, h) {
                        queue.push_back((xn, yn, Ray::Horizontal(d)));
                    }
                },
                (MirrorSplit::Negative, Ray::Horizontal(d)) => {
                    if let Some((xn, yn)) = Ray::Vertical(!d).next(x, y, v, h) {
                        queue.push_back((xn, yn, Ray::Vertical(!d)));
                    }
                },
                (MirrorSplit::Negative, Ray::Vertical(d)) => {
                    if let Some((xn, yn)) = Ray::Horizontal(!d).next(x, y, v, h) {
                        queue.push_back((xn, yn, Ray::Horizontal(!d)));
                    }
                },
                (MirrorSplit::Horizontal, Ray::Vertical(_)) => {
                    if let Some((xn, yn)) = Ray::Horizontal(Direction::Negative).next(x, y, v, h) {
                        queue.push_back((xn, yn, Ray::Horizontal(Direction::Negative)));
                    }
                    if let Some((xn, yn)) = Ray::Horizontal(Direction::Positive).next(x, y, v, h) {
                        queue.push_back((xn, yn, Ray::Horizontal(Direction::Positive)));
                    }
                },
                (MirrorSplit::Vertical, Ray::Horizontal(_)) => {
                    if let Some((xn, yn)) = Ray::Vertical(Direction::Negative).next(x, y, v, h) {
                        queue.push_back((xn, yn, Ray::Vertical(Direction::Negative)));
                    }
                    if let Some((xn, yn)) = Ray::Vertical(Direction::Positive).next(x, y, v, h) {
                        queue.push_back((xn, yn, Ray::Vertical(Direction::Positive)));
                    }
                },
                (_, Ray::Vertical(d)) => {
                    if let Some((xn, yn)) = Ray::Vertical(d).next(x, y, v, h) {
                        queue.push_back((xn, yn, Ray::Vertical(d)));
                    }
                },
                (_, Ray::Horizontal(d)) => {
                    if let Some((xn, yn)) = Ray::Horizontal(d).next(x, y, v, h) {
                        queue.push_back((xn, yn, Ray::Horizontal(d)));
                    }
                },
            }
        }
    }

    energized
        .iter()
        .map(|l| {
            l.iter()
                .map(|&x| if x.energy { 1 } else { 0 })
                .sum::<usize>()
        })
        .sum()
}

fn parse(input: Vec<String>) -> Vec<Vec<MirrorSplit>> {
    let mut grid = Vec::new();
    for line in input {
        let l = line.chars().map(|i| i.into()).collect();
        grid.push(l);
    }
    grid
}

pub fn solution1(input: Vec<String>) -> usize {
    let grid = parse(input);

    let queue: VecDeque<(usize, usize, Ray)> =
        [(0, 0, Ray::Horizontal(Direction::Positive))].into();
    search(&grid, queue)
}

pub fn solution2(input: Vec<String>) -> usize {
    let grid = parse(input);

    let v = grid.len();
    let h = grid[0].len();
    let mut energies = Vec::new();

    for x in 0..v {
        let queue = [(x, 0, Ray::Horizontal(Direction::Positive))].into();
        energies.push(search(&grid, queue));
        let queue = [(x, h - 1, Ray::Horizontal(Direction::Negative))].into();
        energies.push(search(&grid, queue));
    }

    for y in 0..h {
        let queue = [(0, y, Ray::Vertical(Direction::Positive))].into();
        energies.push(search(&grid, queue));
        let queue = [(v - 1, y, Ray::Vertical(Direction::Negative))].into();
        energies.push(search(&grid, queue));
    }

    energies.iter().max().unwrap().to_owned()
}
