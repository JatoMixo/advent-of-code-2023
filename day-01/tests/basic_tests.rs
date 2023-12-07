use day_01::{calculate_calibration, calculate_multiple_calibrations};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_calibration() {
        assert_eq!(calculate_calibration("1abc2".to_string()), 12);
        assert_eq!(calculate_calibration("pqr3stu8vwx".to_string()), 38);
        assert_eq!(calculate_calibration("a1b2c3d4e5f".to_string()), 15);
        assert_eq!(calculate_calibration("treb7uchet".to_string()), 77);
    }

    #[test]
    fn test_multiple_calibrations() {
        assert_eq!(calculate_multiple_calibrations(String::from("1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet")), 142);
    }

    #[test]
    fn test_with_letters() {
        assert_eq!(calculate_multiple_calibrations(String::from("two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen")), 281);
    }
}