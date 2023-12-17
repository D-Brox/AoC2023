#[derive(Debug, Clone, Copy)]
enum Tile {
    Positive,
    Negative,
    Horizontal,
    Vertical,
    Empty,
}

impl From<char> for Tile {
    fn from(value: char) -> Self {
        match value {
            '.' => Tile::Empty,
            '|' => Tile::Vertical,
            '-' => Tile::Horizontal,
            '/' => Tile::Negative,
            '\\' => Tile::Positive,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum Sign {
    Positive,
    Negative,
}

impl std::ops::Not for Sign {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Sign::Positive => Sign::Negative,
            Sign::Negative => Sign::Positive,
        }
    }
}

#[derive(Debug, Clone, Copy)]

enum Direction {
    Horizontal,
    Vertical,
}

impl std::ops::Not for Direction {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            Direction::Horizontal => Direction::Vertical,
            Direction::Vertical => Direction::Horizontal,
        }
    }
}

struct Beam(Direction, Sign);

impl Beam {
    fn next(self, x: usize, y: usize, v: usize, h: usize) -> Option<(usize, usize, Self)> {
        match self {
            Beam(Direction::Vertical, Sign::Negative) => {
                if x > 0 {
                    return Some((x - 1, y, self));
                }
            },
            Beam(Direction::Vertical, Sign::Positive) => {
                if x + 1 < v {
                    return Some((x + 1, y, self));
                }
            },
            Beam(Direction::Horizontal, Sign::Negative) => {
                if y > 0 {
                    return Some((x, y - 1, self));
                }
            },
            Beam(Direction::Horizontal, Sign::Positive) => {
                if y + 1 < h {
                    return Some((x, y + 1, self));
                }
            },
        }
        None
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

    fn overlap(&mut self, beam: &Beam) -> bool {
        match &beam {
            Beam(Direction::Vertical, Sign::Negative) => {
                if !self.energy || !self.left {
                    self.left = true;
                    self.energy = true;
                    return false;
                }
            },
            Beam(Direction::Vertical, Sign::Positive) => {
                if !self.energy || !self.right {
                    self.energy = true;
                    self.right = true;
                    return false;
                }
            },
            Beam(Direction::Horizontal, Sign::Negative) => {
                if !self.energy || !self.up {
                    self.energy = true;
                    self.up = true;
                    return false;
                }
            },
            Beam(Direction::Horizontal, Sign::Positive) => {
                if !self.energy || !self.down {
                    self.energy = true;
                    self.down = true;
                    return false;
                }
            },
        }
        true
    }
}

fn search(grid: &Vec<Vec<Tile>>, mut queue: Vec<(usize, usize, Beam)>) -> usize {
    let h = grid[0].len();
    let v = grid.len();

    let mut energized = vec![vec![Energized::new(); h]; v];
    while let Some((x, y, beam)) = queue.pop() {
        if energized[x][y].overlap(&beam) {
            continue;
        }
        queue.extend(
            match (grid[x][y], beam) {
                (Tile::Positive, Beam(d, s)) => vec![Beam(!d, s).next(x, y, v, h)],
                (Tile::Negative, Beam(d, s)) => vec![Beam(!d, !s).next(x, y, v, h)],
                (Tile::Horizontal, Beam(Direction::Vertical, s)) => vec![
                    Beam(Direction::Horizontal, s).next(x, y, v, h),
                    Beam(Direction::Horizontal, !s).next(x, y, v, h),
                ],
                (Tile::Vertical, Beam(Direction::Horizontal, s)) => vec![
                    Beam(Direction::Vertical, s).next(x, y, v, h),
                    Beam(Direction::Vertical, !s).next(x, y, v, h),
                ],
                (_, b) => vec![b.next(x, y, v, h)],
            }
            .into_iter()
            .flatten(),
        );
    }

    energized
        .iter()
        .map(|l| l.iter().map(|&x| x.energy as usize).sum::<usize>())
        .sum()
}

pub fn solution1(input: Vec<String>) -> usize {
    let grid = input
        .iter()
        .map(|l| l.chars().map(|i| i.into()).collect())
        .collect();
    search(
        &grid,
        [(0, 0, Beam(Direction::Horizontal, Sign::Positive))].into(),
    )
}

pub fn solution2(input: Vec<String>) -> usize {
    let grid: Vec<Vec<_>> = input
        .iter()
        .map(|l| l.chars().map(|i| i.into()).collect())
        .collect();

    let v = grid.len();
    let h = grid[0].len();
    let mut energies = Vec::new();

    for x in 0..v {
        energies.push(search(
            &grid,
            [(x, 0, Beam(Direction::Horizontal, Sign::Positive))].into(),
        ));
        energies.push(search(
            &grid,
            [(x, h - 1, Beam(Direction::Horizontal, Sign::Negative))].into(),
        ));
    }

    for y in 0..h {
        energies.push(search(
            &grid,
            [(0, y, Beam(Direction::Vertical, Sign::Positive))].into(),
        ));
        energies.push(search(
            &grid,
            [(v - 1, y, Beam(Direction::Vertical, Sign::Negative))].into(),
        ));
    }

    energies.iter().max().unwrap().to_owned()
}
