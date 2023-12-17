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

    valid_numbers.iter().sum()
}

fn get_gear_value(gear_numbers: Vec<u64>) -> u64 {
    // A gear has exactly two adjacent numbers, so if the length is different than 2, is not a gear.
    if gear_numbers.len() != 2 {
        return 0;
    }

    gear_numbers[0] * gear_numbers[1]
}

fn get_substring(schematics_line: &String, border_left: usize, border_right: usize) -> String {

    let mut substring = String::new();

    for character in border_left..border_right {
        substring.push(schematics_line.chars().nth(character).unwrap());
    }

    substring
}

// TODO: Refactor these 3 functions into one to remove duplicate code
fn get_upper_section(schematics_vec: &Vec<&str>, line_index: usize, character_index: usize) -> String {
    if line_index == 0 {
        return String::new();
    }

    let maximum_left = std::cmp::max(character_index as i32 - 3, 0) as usize;
    let maximum_right = std::cmp::min(character_index as i32 + 4, schematics_vec[0].len() as i32) as usize;
    get_substring(&schematics_vec[line_index - 1].to_string(), maximum_left, maximum_right)
}

fn get_middle_section(schematics_vec: &Vec<&str>, line_index: usize, character_index: usize) -> String {
    let maximum_left = std::cmp::max(character_index as i32 - 3, 0) as usize;
    let maximum_right = std::cmp::min(character_index as i32 + 4, schematics_vec[0].len() as i32) as usize;

    get_substring(&schematics_vec[line_index].to_string(), maximum_left, maximum_right)
}

fn get_lower_section(schematics_vec: &Vec<&str>, line_index: usize, character_index: usize) -> String {
    if line_index == schematics_vec.len() - 1 {
        return String::new();
    }

    let maximum_left = std::cmp::max(character_index as i32 - 3, 0) as usize;
    let maximum_right = std::cmp::min(character_index as i32 + 4, schematics_vec[0].len() as i32) as usize;

    get_substring(&schematics_vec[line_index + 1].to_string(), maximum_left, maximum_right)
}

fn get_numbers_in_line(line: String) -> Vec<u64> {
    let mut result = Vec::new();

    let mut actual_number = 0;
    for number_index in 0..line.len() {
        match line.chars().nth(number_index).unwrap().to_string().parse::<u64>() {
            Err(_) => {
                if number_index <= 2 {
                    actual_number = 0;
                    continue;
                }

                if actual_number != 0 {
                    result.push(actual_number);
                    actual_number = 0;
                }
            },
            Ok(value) => {
                actual_number = actual_number * 10 + value;
            }
        }
    }

    if actual_number > 99 {
        result.push(actual_number);
    }

    result
}

fn get_gear_numbers(schematics_vec: &Vec<&str>, line_index: usize, character_index: usize) -> Vec<u64> {
    let mut result: Vec<u64> = Vec::new();

    let upper_section = get_upper_section(schematics_vec, line_index, character_index);
    let middle_section = get_middle_section(schematics_vec, line_index, character_index);
    let lower_section = get_lower_section(schematics_vec, line_index, character_index);

    result.extend(get_numbers_in_line(upper_section));
    result.extend(get_numbers_in_line(middle_section));
    result.extend(get_numbers_in_line(lower_section));

    result
}

pub fn get_gear_ratio(schematics: String) -> u64 {
    let mut result = 0;

    let schematics_vec = schematics.split("\n").collect::<Vec<&str>>();
    for line_index in 0..schematics_vec.len() {

        let line = schematics_vec[line_index];

        for character_index in 0..line.len() {
            let character = line.chars().nth(character_index).unwrap().to_string();

            if character != "*" {
                continue;
            }

            let gear_numbers = get_gear_numbers(&schematics_vec, line_index, character_index);
            let gear_value = get_gear_value(gear_numbers);

            result += gear_value;
        }
    }

    result
}
