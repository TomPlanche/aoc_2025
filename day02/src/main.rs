use utils::{Solution, run_solution};

struct Day02;

fn is_invalid_id(num: i64) -> bool {
    let s = num.to_string();

    // Must have even length to be split in half
    if !s.len().is_multiple_of(2) {
        return false;
    }

    // Check for leading zero (which would make it invalid as a number representation)
    if s.starts_with('0') {
        return false;
    }

    // Split in half and check if both halves are identical
    let mid = s.len() / 2;
    let first_half = &s[..mid];
    let second_half = &s[mid..];

    first_half == second_half
}

fn is_invalid_id_part2(num: i64) -> bool {
    let s = num.to_string();

    // Can't have leading zeros
    if s.starts_with('0') {
        return false;
    }

    let len = s.len();

    // Try all possible pattern lengths from 1 to len/2
    // A pattern repeated at least twice means the pattern length is at most len/2
    for pattern_len in 1..=len / 2 {
        // Check if length is divisible by pattern length
        if len.is_multiple_of(pattern_len) {
            let pattern = &s[..pattern_len];
            let repetitions = len / pattern_len;

            // Check if repeating this pattern creates the whole string
            let repeated = pattern.repeat(repetitions);
            if repeated == s {
                return true;
            }
        }
    }

    false
}

impl Solution for Day02 {
    type Input = Vec<(i64, i64)>;
    type Output = i64;

    fn parse_input(&self, input: &str) -> Self::Input {
        input
            .trim()
            .split(',')
            .filter_map(|range| {
                let parts: Vec<&str> = range.trim().split('-').collect();
                if parts.len() == 2 {
                    let start = parts[0].parse().ok()?;
                    let end = parts[1].parse().ok()?;
                    Some((start, end))
                } else {
                    None
                }
            })
            .collect()
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        let mut sum = 0;

        for &(start, end) in data {
            for num in start..=end {
                if is_invalid_id(num) {
                    sum += num;
                }
            }
        }

        sum
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        let mut sum = 0;

        for &(start, end) in data {
            for num in start..=end {
                if is_invalid_id_part2(num) {
                    sum += num;
                }
            }
        }

        sum
    }
}

fn main() {
    run_solution!(Day02);
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

    #[test]
    fn test_part1() {
        let day = Day02;
        let parsed_input = day.parse_input(INPUT);

        let part1 = day.part1(&parsed_input);

        assert_eq!(part1, 1_227_775_554);
    }

    #[test]
    fn test_is_invalid_id() {
        assert!(is_invalid_id(11));
        assert!(is_invalid_id(22));
        assert!(is_invalid_id(99));
        assert!(is_invalid_id(1010));
        assert!(is_invalid_id(6464));
        assert!(is_invalid_id(123_123));

        assert!(!is_invalid_id(12));
        assert!(!is_invalid_id(101));
        assert!(!is_invalid_id(100));
    }

    #[test]
    fn test_is_invalid_id_part2() {
        // Examples from part 2
        assert!(is_invalid_id_part2(11)); // "1" x 2
        assert!(is_invalid_id_part2(111)); // "1" x 3
        assert!(is_invalid_id_part2(999)); // "9" x 3
        assert!(is_invalid_id_part2(1010)); // "10" x 2
        assert!(is_invalid_id_part2(12_341_234)); // "1234" x 2
        assert!(is_invalid_id_part2(123_123_123)); // "123" x 3
        assert!(is_invalid_id_part2(1_212_121_212)); // "12" x 5
        assert!(is_invalid_id_part2(1_111_111)); // "1" x 7
        assert!(is_invalid_id_part2(565_656)); // "56" x 3
        assert!(is_invalid_id_part2(824_824_824)); // "824" x 3
        assert!(is_invalid_id_part2(2_121_212_121)); // "21" x 5

        assert!(!is_invalid_id_part2(12));
        assert!(!is_invalid_id_part2(101));
        assert!(!is_invalid_id_part2(100));
        assert!(!is_invalid_id_part2(123));
    }

    #[test]
    fn test_part2() {
        let day = Day02;
        let parsed_input = day.parse_input(INPUT);

        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 4_174_379_265);
    }
}
