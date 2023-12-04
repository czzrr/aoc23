use std::collections::HashSet;

fn main() {
    let lines = io::read_lines("day4/input");

    let mut cards: Vec<(HashSet<usize>, HashSet<usize>)> = Vec::new();
    for line in &lines {
        let (winning, having) = line.split_once(':').unwrap().1.split_once('|').unwrap();
        let winning =
            HashSet::from_iter(winning.split_ascii_whitespace().map(|s| s.parse().unwrap()));
        let having =
            HashSet::from_iter(having.split_ascii_whitespace().map(|s| s.parse().unwrap()));
        cards.push((winning, having));
    }

    // Part 1
    let mut total_score = 0;
    for (winning, having) in &cards {
        let num_common = winning.intersection(&having).count();
        let score = if num_common == 0 {
            0
        } else {
            2_usize.pow(num_common as u32 - 1)
        };
        total_score += score;
    }
    println!("{}", total_score);

    // Part 2
    let mut total_num_cards = 0;
    let mut card_counts = vec![1; lines.len()];
    for (i, (winning, having)) in cards.iter().enumerate() {
        total_num_cards += card_counts[i];
        let num_common = winning.intersection(&having).count();
        for j in 1..=num_common {
            card_counts[i + j] += card_counts[i];
        }
    }
    println!("{}", total_num_cards);
}
