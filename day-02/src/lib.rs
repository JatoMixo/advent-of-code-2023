#[derive(Debug, PartialEq)]
pub struct Cubes {
    pub red: u32,
    pub blue: u32,
    pub green: u32,
}

impl Cubes {

    pub fn new() -> Cubes {
        Cubes {
            red: 0,
            green: 0,
            blue: 0,
        }
    }

    pub fn set_color_by_string(&mut self, color: &str, value: u32) {
        match color {
            "red" => self.red = value,
            "green" => self.green = value,
            "blue" => self.blue = value,
            _ => println!("You fucked up the color ={}=", color),
        };
    }

    pub fn from_string(string: String) -> Cubes {
        
        let mut result = Cubes::new();

        let split_string = string.split(", ").collect::<Vec<&str>>();

        split_string.into_iter().for_each(|section| {
            let color_parsed = section.split(" ").collect::<Vec<&str>>();

            let color = color_parsed[1].to_lowercase();
            result.set_color_by_string(color.trim(), color_parsed[0].parse::<u32>().unwrap());
        });

        result
    }
}

#[derive(Debug, PartialEq)]
pub struct Game {
    pub id: u32,
    pub cubes_shown: Vec<Cubes>,
}

impl Game {
    pub fn new() -> Game {
        Game {
            id: 0,
            cubes_shown: Vec::new()
        }
    }

    pub fn from_string(string: String) -> Game {

        let mut result = Game::new();

        let main_split = string.split(": ").collect::<Vec<&str>>();

        result.id = Game::get_game_id_from_string(main_split[0]);
        result.cubes_shown = Game::get_cubes_shown_from_string(main_split[1]);

        result
    }

    fn get_game_id_from_string(string: &str) -> u32 {
        string.split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap()
    }

    fn get_cubes_shown_from_string(string: &str) -> Vec<Cubes> {

        let mut result = Vec::new();

        let sections = string.split("; ").collect::<Vec<&str>>();
        sections.into_iter().for_each(|cube_section| {
            result.push(Cubes::from_string(cube_section.to_string()));
        });

        result
    }

    pub fn is_game_valid_with(&self, cubes: &Cubes) -> bool {
        for cube_index in 0..self.cubes_shown.len() {
            if cubes.red < self.cubes_shown[cube_index].red || cubes.green < self.cubes_shown[cube_index].green || cubes.blue < self.cubes_shown[cube_index].blue {
                return false;
            }
        };

        true
    }

    pub fn get_power(&self) -> u32 {
        let mut result = Cubes::new();

        self.cubes_shown.iter().for_each(|cubes| {
            result.red = std::cmp::max(result.red, cubes.red);
            result.green = std::cmp::max(result.green, cubes.green);
            result.blue = std::cmp::max(result.blue, cubes.blue);
        });

        result.red * result.blue * result.green
    }
}

pub fn select_games_valid(games_string: String, cubes: Cubes) -> Vec<u32> {

    let mut result = Vec::new();

    let games = games_string.split("\r\n").collect::<Vec<&str>>();
    games.into_iter().for_each(|games_string| {
        let game = Game::from_string(games_string.to_string());
        if game.is_game_valid_with(&cubes) {
            result.push(game.id);
        }
    });

    result
}

pub fn get_power_of_games(games_string: String) -> u32 {
    let mut result = 0;

    games_string
        .split("\r\n")
        .for_each(|game_string| {
            let game = Game::from_string(game_string.to_string());
            result += game.get_power();
        });

    result
}
