fn main() {
    let lines = io::read_lines("day2/input");

    let mut games = Vec::new();
    for line in lines {
        games.push(parse_game(&line));
    }

    // Part 1
    let mut sum_of_ids = 0;
    for game in &games {
        if game.is_possible() {
            sum_of_ids += game.id;
        }
    }
    println!("{}", sum_of_ids);

    // Part 2
    let mut sum_of_powers = 0;
    for game in games {
        let mut red = usize::MIN;
        let mut green = usize::MIN;
        let mut blue = usize::MIN;
        for set in game.sets {
            for (color, count) in set {
                match color {
                    Color::Red => red = red.max(count),
                    Color::Green => green = green.max(count),
                    Color::Blue => blue = blue.max(count),
                };
            }
        }
        sum_of_powers += red * green * blue;
    }
    println!("{}", sum_of_powers);
}

#[derive(Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
struct Game {
    pub id: usize,
    sets: Vec<Vec<(Color, usize)>>,
}

impl Game {
    pub fn is_possible(&self) -> bool {
        for set in &self.sets {
            for (color, count) in set {
                let max_count = match color {
                    Color::Red => 12,
                    Color::Green => 13,
                    Color::Blue => 14,
                };
                if *count > max_count {
                    return false;
                }
            }
        }
        true
    }
}

fn parse_game(s: &str) -> Game {
    let split: Vec<_> = s.split(':').collect();
    let id = split[0]
        .split_ascii_whitespace()
        .nth(1)
        .unwrap()
        .parse()
        .unwrap();

    let mut sets = Vec::new();
    for set in split[1].split(';') {
        let mut cubes = Vec::new();
        for cube_occ in set.split(',') {
            cubes.push(parse_cube_occ(cube_occ));
        }
        sets.push(cubes);
    }
    Game { id, sets }
}

fn parse_cube_occ(s: &str) -> (Color, usize) {
    let split: Vec<_> = s.split_ascii_whitespace().collect();
    let count = split[0].parse().unwrap();
    let color = match split[1] {
        "red" => Color::Red,
        "green" => Color::Green,
        "blue" => Color::Blue,
        _ => unreachable!(),
    };
    (color, count)
}
