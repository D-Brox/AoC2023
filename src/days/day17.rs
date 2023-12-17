use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl std::ops::Not for Direction {
    type Output = Direction;

    fn not(self) -> Self::Output {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct Node {
    coord: Coord,
    cost: usize,
    last: Direction,
    repeat: u8,
}

#[derive(Hash, Debug, Copy, Clone, Eq, PartialEq)]

struct Coord(usize, usize);

impl Coord {
    fn next(self, direction: Direction, v: usize, h: usize) -> Option<Coord> {
        match direction {
            Direction::Up => {
                if self.0 > 0 {
                    return Some(Coord(self.0 - 1, self.1));
                }
            },
            Direction::Down => {
                if self.0 + 1 < v {
                    return Some(Coord(self.0 + 1, self.1));
                }
            },
            Direction::Left => {
                if self.1 > 0 {
                    return Some(Coord(self.0, self.1 - 1));
                }
            },
            Direction::Right => {
                if self.1 + 1 < h {
                    return Some(Coord(self.0, self.1 + 1));
                }
            },
        }
        None
    }
}

impl Node {
    fn next(self, grid: &Vec<Vec<usize>>) -> Vec<Self> {
        let mut next = Vec::new();
        let v = grid.len();
        let h = grid[0].len();
        for direction in [
            Direction::Down,
            Direction::Right,
            Direction::Up,
            Direction::Left,
        ] {
            if self.last == direction && self.repeat == 3 {
                continue;
            }
            if self.last == !direction {
                continue;
            }
            if let Some(coord) = self.coord.next(direction, v, h) {
                let cost = self.cost + grid[coord.0][coord.1];
                let repeat = if self.last == direction { self.repeat + 1 } else { 1 };
                next.push(Node {
                    coord,
                    cost,
                    last: direction,
                    repeat,
                })
            }
        }
        next
    }

    fn ultra_next(self, grid: &Vec<Vec<usize>>) -> Vec<Self> {
        let mut next = Vec::new();
        let v = grid.len();
        let h = grid[0].len();
        for direction in [
            Direction::Down,
            Direction::Right,
            Direction::Up,
            Direction::Left,
        ] {
            if (self.last != direction && self.repeat < 4)
                || self.last == !direction
                || self.last == direction && self.repeat == 10
            {
                continue;
            }
            if let Some(coord) = self.coord.next(direction, v, h) {
                let cost = self.cost + grid[coord.0][coord.1];
                let repeat = if self.last == direction { self.repeat + 1 } else { 1 };
                next.push(Node {
                    coord,
                    cost,
                    last: direction,
                    repeat,
                })
            }
        }
        next
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering { other.cost.cmp(&self.cost) }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> { Some(self.cmp(other)) }
}

fn dijkstra<F>(input: Vec<String>, next: F) -> usize
where
    F: Fn(Node, &Vec<Vec<usize>>) -> Vec<Node>,
{
    let grid: Vec<Vec<usize>> = input
        .iter()
        .map(|l| {
            l.chars()
                .map(|i| i.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let v = grid.len();
    let h = grid[0].len();

    let mut heap = BinaryHeap::new();
    let mut visited = HashSet::new();

    heap.push(Node {
        coord: Coord(1, 0),
        cost: grid[1][0],
        last: Direction::Down,
        repeat: 1,
    });
    heap.push(Node {
        coord: Coord(0, 1),
        cost: grid[0][1],
        last: Direction::Right,
        repeat: 1,
    });

    while let Some(node) = heap.pop() {
        if node.coord == Coord(v - 1, h - 1) {
            return node.cost;
        }
        next(node, &grid).iter().for_each(|&n| {
            if visited.insert((n.coord, n.last, n.repeat)) {
                heap.push(n)
            }
        });
    }
    usize::MAX
}

pub fn solution1(input: Vec<String>) -> usize { dijkstra(input, |n, grid| n.next(grid)) }

pub fn solution2(input: Vec<String>) -> usize { dijkstra(input, |n, grid| n.ultra_next(grid)) }
