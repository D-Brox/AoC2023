use std::hash::{Hash, Hasher};

struct HolidayHasher {
    state: u8,
}

impl HolidayHasher {
    fn new() -> Self { HolidayHasher { state: 0 } }
}

impl Hasher for HolidayHasher {
    fn write(&mut self, bytes: &[u8]) {
        for &byte in bytes {
            self.state += byte;
            self.state *= 17;
        }
    }

    fn finish(&self) -> u64 { self.state as u64 }
}

struct Step<'a>(&'a [u8]);

impl Hash for Step<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) { state.write(self.0); }
}

pub fn solution1(input: Vec<String>) -> u64 {
    input[0]
        .split(',')
        .map(|s| {
            let x = Step(s.as_bytes());
            let mut hasher = HolidayHasher::new();
            x.hash(&mut hasher);
            hasher.finish()
        })
        .sum()
}

pub fn solution2(input: Vec<String>) -> u64 {
    let mut hashmap: Vec<Vec<(&str, u64)>> = vec![Vec::new(); 256];
    for s in input[0].split(',') {
        let (s, step, n) = if let Some((s, n)) = s.split_once('=') {
            (s, Step(s.as_bytes()), n.parse().ok())
        } else {
            let s = s.strip_suffix('-').unwrap();
            (s, Step(s.as_bytes()), None)
        };

        let mut hasher = HolidayHasher::new();
        step.hash(&mut hasher);
        let hash = hasher.finish() as usize;

        let lbox = hashmap.get_mut(hash).unwrap();

        if let Some(n) = n {
            if let Some(idx) = lbox.iter().position(|(k, _)| k == &s) {
                lbox[idx] = (s, n);
            } else {
                lbox.push((s, n));
            }
        } else if let Some(idx) = lbox.iter().position(|(k, _)| k == &s) {
            lbox.remove(idx);
        }
    }

    hashmap
        .iter()
        .enumerate()
        .map(|(i, lbox)| {
            let mut power = 0;
            for (j, (_, lens)) in lbox.iter().enumerate() {
                power += ((i + 1) as u64) * ((j + 1) as u64) * lens;
            }
            power
        })
        .sum()
}
