use utils::{run_solution, Solution};

struct Day05;

#[derive(Debug)]
struct Database {
    fresh_ranges: Vec<(u64, u64)>,
    ingredient_ids: Vec<u64>,
}

impl Solution for Day05 {
    type Input = Database;
    type Output = usize;

    fn parse_input(&self, input: &str) -> Self::Input {
        let mut lines = input.lines();
        let mut fresh_ranges = Vec::new();
        let mut ingredient_ids = Vec::new();

        // Parse fresh ranges
        for line in lines.by_ref() {
            if line.trim().is_empty() {
                break;
            }
            let parts: Vec<&str> = line.split('-').collect();
            if parts.len() == 2 {
                let start = parts[0].parse().unwrap();
                let end = parts[1].parse().unwrap();
                fresh_ranges.push((start, end));
            }
        }

        // Parse ingredient IDs
        for line in lines {
            if let Ok(id) = line.trim().parse() {
                ingredient_ids.push(id);
            }
        }

        Database {
            fresh_ranges,
            ingredient_ids,
        }
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        data.ingredient_ids
            .iter()
            .filter(|&&id| is_fresh(id, &data.fresh_ranges))
            .count()
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        count_fresh_ids(&data.fresh_ranges)
    }
}

fn is_fresh(id: u64, ranges: &[(u64, u64)]) -> bool {
    ranges.iter().any(|&(start, end)| id >= start && id <= end)
}

fn count_fresh_ids(ranges: &[(u64, u64)]) -> usize {
    if ranges.is_empty() {
        return 0;
    }

    // Sort ranges by start position
    let mut sorted_ranges = ranges.to_vec();
    sorted_ranges.sort_by_key(|&(start, _)| start);

    // Merge overlapping ranges
    let mut merged = Vec::new();
    let mut current = sorted_ranges[0];

    for &(start, end) in &sorted_ranges[1..] {
        if start <= current.1 + 1 {
            // Ranges overlap or are adjacent, merge them
            current.1 = current.1.max(end);
        } else {
            // No overlap, save current and start new range
            merged.push(current);
            current = (start, end);
        }
    }
    merged.push(current);

    // Count total IDs in all merged ranges
    merged
        .iter()
        .map(|&(start, end)| {
            usize::try_from(end - start + 1).unwrap_or(usize::MAX)
        })
        .sum()
}

fn main() {
    run_solution!(Day05);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

    #[test]
    fn test_part1() {
        let day = Day05;
        let parsed_input = day.parse_input(TEST_INPUT);

        let part1 = day.part1(&parsed_input);

        assert_eq!(part1, 3);
    }

    #[test]
    fn test_part2() {
        let day = Day05;
        let parsed_input = day.parse_input(TEST_INPUT);

        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 14);
    }
}
