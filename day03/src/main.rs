use utils::{Solution, run_solution};

struct Day03;

impl Solution for Day03 {
    type Input = Vec<Vec<u32>>;
    type Output = u64;

    fn parse_input(&self, input: &str) -> Self::Input {
        input
            .lines()
            .map(str::trim)
            .filter(|line| !line.is_empty())
            .map(|line| {
                line.chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect()
            })
            .collect()
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        data.iter()
            .map(|bank| Self::max_k_digits(bank, 2))
            .sum()
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        data.iter()
            .map(|bank| Self::max_k_digits(bank, 12))
            .sum()
    }
}
impl Day03 {
    fn max_k_digits(digits: &[u32], k: usize) -> u64 {
        let n = digits.len();
        let mut result = Vec::new();
        let mut start = 0;

        for remaining in (1..=k).rev() {
            // We need to pick 'remaining' more digits
            // We can look ahead to position: n - remaining
            let window_end = (n - remaining + 1).min(n);

            // Find the maximum digit in the window [start, window_end)
            let window = &digits[start..window_end];
            let max_digit = *window.iter().max().unwrap();

            // Find the first occurrence of the max digit in the window
            let max_idx = window.iter().position(|&d| d == max_digit).unwrap();

            let actual_idx = start + max_idx;
            result.push(digits[actual_idx]);
            start = actual_idx + 1;
        }

        // Convert result digits to a number
        result.iter().fold(0u64, |acc, &d| acc * 10 + u64::from(d))
    }
}

fn main() {
    run_solution!(Day03);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        let day = Day03;
        let parsed_input = day.parse_input(input);

        let part1 = day.part1(&parsed_input);

        assert_eq!(part1, 357);
    }

    #[test]
    fn test_part2() {
        let input = "987654321111111
811111111111119
234234234234278
818181911112111";

        let day = Day03;
        let parsed_input = day.parse_input(input);

        let part2 = day.part2(&parsed_input);

        // 987654321111 + 811111111119 + 434234234278 + 888911112111 = 3121910778619
        assert_eq!(part2, 3_121_910_778_619);
    }

    #[test]
    fn test_max_k_digits() {
        // Test individual examples from part 2
        let digits1: Vec<u32> = "987654321111111"
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        assert_eq!(Day03::max_k_digits(&digits1, 12), 987_654_321_111);

        let digits2: Vec<u32> = "811111111111119"
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        assert_eq!(Day03::max_k_digits(&digits2, 12), 811_111_111_119);

        let digits3: Vec<u32> = "234234234234278"
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        assert_eq!(Day03::max_k_digits(&digits3, 12), 434_234_234_278);

        let digits4: Vec<u32> = "818181911112111"
            .chars()
            .map(|c| c.to_digit(10).unwrap())
            .collect();
        assert_eq!(Day03::max_k_digits(&digits4, 12), 888_911_112_111);
    }
}
