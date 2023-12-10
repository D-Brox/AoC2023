fn prediction(seq: Vec<i64>) -> i64 {
    let first = seq.first().unwrap().to_owned();

    if seq.iter().all(|&x| x == first) {
        first
    } else {
        let diff: Vec<i64> = seq
            .windows(2)
            .map(|x| if let &[a, b] = x { b - a } else { 0 })
            .collect();
        seq.last().unwrap() + prediction(diff)
    }
}

pub fn solution1(input: Vec<String>) -> i64 {
    let mut output: Vec<i64> = Vec::new();

    for line in input {
        let seq: Vec<i64> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        output.push(prediction(seq));
    }

    output.iter().sum()
}

pub fn solution2(input: Vec<String>) -> i64 {
    let mut output: Vec<i64> = Vec::new();

    for line in input {
        let mut seq: Vec<i64> = line.split(' ').map(|x| x.parse().unwrap()).collect();
        seq.reverse();
        output.push(prediction(seq));
    }

    output.iter().sum()
}
