fn main() {
    let lines = io::read_lines("day5/input");
    let seeds: Vec<usize> = lines[0]
        .split_once(':')
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    let mut maps = Vec::new();
    let mut lines = &lines[..];
    for _ in 0..7 {
        let (lines_next, entries) = parse_map(lines);
        lines = lines_next;
        maps.push(Map { entries });
    }
    //dbg!(&maps);

    let mut seeds = seeds;
    let mut actual_seeds = Vec::new();
    actual_seeds.extend(seeds[0]..seeds[0] + seeds[1]);
    actual_seeds.extend(seeds[2]..seeds[2] + seeds[3]);
    seeds = actual_seeds;
    for map in maps {
        for s in seeds.iter_mut() {
            *s = map.lookup(*s);
        }
    }
    let answer = seeds.iter().min().unwrap();
    println!("{}", answer);
}

#[derive(Debug)]
struct Map {
    entries: Vec<Entry>,
}

impl Map {
    pub fn lookup(&self, n: usize) -> usize {
        for entry in &self.entries {
            match entry.lookup(n) {
                Some(n) => return n,
                None => (),
            }
        }
        n
    }
}

#[derive(Debug)]
struct Entry {
    dst: usize,
    src: usize,
    cnt: usize,
}

impl Entry {
    pub fn lookup(&self, n: usize) -> Option<usize> {
        if self.src <= n && n <= self.src + self.cnt {
            Some(self.dst + (n - self.src))
        } else {
            None
        }
    }
}

fn parse_map(lines: &[String]) -> (&[String], Vec<Entry>) {
    let mut i = 0;
    while !lines[i].contains("map:") {
        i += 1;
    }
    i += 1;
    let mut entries = Vec::new();
    while i < lines.len() && !lines[i].is_empty() {
        let nums: Vec<usize> = lines[i]
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let dst = nums[0];
        let src = nums[1];
        let cnt = nums[2];
        entries.push(Entry { dst, src, cnt });
        i += 1;
    }
    (&lines[i..], entries)
}
