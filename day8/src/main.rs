use std::collections::HashMap;

fn main() {
    let lines = io::read_lines("day8/input");
    let directions = &lines[0];
    let mut map = HashMap::new();
    for line in &lines[2..] {
        let (from, to) = line.split_once('=').unwrap();
        let from = from.trim_end().to_owned();
        let (to_left, to_right) = to.split_once(',').unwrap();
        let to_left = to_left.replace(" (", "");
        let to_right = to_right.trim().replace(')', "");
        map.insert(from, (to_left, to_right));
    }

    // Part 1
    let mut count = 0;
    let mut location = &"AAA".to_owned();
    for direction in directions.chars().cycle() {
        if location == "ZZZ" {
            break;
        }
        match direction {
            'L' => location = &map.get(location).unwrap().0,
            'R' => location = &map.get(location).unwrap().1,
            _ => unreachable!(),
        }
        count += 1;
    }
    println!("{}", count);

    // Part 2
    let mut distances = Vec::new();
    for start in map.keys().filter(|s| s.ends_with('A')) {
        let mut location = start;
        let mut num_steps = 0;
        for direction in directions.chars().cycle() {
            if location.ends_with('Z') {
                distances.push(num_steps);
                break;
            }
            let next_location = match direction {
                'L' => &map.get(location).unwrap().0,
                'R' => &map.get(location).unwrap().1,
                _ => unreachable!(),
            };
            location = next_location;
            num_steps += 1;
        }
    }
    let least_common_multiple = distances[1..]
        .iter()
        .fold(distances[0], |acc, n| lcm(acc, *n));
    println!("{}", least_common_multiple);
}

fn gcd(a: usize, b: usize) -> usize {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: usize, b: usize) -> usize {
    a * b / gcd(a, b)
}
