use regex::Regex;
use utils::{Solution, run_solution};

// Thanks Claude for this one...
// I thought about the logic behind it, with the matrix operations and asked Claude for implementation.
//
// What I looked:
// - https://github.com/pmneila/Lights-Out
// - https://www.cs.umd.edu/~gasarch/TOPICS/factoring/fastgauss.pdf
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
    buttons: Vec<Vec<bool>>,
}

impl Machine {
    fn parse(line: &str) -> Self {
        let target_re = Regex::new(r"\[([.#]+)\]").unwrap();
        let button_re = Regex::new(r"\(([0-9,]+)\)").unwrap();

        let target: Vec<bool> = target_re
            .captures(line)
            .map(|cap| cap[1].chars().map(|c| c == '#').collect())
            .unwrap_or_default();

        let buttons: Vec<Vec<bool>> = button_re
            .captures_iter(line)
            .map(|cap| {
                let indices: Vec<usize> =
                    cap[1].split(',').filter_map(|s| s.parse().ok()).collect();

                let mut button = vec![false; target.len()];
                for &idx in &indices {
                    if idx < button.len() {
                        button[idx] = true;
                    }
                }
                button
            })
            .collect();

        Self { target, buttons }
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
                matrix[light_idx][button_idx] = button[light_idx];
            }
            matrix[light_idx][n_buttons] = target_val;
        }

        gaussian_elimination_gf2_min_weight(&mut matrix)
    }
}

struct Day10;

impl Solution for Day10 {
    type Input = Vec<Machine>;
    type Output = usize;

    fn parse_input(&self, input: &str) -> Self::Input {
        input
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(Machine::parse)
            .collect()
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        data.iter().map(Machine::min_button_presses).sum()
    }

    fn part2(&self, _data: &Self::Input) -> Self::Output {
        0
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
        assert_eq!(machine.buttons[0], vec![false, false, false, true]);
        assert_eq!(machine.buttons[1], vec![false, true, false, true]);
        assert_eq!(machine.buttons[5], vec![true, true, false, false]);
    }

    #[test]
    fn test_individual_machines() {
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

        assert_eq!(part2, 0);
    }
}
