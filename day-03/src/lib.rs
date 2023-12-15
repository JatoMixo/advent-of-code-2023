fn is_valid_symbol(character: char) -> bool {
    !character.is_alphanumeric() && character != '.' && character.to_string().trim() == character.to_string()
}

pub fn calculate_schematic(schematic: String) -> u32 {
    let mut valid_numbers: Vec<u32> = Vec::new();
    let schematic_lines = schematic.split("\n").collect::<Vec<&str>>();

    for line_index in 0..schematic_lines.len() {
        
        let line = schematic_lines[line_index];
        let mut has_symbol = false;
        let mut actual_number = 0;

        for character_index in 0..line.len() {

            let character = line.chars().nth(character_index).unwrap();

            match character.to_string().parse::<u32>() {
                Err(_) => {
                    if has_symbol {
                        valid_numbers.push(actual_number);
                    }

                    actual_number = 0;
                    has_symbol = false;
                },
                Ok(value) => {
                    actual_number = (actual_number * 10) + value;

                    // Check lateral
                    let left_index_char = std::cmp::max(character_index as i32 - 1, 0);
                    let left_character = line.chars().nth(left_index_char as usize).unwrap_or('.');
                    let right_character = line.chars().nth(character_index + 1).unwrap_or('.');

                    has_symbol |= is_valid_symbol(left_character) || is_valid_symbol(right_character);

                    // Check up and diagonal
                    let left_index_line = std::cmp::max(line_index as i32 - 1, 0);
                    let up_line = *schematic_lines.get(left_index_line as usize).unwrap_or(&"...");
                    let down_line = *schematic_lines.get(line_index + 1).unwrap_or(&"...");

                    for check in 0..3 {
                        let char_index = std::cmp::max(check as i32 - 1 + character_index as i32, 0);
                        let char_up = up_line.chars().nth(char_index as usize).unwrap_or('.');
                        let char_down = down_line.chars().nth(char_index as usize).unwrap_or('.');

                        has_symbol |= is_valid_symbol(char_up) || is_valid_symbol(char_down);
                    }
                },
            }
        };
    };

    println!("{}", valid_numbers[2]);
    valid_numbers.iter().sum()
}