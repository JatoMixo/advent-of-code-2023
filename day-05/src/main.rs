use day_05::get_lowest_location_ranges;

fn main() {
    let input = std::fs::read_to_string("seeds.txt").expect("Forgot the file...");

    println!("{}", get_lowest_location_ranges(input));
}