use day_01::calculate_multiple_calibrations;

fn main() {
    let  input = std::fs::read_to_string("calibration_file.txt").expect("Forgot the file...");

    println!("{}", calculate_multiple_calibrations(input));
}