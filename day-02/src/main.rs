use day_02::{select_games_valid, Cubes};

fn main() {
    let input = std::fs::read_to_string("game_data.txt").expect("Forgot the file...");

    println!("{:?}", select_games_valid(input, Cubes {red: 12, green: 13, blue: 14}).into_iter().sum::<u32>());
}