use std::collections::HashMap;

fn remove_last_char(string: String) -> String {
    let mut chars = string.chars();
    chars.next_back();

    chars.as_str().to_string()
}

pub fn calculate_calibration(calibration_string: String) -> u32 {

    let numbers: HashMap<&str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    let mut digits: Vec<u32> = Vec::new();
    let mut number_string: String = String::new();
    
    calibration_string
        .chars()
        .for_each(|character| {
            let digit = character.to_string().parse::<u32>();
            if digit.is_ok() {
                digits.push(digit.unwrap())
            }

            number_string += &character.to_string();

            let mut found_number = None;
            numbers
                .keys()
                .filter(|&key| number_string.contains(key))
                .for_each(|&number| {
                    digits.push(*numbers.get(number).unwrap());
                    found_number = Some(number);
                });
            
            if found_number.is_some() {
                number_string = number_string.replace(&remove_last_char(found_number.unwrap().to_string()), "");
            }
        });
    
    digits[0] * 10 + digits[digits.len() - 1]
}

pub fn calculate_multiple_calibrations(calibrations: String) -> u32 {

    let mut result: u32 = 0;

    calibrations.split("\n").for_each(|row| {
        result += calculate_calibration(row.to_string());
    });

    result
}
