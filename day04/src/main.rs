use utils::{Direction, Solution, run_solution};

struct Day04;

type Grid = Vec<Vec<char>>;

impl Solution for Day04 {
    type Input = Grid;
    type Output = usize;

    fn parse_input(&self, input: &str) -> Self::Input {
        input.lines().map(|line| line.chars().collect()).collect()
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        count_accessible_rolls(data)
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        count_removable_rolls(data)
    }
}

fn count_adjacent_rolls(grid: &Grid, row: usize, col: usize) -> usize {
    let rows = grid.len();
    let cols = grid[0].len();
    let mut count = 0;

    for dir in &Direction::all() {
        let (dx, dy): (isize, isize) = (*dir).into();
        let new_row = row.wrapping_add_signed(dy);
        let new_col = col.wrapping_add_signed(dx);

        if new_row < rows && new_col < cols && grid[new_row][new_col] == '@' {
            count += 1;
        }
    }

    count
}

fn count_accessible_rolls(grid: &Grid) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    let mut accessible_count = 0;

    for row in 0..rows {
        for col in 0..cols {
            if grid[row][col] == '@' && count_adjacent_rolls(grid, row, col) < 4 {
                accessible_count += 1;
            }
        }
    }

    accessible_count
}

fn count_removable_rolls(grid: &Grid) -> usize {
    let rows = grid.len();
    if rows == 0 {
        return 0;
    }
    let cols = grid[0].len();

    // Create a mutable copy of the grid
    let mut working_grid = grid.clone();
    let mut total_removed = 0;

    loop {
        // Find all accessible rolls in the current state
        let mut to_remove = Vec::new();

        for row in 0..rows {
            for col in 0..cols {
                if working_grid[row][col] == '@'
                    && count_adjacent_rolls(&working_grid, row, col) < 4
                {
                    to_remove.push((row, col));
                }
            }
        }

        // If no rolls can be removed, we're done
        if to_remove.is_empty() {
            break;
        }

        // Remove all accessible rolls
        for (row, col) in &to_remove {
            working_grid[*row][*col] = '.';
        }

        total_removed += to_remove.len();
    }

    total_removed
}

fn main() {
    run_solution!(Day04);
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

    #[test]
    fn test_part1() {
        let day = Day04;
        let parsed_input = day.parse_input(TEST_INPUT);

        let part1 = day.part1(&parsed_input);

        assert_eq!(part1, 13);
    }

    #[test]
    fn test_part2() {
        let day = Day04;
        let parsed_input = day.parse_input(TEST_INPUT);

        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 43);
    }
}
