pub fn calculate_calibration(calibration_string: String) -> u32 {

    let mut digits = Vec::new();
    
    calibration_string
        .chars()
        .filter(|character| character.to_string().parse::<u32>().is_ok())
        .for_each(|character| {
            digits.push(character.to_string().parse::<u32>().unwrap())
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
