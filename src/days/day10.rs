use std::cmp::max;
use std::collections::VecDeque;

#[derive(Copy, Clone)]
struct Node {
    pub left: bool,
    pub right: bool,
    pub top: bool,
    pub bottom: bool,
}

impl Node {
    pub fn type_line(self) -> Option<Vertical> {
        if self.top || self.left || self.right || self.bottom {
            match (self.top, self.bottom) {
                (true, true) => Some(Vertical::Both),
                (true, false) => Some(Vertical::Up),
                (false, true) => Some(Vertical::Down),
                (false, false) => Some(Vertical::Not),
            }
        } else {
            None
        }
    }
}

impl From<char> for Node {
    fn from(value: char) -> Self {
        match value {
            '.' => Node {
                left: false,
                right: false,
                top: false,
                bottom: false,
            },
            '|' => Node {
                left: false,
                right: false,
                top: true,
                bottom: true,
            },
            '-' => Node {
                left: true,
                right: true,
                top: false,
                bottom: false,
            },
            'L' => Node {
                left: false,
                right: true,
                top: true,
                bottom: false,
            },
            'J' => Node {
                left: true,
                right: false,
                top: true,
                bottom: false,
            },
            '7' => Node {
                left: true,
                right: false,
                top: false,
                bottom: true,
            },
            'F' => Node {
                left: false,
                right: true,
                top: false,
                bottom: true,
            },
            'S' => Node {
                left: true,
                right: true,
                top: true,
                bottom: true,
            },
            _ => unreachable!(),
        }
    }
}

#[derive(PartialEq)]
enum Vertical {
    Up,
    Down,
    Both,
    Not,
}

fn parse(input: String) -> (Vec<Node>, Option<usize>) {
    let mut start = None;
    let nodes = input
        .chars()
        .enumerate()
        .map(|(n, i)| {
            if i == 'S' {
                start = Some(n)
            };
            i.into()
        })
        .collect();
    (nodes, start)
}

pub fn solution1(input: Vec<String>) -> usize {
    let mut grid = Vec::new();
    let mut start = (usize::MAX, usize::MAX);

    for (n, line) in input.iter().enumerate() {
        let (l, s) = parse(line.to_owned());

        grid.push(l);

        if let Some(s) = s {
            start = (n, s);
        }
    }

    for (i, l) in grid.clone().iter().cloned().enumerate() {
        for (j, n) in l.iter().enumerate() {
            // println!("{i} {j}");
            if j + 1 == l.len() || (n.right && !l[j + 1].left) {
                grid[i][j].right = false;
            }
            if j == 0 || (n.left && !l[j - 1].right) {
                grid[i][j].left = false;
            }
            if i == 0 || (n.top && !grid[i - 1][j].bottom) {
                grid[i][j].top = false;
            }
            if i + 1 == grid.len() || (n.bottom && !grid[i + 1][j].top) {
                grid[i][j].bottom = false;
            }
        }
    }

    let mut queue: VecDeque<((usize, usize), usize)> = [(start, 0)].into();
    let mut max_dist = 0;

    while !queue.is_empty() {
        let (node, distance) = queue.pop_front().unwrap();
        let Node {
            left,
            right,
            top,
            bottom,
        } = grid[node.0][node.1];
        if left {
            grid[node.0][node.1 - 1].right = false;
            max_dist = max(max_dist, distance + 1);
            queue.push_back(((node.0, node.1 - 1), distance + 1))
        }
        if right {
            grid[node.0][node.1 + 1].left = false;
            max_dist = max(max_dist, distance + 1);
            queue.push_back(((node.0, node.1 + 1), distance + 1))
        }
        if top {
            grid[node.0 - 1][node.1].bottom = false;
            max_dist = max(max_dist, distance + 1);
            queue.push_back(((node.0 - 1, node.1), distance + 1))
        }
        if bottom {
            grid[node.0 + 1][node.1].top = false;
            max_dist = max(max_dist, distance + 1);
            queue.push_back(((node.0 + 1, node.1), distance + 1))
        }
    }

    max_dist
}

pub fn solution2(input: Vec<String>) -> usize {
    let mut grid = Vec::new();
    let mut start = (usize::MAX, usize::MAX);

    for (n, line) in input.iter().enumerate() {
        let (l, s) = parse(line.to_owned());

        grid.push(l);

        if let Some(s) = s {
            start = (n, s);
        }
    }

    for (i, l) in grid.clone().iter().cloned().enumerate() {
        for (j, n) in l.iter().enumerate() {
            // println!("{i} {j}");
            if j + 1 == l.len() || (n.right && !l[j + 1].left) {
                grid[i][j].right = false;
            }
            if j == 0 || (n.left && !l[j - 1].right) {
                grid[i][j].left = false;
            }
            if i == 0 || (n.top && !grid[i - 1][j].bottom) {
                grid[i][j].top = false;
            }
            if i + 1 == grid.len() || (n.bottom && !grid[i + 1][j].top) {
                grid[i][j].bottom = false;
            }
        }
    }

    let mut nest_grid = vec![
        vec![
            Node {
                left: false,
                right: false,
                top: false,
                bottom: false
            };
            grid.first().unwrap().len()
        ];
        grid.len()
    ];
    nest_grid[start.0][start.1] = grid[start.0][start.1];
    let mut queue: VecDeque<((usize, usize), usize)> = [(start, 0)].into();
    let mut last = (start, 1);
    while !queue.is_empty() {
        let (node, distance) = queue.pop_front().unwrap();
        if (node, distance) != last {
            last = (node, distance);

            let Node {
                left,
                right,
                top,
                bottom,
            } = grid[node.0][node.1];

            if left {
                nest_grid[node.0][node.1 - 1] = grid[node.0][node.1 - 1];
                grid[node.0][node.1 - 1].right = false;
                queue.push_back(((node.0, node.1 - 1), distance + 1))
            }
            if right {
                nest_grid[node.0][node.1 + 1] = grid[node.0][node.1 + 1];
                grid[node.0][node.1 + 1].left = false;
                queue.push_back(((node.0, node.1 + 1), distance + 1))
            }
            if top {
                nest_grid[node.0 - 1][node.1] = grid[node.0 - 1][node.1];
                grid[node.0 - 1][node.1].bottom = false;
                queue.push_back(((node.0 - 1, node.1), distance + 1))
            }
            if bottom {
                nest_grid[node.0 + 1][node.1] = grid[node.0 + 1][node.1];
                grid[node.0 + 1][node.1].top = false;
                queue.push_back(((node.0 + 1, node.1), distance + 1))
            }
        }
    }

    let mut inside = 0;

    for line in nest_grid {
        let mut last_vertical = None;
        let mut outside = true;
        for n in line {
            let line = n.type_line();

            match line {
                Some(Vertical::Up) => {
                    if last_vertical.is_some() {
                        if let Some(Vertical::Down) = last_vertical {
                            outside = !outside;
                        }
                    } else {
                        last_vertical = line;
                    }
                },
                Some(Vertical::Down) => {
                    if last_vertical.is_some() {
                        if let Some(Vertical::Up) = last_vertical {
                            outside = !outside;
                        }
                    } else {
                        last_vertical = line;
                    }
                },
                Some(Vertical::Both) => outside = !outside,
                Some(Vertical::Not) => (),
                None => {
                    if !outside {
                        inside += 1;
                    }
                },
            }
        }
    }

    inside
}
