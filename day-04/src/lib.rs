use std::collections::HashMap;

fn get_game_id(game: &str) -> u64 {
    let id_section = game.split(": ").collect::<Vec<&str>>()[0];

    id_section
        .replace("   ", " ")
        .replace("  ", " ")
        .split(" ")
        .collect::<Vec<&str>>()[1]
        .parse::<u64>()
        .expect(&format!("Game ID Terribly parsed: {id_section}"))
}

fn get_winner_numbers(game: &str) -> Vec<u64> {
    let left_section = game.split(" | ").collect::<Vec<&str>>()[0];
    let winners_section = left_section.split(": ").collect::<Vec<&str>>()[1];

    let mut winner_numbers = Vec::new();
    winners_section.split(" ").for_each(|winner| {
        if !winner.is_empty() {
            winner_numbers.push(winner.parse::<u64>().expect("Terribly parsed winner number"));
        }
    });

    winner_numbers
}

fn get_card_numbers(game: &str) -> Vec<u64> {
    let numbers_section = game.split(" | ").collect::<Vec<&str>>()[1];
    let mut card_numbers = Vec::new();

    numbers_section.split(" ").collect::<Vec<&str>>().iter().for_each(|card_number| {
        if !card_number.is_empty() {
            card_numbers.push(card_number.trim().parse::<u64>().expect(&format!("Terribly parsed Card Number: ={}=", card_number)));
        }
    });

    card_numbers
}

fn get_value_of_card(winner_numbers: Vec<u64>, card_numbers: Vec<u64>) -> u64 {
    let correct_numbers = winner_numbers
        .into_iter()
        .filter(|winner_number| card_numbers.contains(winner_number))
        .collect::<Vec<u64>>();

    if correct_numbers.len() == 0 {
        return 0;
    }

    2u64.pow(correct_numbers.len() as u32 - 1)
}

pub fn get_total_value(cards: String) -> u64 {
    let mut total_value = 0;
    let cards_vec = cards.split("\n").collect::<Vec<&str>>();

    cards_vec.iter().for_each(|&game| {
        let winner_numbers = get_winner_numbers(game);
        let card_numbers = get_card_numbers(game);

        total_value += get_value_of_card(winner_numbers, card_numbers);
    });

    total_value
}

fn get_correct_numbers(winner_numbers: Vec<u64>, card_numbers: Vec<u64>) -> u64 {
    card_numbers
        .iter()
        .filter(|&number| winner_numbers.contains(number))
        .collect::<Vec<&u64>>()
        .len() as u64
}

pub fn get_stacked_value(cards: String) -> u64 {
    let mut cards_to_calculate: HashMap<u64, u64> = HashMap::new();

    let cards_vec = cards.split("\n").collect::<Vec<&str>>();

    for card in 0..cards_vec.len() {
        let game = cards_vec[card];

        let game_id = get_game_id(game);
        let winner_numbers = get_winner_numbers(game);
        let card_numbers = get_card_numbers(game);

        let correct_numbers = get_correct_numbers(winner_numbers, card_numbers);

        cards_to_calculate.entry(game_id).and_modify(|quantity| *quantity += 1).or_insert(1);
        for number in 1..(correct_numbers + 1) {
            let card_increase = *cards_to_calculate.get(&game_id).unwrap();
            cards_to_calculate.entry(number + game_id).and_modify(|quantity| *quantity += card_increase).or_insert(card_increase);
        }
    }

    cards_to_calculate
        .values()
        .sum::<u64>()
}
