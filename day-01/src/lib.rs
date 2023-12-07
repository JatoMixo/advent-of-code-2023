use std::collections::HashMap;

fn remove_last_char(string: &String) -> String {
    let mut chars = string.chars();
    chars.next_back();

    chars.as_str().to_string()
}

fn find_number_in_string(number_string: &String, numbers: &HashMap<&str, u32>) -> Option<String> {
    let mut found_number = None;

    numbers
        .keys()
        .filter(|&key| number_string.contains(key))
        .for_each(|&number| {
            found_number = Some(number.to_string());
        });
    
    found_number
}

fn remove_number_from_string(number_string: &String, found_number: &String) -> String {
    number_string.replace(&remove_last_char(&found_number), "")
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
            // In case the number is typed normally
            let digit = character.to_string().parse::<u32>();
            if digit.is_ok() {
                digits.push(digit.unwrap())
            }

            // In case the number is typed with letters
            number_string += &character.to_string();

            let found_number = find_number_in_string(&number_string, &numbers);
            if found_number.is_some() {
                number_string = remove_number_from_string(&number_string, &found_number.clone().unwrap());
                digits.push(*numbers.get(&found_number.unwrap().as_str()).unwrap());
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
