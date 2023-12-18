use day_05::get_lowest_location;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic_test() {
        assert_eq!(get_lowest_location("seeds: 79 14 55 13\r
\r
seed-to-soil map:\r
50 98 2\r
52 50 48\r
\r
soil-to-fertilizer map:\r
0 15 37\r
37 52 2\r
39 0 15\r
\r
fertilizer-to-water map:\r
49 53 8\r
0 11 42\r
42 0 7\r
57 7 4\r
\r
water-to-light map:\r
88 18 7\r
18 25 70\r
\r
light-to-temperature map:\r
45 77 23\r
81 45 19\r
68 64 13\r
\r
temperature-to-humidity map:\r
0 69 1\r
1 0 69\r
\r
humidity-to-location map:\r
60 56 37\r
56 93 4".to_string()), 35);
    }

    #[test]
    fn the_definitive_1() {
        assert_eq!(get_lowest_location(std::fs::read_to_string("seeds.txt").expect("The file again")), 174137457);
    }
}