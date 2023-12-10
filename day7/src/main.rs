use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let lines = io::read_lines("day7/input");
    let mut entries: Vec<(Hand, usize)> = Vec::new();
    for line in lines {
        let (hand, bid) = line.split_once(' ').unwrap();
        let bid = bid.parse().unwrap();
        let card = Hand::new(hand.to_owned());
        entries.push((card, bid));
    }
    entries.sort();
    dbg!(&entries);
    let mut total_winnings = 0;
    for (rank, (_, bid)) in entries.iter().enumerate() {
        total_winnings += (rank + 1) * bid
    }
    println!("{}", total_winnings);
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Card {
    J,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    T,
    Q,
    K,
    A,
}

impl Card {
    fn new(card: char) -> Card {
        match card {
            '2' => Self::Two,
            '3' => Self::Three,
            '4' => Self::Four,
            '5' => Self::Five,
            '6' => Self::Six,
            '7' => Self::Seven,
            '8' => Self::Eight,
            '9' => Self::Nine,
            'T' => Self::T,
            'J' => Self::J,
            'Q' => Self::Q,
            'K' => Self::K,
            'A' => Self::A,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

impl HandType {
    fn new(cards: String) -> HandType {
        let hs: HashSet<char> = cards.chars().collect();
        let mut count = Vec::new();
        for card in &hs {
            let n = cards.chars().filter(|c| c == card).count();
            count.push((card, n));
        }
        let mut count: Vec<_> = count.iter().map(|(_, n)| n).collect();
        count.sort();
        //dbg!(&count);
        let hand_type = match &count[..] {
            [5] => Self::FiveOfAKind,
            [1, 4] => Self::FourOfAKind,
            [2, 3] => Self::FullHouse,
            [1, 1, 3] => Self::ThreeOfAKind,
            [1, 2, 2] => Self::TwoPair,
            [1, 1, 1, 2] => Self::OnePair,
            [1, 1, 1, 1, 1] => Self::HighCard,
            _ => unreachable!(),
        };
        let num_j = cards.chars().filter(|c| *c == 'J').count();
        match (hand_type, num_j) {
            (_, 0) => hand_type,
            (Self::FourOfAKind, _) => Self::FiveOfAKind,
            (Self::FullHouse, _) => Self::FiveOfAKind,
            (Self::ThreeOfAKind, _) => Self::FourOfAKind,
            (Self::TwoPair, 1) => Self::FullHouse,
            (Self::TwoPair, 2) => Self::FourOfAKind,
            (Self::OnePair, _) => Self::ThreeOfAKind,
            (Self::HighCard, _) => Self::OnePair,
            _ => hand_type,
        }
    }
}

impl Hand {
    fn new(cards: String) -> Hand {
        let hand_type = HandType::new(cards.clone());
        let cards: Vec<_> = cards.chars().map(Card::new).collect();
        Hand { cards, hand_type }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.hand_type.cmp(&other.hand_type) {
            Ordering::Equal => self.cards.cmp(&other.cards),
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
