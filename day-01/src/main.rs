use day_01::calculate_multiple_calibrations;

fn main() {
    println!("{}", calculate_multiple_calibrations(std::fs::read_to_string("calibration_file.txt").expect("Forgot the file...")));
}