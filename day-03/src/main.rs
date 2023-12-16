use day_03::get_gear_ratio;

fn main() {
    let input = std::fs::read_to_string("engine_schematic.txt").expect("Forgot the file...");

    println!("{}", get_gear_ratio(input));
}
