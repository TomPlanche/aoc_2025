/**
 * For the first time of this AOC,I resorted to Claude based on my linear calculous algorithms.
 *
 * I used Claude to implement the `search_free_variables`, `solve_integer_linear_min` and `gaussian_elimination_gf2_min_weight` functions
 * from articles I found and my initial approach to the problem.
 */
use regex::Regex;
use utils::{Solution, run_solution};

// Implementation by Claude Code based on my logic for exhaustive search over free variables.
// This helper function recursively searches all combinations of free variable values
// to find the minimum cost solution for the integer linear programming problem.
#[allow(clippy::too_many_arguments)]
fn search_free_variables(
    free_vars: &[usize],
    pivot_col: &[Option<usize>],
    matrix: &[Vec<i64>],
    n_vars: usize,
    current_idx: usize,
    solution: &mut Vec<i64>,
    min_cost: &mut i64,
    max_val: i64,
) {
    if current_idx == free_vars.len() {
        for (row, &col_opt) in pivot_col.iter().enumerate() {
            if let Some(col) = col_opt {
                let pivot_val = matrix[row][col];
                if pivot_val == 0 {
                    continue;
                }

                let mut val = matrix[row][n_vars];
                for (var_idx, &var_val) in solution.iter().enumerate() {
                    if var_idx != col {
                        val -= matrix[row][var_idx] * var_val;
                    }
                }

                if val % pivot_val != 0 {
                    return;
                }
                let result = val / pivot_val;
                if result < 0 {
                    return;
                }
                solution[col] = result;
            }
        }

        if solution.iter().all(|&x| x >= 0) {
            let cost: i64 = solution.iter().sum();
            *min_cost = (*min_cost).min(cost);
        }
        return;
    }

    let free_var = free_vars[current_idx];
    for val in 0..=max_val {
        if *min_cost < i64::MAX && val > *min_cost {
            break;
        }

        solution[free_var] = val;

        search_free_variables(
            free_vars,
            pivot_col,
            matrix,
            n_vars,
            current_idx + 1,
            solution,
            min_cost,
            max_val,
        );
    }
}

// Implementation by Claude Code based on my logic and understanding of the problem.
// I designed the approach using Gaussian elimination over integers with bounded search
// for minimum L1-norm solutions (Integer Linear Programming), then Claude implemented it in Rust.
//
// Ressources:
// - https://see.stanford.edu/materials/lsoeldsee263/08-min-norm.pdf
fn solve_integer_linear_min(matrix: &mut [Vec<i64>]) -> i64 {
    let rows = matrix.len();
    if rows == 0 {
        return 0;
    }

    let cols = matrix[0].len();
    if cols <= 1 {
        return 0;
    }

    let n_vars = cols - 1;
    let mut pivot_col = vec![None; rows];
    let mut current_row = 0;

    for col in 0..n_vars {
        let pivot = (current_row..rows)
            .filter(|&r| matrix[r][col] != 0)
            .min_by_key(|&r| matrix[r][col].abs());

        if let Some(pivot_row) = pivot {
            if pivot_row != current_row {
                matrix.swap(pivot_row, current_row);
            }

            for row in 0..rows {
                if row != current_row && matrix[row][col] != 0 {
                    let factor = matrix[row][col];
                    let pivot_val = matrix[current_row][col];
                    let pivot_row_copy = matrix[current_row].clone();

                    for (c, &pivot_c) in pivot_row_copy.iter().enumerate() {
                        matrix[row][c] = matrix[row][c] * pivot_val - factor * pivot_c;
                    }
                }
            }

            pivot_col[current_row] = Some(col);
            current_row += 1;

            if current_row >= rows {
                break;
            }
        }
    }

    if matrix.iter().skip(current_row).any(|row| row[n_vars] != 0) {
        return 0;
    }

    let mut is_pivot = vec![false; n_vars];
    for &col_opt in &pivot_col {
        if let Some(col) = col_opt {
            is_pivot[col] = true;
        }
    }

    let free_vars: Vec<usize> = (0..n_vars).filter(|&i| !is_pivot[i]).collect();

    if free_vars.is_empty() {
        let mut solution = vec![0i64; n_vars];

        for (row, &col_opt) in pivot_col.iter().enumerate() {
            if let Some(col) = col_opt {
                let pivot_val = matrix[row][col];
                if pivot_val != 0 && matrix[row][n_vars] % pivot_val == 0 {
                    solution[col] = matrix[row][n_vars] / pivot_val;
                }
            }
        }

        return solution.iter().sum();
    }

    let mut max_target = matrix
        .iter()
        .map(|row| row[n_vars])
        .max()
        .unwrap_or(0)
        .abs();
    max_target = max_target.max(100);

    let mut min_cost = i64::MAX;
    let mut solution = vec![0i64; n_vars];

    search_free_variables(
        &free_vars,
        &pivot_col,
        matrix,
        n_vars,
        0,
        &mut solution,
        &mut min_cost,
        max_target,
    );

    if min_cost == i64::MAX { 0 } else { min_cost }
}

