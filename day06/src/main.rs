use utils::{run_solution, Solution};

struct Day06;

#[derive(Debug)]
struct Problem {
    numbers_part1: Vec<i64>,
    numbers_part2: Vec<i64>,
    operation: char,
}

impl Solution for Day06 {
    type Input = Vec<Problem>;
    type Output = i64;

    fn parse_input(&self, input: &str) -> Self::Input {
        let lines: Vec<&str> = input.lines().collect();
        if lines.is_empty() {
            return Vec::new();
        }

        // Find max line length
        let max_len = lines.iter().map(|l| l.len()).max().unwrap_or(0);

        // Identify separator columns (all spaces in all lines)
        let mut separator_cols = Vec::new();
        for col_idx in 0..max_len {
            let mut is_separator = true;
            for line in &lines {
                if let Some(ch) = line.chars().nth(col_idx)
                    && ch != ' '
                {
                    is_separator = false;
                    break;
                }
            }
            if is_separator {
                separator_cols.push(col_idx);
            }
        }

        // Identify problem column ranges
        let mut problem_ranges = Vec::new();
        let mut start = 0;

        for &sep_col in &separator_cols {
            if sep_col > start {
                problem_ranges.push((start, sep_col - 1));
            }
            start = sep_col + 1;
        }

        // Add the last range
        if start < max_len {
            problem_ranges.push((start, max_len - 1));
        }

        // For each problem range, extract numbers and operation
        let mut problems = Vec::new();

        for (col_start, col_end) in problem_ranges {
            let mut numbers_part1 = Vec::new();
            let mut numbers_part2 = Vec::new();
            let mut operation = ' ';

            // Part 1: Extract substring for each line, each line is a number
            for (line_idx, line) in lines.iter().enumerate() {
                let start_idx = col_start.min(line.len());
                let end_idx = (col_end + 1).min(line.len());

                if start_idx >= end_idx {
                    continue;
                }

                let substring = &line[start_idx..end_idx];
                let trimmed = substring.trim();

                if trimmed.is_empty() {
                    continue;
                }

                // Last line contains the operation
                if line_idx == lines.len() - 1 {
                    operation = trimmed.chars().next().unwrap_or(' ');
                } else if let Ok(num) = trimmed.parse::<i64>() {
                    numbers_part1.push(num);
                }
            }

            // Part 2: Each column forms a number by reading bottom-to-top
            // (bottom = least significant digit)
            for col_idx in col_start..=col_end {
                let mut digits = Vec::new();

                // Read from bottom to top (excluding last line which has operator)
                for line_idx in (0..lines.len() - 1).rev() {
                    if let Some(ch) = lines[line_idx].chars().nth(col_idx)
                        && ch.is_ascii_digit()
                    {
                        digits.push(ch);
                    }
                }

                if !digits.is_empty() {
                    // Digits are already in bottom-to-top order (least to most significant)
                    // Reverse to get most-to-least significant for parsing
                    digits.reverse();
                    let num_str: String = digits.iter().collect();
                    if let Ok(num) = num_str.parse::<i64>() {
                        numbers_part2.push(num);
                    }
                }
            }

            if !numbers_part1.is_empty() && (operation == '+' || operation == '*') {
                problems.push(Problem {
                    numbers_part1,
                    numbers_part2,
                    operation,
                });
            }
        }

        problems
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        data.iter()
            .map(|problem| {
                let mut result = problem.numbers_part1[0];
                for &num in &problem.numbers_part1[1..] {
                    match problem.operation {
                        '+' => result += num,
                        '*' => result *= num,
                        _ => {}
                    }
                }
                result
            })
            .sum()
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        data.iter()
            .map(|problem| {
                let mut result = problem.numbers_part2[0];
                for &num in &problem.numbers_part2[1..] {
                    match problem.operation {
                        '+' => result += num,
                        '*' => result *= num,
                        _ => {}
                    }
                }
                result
            })
            .sum()
    }
}

fn main() {
    run_solution!(Day06);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

    #[test]
    fn test_part1() {
        let day = Day06;
        let parsed_input = day.parse_input(TEST_INPUT);

        // Expected: 123*45*6 + 328+64+98 + 51*387*215 + 64+23+314
        // = 33210 + 490 + 4243455 + 401 = 4277556
        let part1 = day.part1(&parsed_input);

        assert_eq!(part1, 4_277_556);
    }

    #[test]
    fn test_part2() {
        let day = Day06;
        let parsed_input = day.parse_input(TEST_INPUT);

        // Cephalopod math: reading columns bottom-to-top
        // Rightmost: 4 + 431 + 623 = 1058
        // Second from right: 175 * 581 * 32 = 3253600
        // Third from right: 8 + 248 + 369 = 625
        // Leftmost: 356 * 24 * 1 = 8544
        // Total: 1058 + 3253600 + 625 + 8544 = 3263827
        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 3_263_827);
    }
}
