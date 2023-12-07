use day_02::{Cubes, Game};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cubes_parsing() {
        assert_eq!(Cubes::from_string("3 blue, 4 red".to_string()), Cubes { blue: 3, red: 4, green: 0 });
        assert_eq!(Cubes::from_string("1 red, 2 green, 6 blue".to_string()), Cubes { blue: 6, red: 1, green: 2 });

        assert_eq!(Cubes::from_string("1 blue".to_string()), Cubes { blue: 1, red: 0, green: 0 });
        assert_eq!(Cubes::from_string("6 red, 1 blue, 3 green".to_string()), Cubes { red: 6, blue: 1, green: 3 });
    }

    #[test]
    fn test_game_parsing() {
        assert_eq!(Game::from_string("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()),
            Game { id: 1, cubes_shown: vec![Cubes {blue: 3, red: 4, green: 0}, Cubes {red: 1, green: 2, blue: 6}, Cubes {green: 2, blue: 0, red: 0 }] });
        
        assert_eq!(Game::from_string("Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string()),
            Game { id: 4, cubes_shown: vec![Cubes {green: 1, red: 3, blue: 6}, Cubes {red: 6, green: 3, blue: 0}, Cubes {green: 3, blue: 15, red: 14}] });
    }

    #[test]
    fn test_is_game_valid() {
        let game = Game::from_string("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string());
        
        assert_eq!(game.is_game_valid_with(Cubes { red: 12, green: 13, blue: 14 }), true);
        assert_eq!(game.is_game_valid_with(Cubes { red: 0, green: 13, blue: 14 }), false);
    }
}