// Implementation by Claude Code based on my logic and understanding of the problem.
// I designed the approach using Gaussian elimination over GF(2) with exhaustive search
// for minimum weight solutions, then Claude implemented it in Rust.
//
// Resources consulted:
// - https://github.com/pmneila/Lights-Out (Lights Out puzzle solver)
// - https://www.cs.umd.edu/~gasarch/TOPICS/factoring/fastgauss.pdf (Fast Gaussian elimination)
fn gaussian_elimination_gf2_min_weight(matrix: &mut [Vec<bool>]) -> usize {
    let rows = matrix.len();
    if rows == 0 {
        return 0;
    }

    let cols = matrix[0].len();
    if cols <= 1 {
        return 0;
    }

    let n_vars = cols - 1;
    let mut pivot_col = vec![None; rows];
    let mut current_row = 0;

    for col in 0..n_vars {
        let pivot = (current_row..rows).find(|&r| matrix[r][col]);

        if let Some(pivot_row) = pivot {
            if pivot_row != current_row {
                matrix.swap(pivot_row, current_row);
            }

            for row in 0..rows {
                if row != current_row && matrix[row][col] {
                    let pivot_row_copy = matrix[current_row].clone();
                    for (c, &pivot_val) in pivot_row_copy.iter().enumerate() {
                        matrix[row][c] ^= pivot_val;
                    }
                }
            }

            pivot_col[current_row] = Some(col);
            current_row += 1;

            if current_row >= rows {
                break;
            }
        }
    }

    if matrix.iter().skip(current_row).any(|row| row[n_vars]) {
        return 0;
    }

    let mut is_pivot = vec![false; n_vars];
    for &col_opt in &pivot_col {
        if let Some(col) = col_opt {
            is_pivot[col] = true;
        }
    }

    let free_vars: Vec<usize> = (0..n_vars).filter(|&i| !is_pivot[i]).collect();

    let mut min_weight = usize::MAX;

    for mask in 0..(1 << free_vars.len()) {
        let mut solution = vec![false; n_vars];

        for (i, &free_var) in free_vars.iter().enumerate() {
            solution[free_var] = (mask >> i) & 1 == 1;
        }

        for (row, &col_opt) in pivot_col.iter().enumerate() {
            if let Some(col) = col_opt {
                let mut val = matrix[row][n_vars];
                for (var_idx, &var_val) in solution.iter().enumerate() {
                    if var_idx != col && var_val {
                        val ^= matrix[row][var_idx];
                    }
                }
                solution[col] = val;
            }
        }

        let weight = solution.iter().filter(|&&x| x).count();
        min_weight = min_weight.min(weight);
    }

    min_weight
}

#[derive(Debug, Clone)]
struct Machine {
    target: Vec<bool>,
    buttons: Vec<Vec<usize>>,
    joltage: Vec<i64>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let target_re = Regex::new(r"\[([.#]+)\]").unwrap();
        let button_re = Regex::new(r"\(([0-9,]+)\)").unwrap();
        let joltage_re = Regex::new(r"\{([0-9,]+)\}").unwrap();

        let target: Vec<bool> = target_re
            .captures(line)
            .map(|cap| cap[1].chars().map(|c| c == '#').collect())
            .unwrap_or_default();

