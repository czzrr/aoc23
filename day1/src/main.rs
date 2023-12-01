fn main() {
    let lines = io::read_lines("day1/input");

    // Part 1
    let result: u32 = lines
        .iter()
        .map(|s| {
            let digits = s.chars().filter(|c| c.is_ascii_digit());
            let first_digit = digits.clone().next().unwrap().to_digit(10).unwrap();
            let last_digit = digits.rev().next().unwrap().to_digit(10).unwrap();
            10 * first_digit + last_digit
        })
        .sum();
    println!("{}", result);

    // Part 2
    fn letters_to_digit(letters: &[u8]) -> Option<u32> {
        let letters = letters.to_owned();
        match () {
            _ if letters.starts_with(b"one") => Some(1),
            _ if letters.starts_with(b"two") => Some(2),
            _ if letters.starts_with(b"three") => Some(3),
            _ if letters.starts_with(b"four") => Some(4),
            _ if letters.starts_with(b"five") => Some(5),
            _ if letters.starts_with(b"six") => Some(6),
            _ if letters.starts_with(b"seven") => Some(7),
            _ if letters.starts_with(b"eight") => Some(8),
            _ if letters.starts_with(b"nine") => Some(9),
            _ => None,
        }
    }

    let mut result = 0;
    for line in lines {
        let line = line.as_bytes();
        let mut digits = Vec::new();
        if line.len() < 3 {
            for i in 0..line.len() {
                if line[i].is_ascii_digit() {
                    digits.push((line[i] as char).to_digit(10).unwrap());
                }
            }
        } else {
            for i in 0..line.len() - 2 {
                if line[i].is_ascii_digit() {
                    digits.push((line[i] as char).to_digit(10).unwrap());
                } else {
                    match letters_to_digit(&line[i..(i + 5).min(line.len())]) {
                        Some(d) => digits.push(d),
                        _ => (),
                    }
                }
            }
            for i in line.len() - 2..line.len() {
                if line[i].is_ascii_digit() {
                    digits.push((line[i] as char).to_digit(10).unwrap());
                }
            }
        }
        let first_digit = digits.first().unwrap();
        let last_digit = digits.last().unwrap();
        result += 10 * first_digit + last_digit;
    }
    println!("{}", result);
}
