enum CardType {
    FiveKind,
    FourKind,
    FullHouse,
    ThreeKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl CardType {
    pub fn score(&self) -> u64 {
        match self {
            CardType::FiveKind => 7,
            CardType::FourKind => 6,
            CardType::FullHouse => 5,
            CardType::ThreeKind => 4,
            CardType::TwoPair => 3,
            CardType::OnePair => 2,
            CardType::HighCard => 1,
        }
    }
}

fn calculate_winning(cards: String) -> u64 {
    0
}

fn main() {
    let input = std::fs::read_to_string("input.txt").unwrap();

    println!("Result 1: {}", calculate_winning(input));
}
