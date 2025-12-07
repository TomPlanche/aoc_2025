use std::collections::{HashMap, HashSet, VecDeque};
use utils::{run_solution, Solution};

struct Day07;

#[derive(Debug)]
struct Manifold {
    grid: Vec<Vec<char>>,
    width: usize,
    height: usize,
    start: (usize, usize),
}

impl Solution for Day07 {
    type Input = Manifold;
    type Output = usize;

    fn parse_input(&self, input: &str) -> Self::Input {
        let grid: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

        let height = grid.len();
        let width = if height > 0 { grid[0].len() } else { 0 };

        // Find starting position 'S'
        let mut start = (0, 0);
        for (row, line) in grid.iter().enumerate() {
            for (col, &ch) in line.iter().enumerate() {
                if ch == 'S' {
                    start = (row, col);
                    break;
                }
            }
        }

        Manifold {
            grid,
            width,
            height,
            start,
        }
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        let mut split_count = 0;
        let mut queue = VecDeque::new();
        let mut seen = HashSet::new();
        let mut splitters_hit = HashSet::new();

        // Start with a beam at S, moving downward
        // Beam format: (row, col) - the current position of the beam
        queue.push_back(data.start);

        while let Some((row, col)) = queue.pop_front() {
            // Skip if we've already processed a beam from this position
            if !seen.insert((row, col)) {
                continue;
            }

            // Move downward from current position
            let mut current_row = row + 1;

            // Continue moving down until we hit a splitter or exit the manifold
            while current_row < data.height {
                let ch = data.grid[current_row][col];

                if ch == '^' {
                    // Hit a splitter - count only if this splitter hasn't been hit before
                    if splitters_hit.insert((current_row, col)) {
                        split_count += 1;
                    }

                    // Add new beams starting from left and right of the splitter
                    if col > 0 {
                        queue.push_back((current_row, col - 1));
                    }
                    if col + 1 < data.width {
                        queue.push_back((current_row, col + 1));
                    }

                    break; // This beam stops here
                }

                current_row += 1;
            }
        }

        split_count
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        // Use memoization to count timelines from each position
        let mut memo: HashMap<(usize, usize), usize> = HashMap::new();
        count_timelines_from(data.start.0, data.start.1, data, &mut memo)
    }
}

fn count_timelines_from(
    row: usize,
    col: usize,
    data: &Manifold,
    memo: &mut HashMap<(usize, usize), usize>,
) -> usize {
    // Check if we've already computed this
    if let Some(&count) = memo.get(&(row, col)) {
        return count;
    }

    // Move downward from current position
    let mut current_row = row + 1;

    // Continue moving down until we hit a splitter or exit the manifold
    while current_row < data.height {
        let ch = data.grid[current_row][col];

        if ch == '^' {
            // Hit a splitter - particle takes BOTH paths
            let mut total = 0;

            // Left path
            if col > 0 {
                total += count_timelines_from(current_row, col - 1, data, memo);
            } else {
                // Can't go left (at edge), this timeline ends
                total += 1;
            }

            // Right path
            if col + 1 < data.width {
                total += count_timelines_from(current_row, col + 1, data, memo);
            } else {
                // Can't go right (at edge), this timeline ends
                total += 1;
            }

            memo.insert((row, col), total);
            return total;
        }

        current_row += 1;
    }

    // Exited the bottom - this is one timeline
    memo.insert((row, col), 1);
    1
}

fn main() {
    run_solution!(Day07);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = ".......S.......
...............
.......^.......
...............
......^.^......
...............
.....^.^.^.....
...............
....^.^...^....
...............
...^.^...^.^...
...............
..^...^.....^..
...............
.^.^.^.^.^...^.
...............";

    #[test]
    fn test_part1() {
        let day = Day07;
        let parsed_input = day.parse_input(TEST_INPUT);

        let part1 = day.part1(&parsed_input);

        assert_eq!(part1, 21);
    }

    #[test]
    fn test_part2() {
        let day = Day07;
        let parsed_input = day.parse_input(TEST_INPUT);

        // With quantum splitting, particle takes both paths at each splitter
        // Total of 40 different timelines
        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 40);
    }
}
