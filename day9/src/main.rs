fn main() {
    let lines = io::read_lines("day9/input");

    // Part 1
    let mut sum = 0;
    for line in &lines {
        let nums: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut rows = vec![nums];
        loop {
            let mut row = Vec::new();
            let last_row = rows.last().unwrap();
            if last_row.iter().all(|n| *n == 0) {
                break;
            }
            for i in 1..last_row.len() {
                row.push(last_row[i] - last_row[i - 1]);
            }
            rows.push(row);
        }
        let extrapolation = rows.iter().fold(0, |acc, row| row.last().unwrap() + acc);
        sum += extrapolation;
    }
    println!("{}", sum);

    // Part 2
    let mut sum = 0;
    for line in lines {
        let nums: Vec<i32> = line
            .split_ascii_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let mut rows = vec![nums];
        loop {
            let mut row = Vec::new();
            let last_row = rows.last().unwrap();
            if last_row.iter().all(|n| *n == 0) {
                break;
            }
            for i in 1..last_row.len() {
                row.push(last_row[i] - last_row[i - 1]);
            }
            rows.push(row);
        }
        let extrapolation = rows.iter().rev().fold(0, |acc, row| row[0] - acc);
        sum += extrapolation;
    }
    println!("{}", sum);
}
