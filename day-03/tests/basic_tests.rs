use day_03::calculate_schematic;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(calculate_schematic("467..114..
                                        ...*......
                                        ..35..633.
                                        ......#...
                                        617*......
                                        .....+.58.
                                        ..592.....
                                        ......755.
                                        ...$.*....
                                        .664.598..".to_string()), 4361);
    }
}