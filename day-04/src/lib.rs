fn get_game_id(game: &str) -> u32 {
    let id_section = game.split(": ").collect::<Vec<&str>>()[0];

    id_section.split(" ").collect::<Vec<&str>>()[1].parse::<u32>().expect("Game ID Terribly parsed")
}

fn get_winner_numbers(game: &str) -> Vec<u32> {
    let left_section = game.split(" | ").collect::<Vec<&str>>()[0];
    let winners_section = left_section.split(": ").collect::<Vec<&str>>()[1];

    let mut winner_numbers = Vec::new();
    winners_section.split(" ").for_each(|winner| {
        if !winner.is_empty() {
            winner_numbers.push(winner.parse::<u32>().expect("Terribly parsed winner number"));
        }
    });

    winner_numbers
}

fn get_card_numbers(game: &str) -> Vec<u32> {
    let numbers_section = game.split(" | ").collect::<Vec<&str>>()[1];
    let mut card_numbers = Vec::new();

    numbers_section.split(" ").collect::<Vec<&str>>().iter().for_each(|card_number| {
        if !card_number.is_empty() {
            card_numbers.push(card_number.trim().parse::<u32>().expect(&format!("Terribly parsed Card Number: ={}=", card_number)));
        }
    });

    card_numbers
}

fn get_value_of_card(winner_numbers: Vec<u32>, card_numbers: Vec<u32>) -> u32 {
    let correct_numbers = winner_numbers
        .into_iter()
        .filter(|winner_number| card_numbers.contains(winner_number))
        .collect::<Vec<u32>>();

    if correct_numbers.len() == 0 {
        return 0;
    }

    2u32.pow(correct_numbers.len() as u32 - 1)
}

pub fn get_total_value(cards: String) -> u32 {
    let mut total_value = 0;
    let cards_vec = cards.split("\n").collect::<Vec<&str>>();

    cards_vec.iter().for_each(|&game| {
        // let game_id = get_game_id(game);
        let winner_numbers = get_winner_numbers(game);
        let card_numbers = get_card_numbers(game);

        total_value += get_value_of_card(winner_numbers, card_numbers);
    });

    total_value
}