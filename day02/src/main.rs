use rayon::iter::{IntoParallelIterator, ParallelIterator};
use std::collections::BTreeSet;
use utils::{Solution, run_solution};

struct Day02;

impl Day02 {
    /// Helper function to check if a number falls within any range and add it to the sum.
    /// Returns true if the number was added (i.e., it was in a range and not seen before).
    fn try_add_invalid_num(
        invalid_num: i64,
        data: &[(i64, i64)],
        seen: &mut BTreeSet<i64>,
        sum: &mut i64,
    ) -> bool {
        // Skip if already seen to avoid double-counting.
        if seen.contains(&invalid_num) {
            return false;
        }

        // Check if the number falls within any of the given ranges.
        for &(start, end) in data {
            if invalid_num >= start && invalid_num <= end {
                *sum += invalid_num;
                seen.insert(invalid_num);
                return true;
            }
        }
        false
    }
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
        if data.is_empty() {
            return 0;
        }

        let max_val = data.iter().map(|r| r.1).max().unwrap();

        // Collect all valid numbers from all iterations
        let mut all_valid_nums = Vec::new();

        for d in 1..=9 {
            let power_of_10_d = 10_i64.pow(d);
            let multiplier = power_of_10_d + 1;
            let lower_k = 10_i64.pow(d - 1);

            // Early termination: smallest possible number already exceeds max_val.
            if lower_k > max_val / multiplier {
                break;
            }

            let upper_k = power_of_10_d;

            // Parallelize the inner loop using rayon
            let valid_nums: Vec<i64> = (lower_k..upper_k)
                .into_par_iter()
                .filter_map(|k| {
                    let invalid_num = k * multiplier;

                    if invalid_num > max_val {
                        return None;
                    }

                    // Check if the number falls within any of the given ranges
                    for &(start, end) in data {
                        if invalid_num >= start && invalid_num <= end {
                            return Some(invalid_num);
                        }
                    }
                    None
                })
                .collect();

            all_valid_nums.extend(valid_nums);
        }

        // Remove duplicates using BTreeSet and sum
        all_valid_nums
            .into_iter()
            .collect::<BTreeSet<_>>()
            .into_iter()
            .sum()
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        if data.is_empty() {
            return 0;
        }

        let max_val = data.iter().map(|r| r.1).max().unwrap();
        let mut sum = 0;
        let mut seen = BTreeSet::new();

        'd_loop: for d in 1..=18 {
            let lower_k = if d == 1 { 1 } else { 10_i64.pow(d - 1) };
            let upper_k = 10_i64.pow(d);

            'k_loop: for k in lower_k..upper_k {
                if k == 0 {
                    continue;
                }
                let k_str = k.to_string();

                for r in 2.. {
                    let total_len = k_str.len() * r;
                    if total_len > 19 {
                        break;
                    }

                    let s = k_str.repeat(r);
                    let invalid_num: i64 = match s.parse() {
                        Ok(n) => n,
                        Err(_) => break,
                    };

                    if invalid_num > max_val {
                        if k == lower_k && r == 2 {
                            break 'd_loop;
                        }

                        if r == 2 {
                            break 'k_loop;
                        }

                        break;
                    }

                    Self::try_add_invalid_num(invalid_num, data, &mut seen, &mut sum);
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
        // These tests are for the old implementation, but we can verify the new one with some manual checks.
        // The logic is now inside part1, not in a dedicated function.
        let day = Day02;
        assert_eq!(day.part1(&day.parse_input("11-11")), 11);
        assert_eq!(day.part1(&day.parse_input("6464-6464")), 6464);
        assert_eq!(day.part1(&day.parse_input("123123-123123")), 123_123);
        assert_eq!(day.part1(&day.parse_input("12-12")), 0);
        assert_eq!(day.part1(&day.parse_input("101-101")), 0);
    }

    #[test]
    fn test_is_invalid_id_part2() {
        // These tests are for the old implementation.
        // We can verify the new logic by testing part2 with specific inputs.
        let day = Day02;
        assert_eq!(day.part2(&day.parse_input("11-11")), 11);
        assert_eq!(day.part2(&day.parse_input("111-111")), 111);
        assert_eq!(day.part2(&day.parse_input("999-999")), 999);
        assert_eq!(day.part2(&day.parse_input("1010-1010")), 1010);
        assert_eq!(day.part2(&day.parse_input("12341234-12341234")), 12_341_234);
        assert_eq!(
            day.part2(&day.parse_input("123123123-123123123")),
            123_123_123
        );
        assert_eq!(day.part2(&day.parse_input("12-12")), 0);
    }

    #[test]
    fn test_part2() {
        let day = Day02;
        let parsed_input = day.parse_input(INPUT);

        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 4_174_379_265);
    }
}