        let buttons: Vec<Vec<usize>> = button_re
            .captures_iter(line)
            .map(|cap| cap[1].split(',').filter_map(|s| s.parse().ok()).collect())
            .collect();

        let joltage: Vec<i64> = joltage_re
            .captures(line)
            .map(|cap| cap[1].split(',').filter_map(|s| s.parse().ok()).collect())
            .unwrap_or_default();

        Self {
            target,
            buttons,
            joltage,
        }
    }

    fn min_button_presses(&self) -> usize {
        let n_lights = self.target.len();
        let n_buttons = self.buttons.len();

        if n_lights == 0 || n_buttons == 0 {
            return 0;
        }

        let mut matrix = vec![vec![false; n_buttons + 1]; n_lights];

        for (light_idx, &target_val) in self.target.iter().enumerate() {
            for (button_idx, button) in self.buttons.iter().enumerate() {
                matrix[light_idx][button_idx] = button.contains(&light_idx);
            }
            matrix[light_idx][n_buttons] = target_val;
        }

        gaussian_elimination_gf2_min_weight(&mut matrix)
    }

    fn min_button_presses_joltage(&self) -> i64 {
        let n_counters = self.joltage.len();
        let n_buttons = self.buttons.len();

        if n_counters == 0 || n_buttons == 0 {
            return 0;
        }

        let mut matrix = vec![vec![0i64; n_buttons + 1]; n_counters];

        for (counter_idx, &target_val) in self.joltage.iter().enumerate() {
            for (button_idx, button) in self.buttons.iter().enumerate() {
                matrix[counter_idx][button_idx] = i64::from(button.contains(&counter_idx));
            }
            matrix[counter_idx][n_buttons] = target_val;
        }

        solve_integer_linear_min(&mut matrix)
    }
}

struct Day10;

impl Solution for Day10 {
    type Input = Vec<Machine>;
    type Output = i64;

    fn parse_input(&self, input: &str) -> Self::Input {
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(Machine::parse)
            .collect()
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        data.iter()
            .map(|m| i64::try_from(m.min_button_presses()).unwrap_or(0))
            .sum()
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        data.iter().map(Machine::min_button_presses_joltage).sum()
    }
}

fn main() {
    run_solution!(Day10);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}
[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}
[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}";

    #[test]
    fn test_machine_parsing() {
        let machine = Machine::parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");

        assert_eq!(machine.target, vec![false, true, true, false]);
        assert_eq!(machine.buttons.len(), 6);
        assert_eq!(machine.buttons[0], vec![3]);
        assert_eq!(machine.buttons[1], vec![1, 3]);
        assert_eq!(machine.buttons[5], vec![0, 1]);
        assert_eq!(machine.joltage, vec![3, 5, 4, 7]);
    }

    #[test]
    fn test_individual_machines_part1() {
        let machine1 = Machine::parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(machine1.min_button_presses(), 2);

        let machine2 =
            Machine::parse("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(machine2.min_button_presses(), 3);

        let machine3 =
            Machine::parse("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(machine3.min_button_presses(), 2);
    }

    #[test]
    fn test_individual_machines_part2() {
        let machine1 = Machine::parse("[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {3,5,4,7}");
        assert_eq!(machine1.min_button_presses_joltage(), 10);

        let machine2 =
            Machine::parse("[...#.] (0,2,3,4) (2,3) (0,4) (0,1,2) (1,2,3,4) {7,5,12,7,2}");
        assert_eq!(machine2.min_button_presses_joltage(), 12);

        let machine3 =
            Machine::parse("[.###.#] (0,1,2,3,4) (0,3,4) (0,1,2,4,5) (1,2) {10,11,11,5,10,5}");
        assert_eq!(machine3.min_button_presses_joltage(), 11);
    }

    #[test]
    fn test_part1() {
        let day = Day10;
        let parsed_input = day.parse_input(EXAMPLE);

        let part1 = day.part1(&parsed_input);

        assert_eq!(part1, 7);
    }

    #[test]
    fn test_part2() {
        let day = Day10;
        let parsed_input = day.parse_input(EXAMPLE);

        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 33);
    }
}
