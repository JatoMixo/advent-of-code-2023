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
            _ => println!("You fucked up the color, man"),
        };
    }

    pub fn from_string(string: String) -> Cubes {
        
        let mut result = Cubes::new();

        let split_string = string.split(", ").collect::<Vec<&str>>();

        split_string.into_iter().for_each(|section| {
            let color_parsed = section.split(" ").collect::<Vec<&str>>();

            result.set_color_by_string(color_parsed[1], color_parsed[0].parse::<u32>().unwrap());
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

    pub fn is_game_valid_with(&self, cubes: Cubes) -> bool {
        let mut total_cubes = Cubes::new();

        for cube_index in 0..self.cubes_shown.len() {
            total_cubes.red += self.cubes_shown[cube_index].red;
            total_cubes.green += self.cubes_shown[cube_index].green;
            total_cubes.blue += self.cubes_shown[cube_index].blue;
        };

        total_cubes.red <= cubes.red && total_cubes.blue <= cubes.blue && total_cubes.green <= cubes.green
    }
}
