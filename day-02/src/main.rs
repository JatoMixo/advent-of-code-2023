use day_02::{select_games_valid, get_power_of_games, Cubes};

fn main() {

    const PART_TWO: bool = true;

    let input = std::fs::read_to_string("game_data.txt").expect("Forgot the file...");
    
    if !PART_TWO {
        let mut result = 0;

        select_games_valid(input, Cubes {red: 12, green: 13, blue: 14}).into_iter().for_each(|game| {
            result += game;
        });

        println!("{}", result);
        
        return;
    }

    println!("{}", get_power_of_games(input));
}