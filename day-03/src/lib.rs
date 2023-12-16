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

fn get_substring(schematics_line: &String, border_left: usize, border_right: usize) -> String {

    let mut substring = String::new();

    for character in border_left..border_right + 1 {
        substring.push(schematics_line.chars().nth(character).unwrap());
    }

    substring
}

fn get_gear_section(schematics_line: &String, line_length: &usize, character_index: &usize) -> String {
    let border_left = std::cmp::max(0, *character_index as u32 - (*line_length as u32 + 3)) as usize;
    let border_right = std::cmp::min(schematics_line.len() as u32 - 1, *character_index as u32 + (*line_length as u32 + 3)) as usize;

    let gear_section = get_substring(schematics_line, border_left, border_right);

    gear_section
}

fn get_gear_value(gear_section: &String) -> u64 {

    // Filter string to remove imposible positions
    let middle_point = (gear_section.len() as f32 / 2f32).floor() as usize;
    let filtered_string = get_substring(gear_section, 0, 6) + &get_substring(gear_section, middle_point - 3, middle_point + 2) + &get_substring(gear_section, gear_section.len() - 8, gear_section.len() - 1);

    let mut numbers: Vec<u64> = Vec::new();
    let mut actual_number = 0;
    filtered_string
        .chars()
        .into_iter()
        .for_each(|character| {
            if character.to_string().parse::<u64>().is_err() {
                if actual_number != 0 {
                    numbers.push(actual_number);
                }

                actual_number = 0;
            }

            actual_number = actual_number * 10 + character.to_string().parse::<u64>().unwrap_or(0);
        });

    let mut first_number = 0;
    let mut final_number = 0;

    for number in 0..numbers.len() {

        if number <= 9 {
            continue;
        }

        if first_number == 0 {
            first_number = numbers[number];
        }

        final_number = numbers[number];
    };

    println!("{}", filtered_string);
    println!("{:?}", numbers);
    if first_number == final_number {
        return 0;
    }

    first_number * final_number
}

pub fn get_gear_ratio(schematics: String) -> u64 {
    let mut result = 0;

    let line_length: usize = schematics.split("\n").collect::<Vec<&str>>()[0].len();
    let schematics_line: String = schematics.replace("\n", "");

    for character_index in 0..schematics_line.len() {
        let character = schematics_line.chars().nth(character_index).unwrap();

        if character != '*' {
            continue;
        }

        let gear_section = get_gear_section(&schematics_line, &line_length, &character_index);
        let gear_value = get_gear_value(&gear_section);

        result += gear_value;
    }

    result
}
