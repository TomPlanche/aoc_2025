use utils::{run_solution, Solution};

struct Day03;

impl Solution for Day03 {
    type Input = Vec<String>;
    type Output = u64;

    fn parse_input(&self, input: &str) -> Self::Input {
        input
            .lines()
            .map(|line| line.trim().to_string())
            .filter(|line| !line.is_empty())
            .collect()
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        data.iter()
            .map(|bank| {
                let digits: Vec<u32> = bank
                    .chars()
                    .map(|c| c.to_digit(10).unwrap())
                    .collect();

                let mut max_joltage = 0;

                // Try all pairs (i, j) where i < j
                for i in 0..digits.len() {
                    for j in (i + 1)..digits.len() {
                        let joltage = digits[i] * 10 + digits[j];
                        max_joltage = max_joltage.max(joltage);
                    }
                }

                max_joltage as u64
            })
            .sum()
    }

    fn part2(&self, _data: &Self::Input) -> Self::Output {
        // TODO: Implement part 2
        0
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
        let input = "";

        let day = Day03;
        let parsed_input = day.parse_input(input);

        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 0);
    }
}
