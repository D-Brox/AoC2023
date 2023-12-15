use std::collections::HashMap;

#[derive(Hash, Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Platform {
    Space,
    Round,
    Cube,
}

fn split_vec(vec: &Vec<Platform>) -> Vec<&[Platform]> {
    let mut result = Vec::new();
    let mut start = 0;

    for (i, item) in vec.iter().enumerate() {
        if item == &Platform::Cube {
            if start <= i {
                result.push(&vec[start..i]);
                result.push(&[Platform::Cube]);
            }
            start = i + 1;
        }
    }

    if start < vec.len() {
        result.push(&vec[start..]);
    }
    result
}

fn transpose(mut v: Vec<Vec<Platform>>) -> Vec<Vec<Platform>> {
    for inner in &mut v {
        inner.reverse();
    }

    (0..v[0].len())
        .map(|_| v.iter_mut().map(|inner| inner.pop().unwrap()).collect())
        .collect()
}

fn tilt(platform: Vec<Vec<Platform>>, right: bool) -> Vec<Vec<Platform>> {
    let mut tilted = Vec::new();

    for line in platform {
        let splits = split_vec(&line);
        tilted.push(
            splits
                .iter()
                .flat_map(|&s| {
                    let mut s = s.to_vec();
                    s.sort();
                    if right {
                        s.to_vec()
                    } else {
                        s.reverse();
                        s.to_vec()
                    }
                })
                .collect(),
        );
    }

    tilted
}

fn parse(input: Vec<String>) -> Vec<Vec<Platform>> {
    let mut output = Vec::new();

    for line in input {
        let mut l = Vec::new();
        for c in line.chars() {
            match c {
                '.' => l.push(Platform::Space),
                'O' => l.push(Platform::Round),
                '#' => l.push(Platform::Cube),
                _ => unreachable!(),
            }
        }
        output.push(l);
    }

    transpose(output)
}

fn count(platform: Vec<Vec<Platform>>) -> usize {
    let platform = transpose(platform);

    let mut count = 0;
    for (n, line) in platform.iter().enumerate() {
        count += (platform.len() - n)
            * line
                .iter()
                .fold(0, |a, &b| a + if b == Platform::Round { 1 } else { 0 })
    }
    count
}

pub fn solution1(input: Vec<String>) -> usize {
    let mut platform = parse(input);
    platform = tilt(platform, false);
    
    count(platform)
}

pub fn solution2(input: Vec<String>) -> usize {
    let mut platform = parse(input);

    let mut platmap: HashMap<Vec<Vec<Platform>>, usize> = HashMap::new();
    let mut idxmap: HashMap<usize, usize> = HashMap::new();

    for i in 0..1000000000 {
        for j in 0..4 {
            platform = tilt(platform, j >> 1 == 1);
            platform = transpose(platform);
        }

        // Check if it's repeating cycle :D
        if let Some(&n) = platmap.get(&platform) {
            return idxmap
                .get(&(n + (1000000000 - i - 1) % (i - n)))
                .unwrap()
                .to_owned();
        }

        let count = count(platform.clone());
        platmap.insert(platform.clone(), i);
        idxmap.insert(i, count);
    }
    count(platform)
}
