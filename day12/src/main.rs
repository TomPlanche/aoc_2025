use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::HashSet;
use utils::{Solution, run_solution};

/// `(row, column)`
/// `isize` allows negative coordinates during transformations
type Point = (isize, isize);

/// Represents a present shape as a collection of cells.
///
/// Shapes are stored as a list of relative positions from an origin point.
/// The shape can be rotated and flipped to generate all possible orientations.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Shape {
    /// The cells that make up this shape, represented as (row, col) offsets
    cells: Vec<Point>,
}

impl Shape {
    /// Parses a shape from visual representation lines.
    ///
    /// # Arguments
    /// * `lines` - Array of strings where '#' represents a filled cell
    ///
    /// # Example
    /// ```text
    /// ###
    /// #..
    /// ```
    /// This creates a shape with cells at positions (0,0), (0,1), (0,2), (1,0)
    fn from_lines(lines: &[&str]) -> Self {
        let mut cells = Vec::new();

        for (row, line) in lines.iter().enumerate() {
            for (col, ch) in line.chars().enumerate() {
                if ch == '#'
                    && let (Ok(r), Ok(c)) = (isize::try_from(row), isize::try_from(col))
                {
                    cells.push((r, c));
                }
            }
        }

        Self { cells }
    }

    /// Normalizes the shape by moving it so the top-left cell is at (0, 0).
    ///
    /// This ensures shapes can be compared regardless of their absolute position.
    fn normalize(&self) -> Self {
        if self.cells.is_empty() {
            return self.clone();
        }

        let min_r = self.cells.iter().map(|&(r, _)| r).min().unwrap();
        let min_c = self.cells.iter().map(|&(_, c)| c).min().unwrap();

        let cells = self
            .cells
            .iter()
            .map(|&(r, c)| (r - min_r, c - min_c))
            .collect();

        Self { cells }
    }

    /// Rotates the shape 90 degrees clockwise.
    ///
    /// Uses the rotation matrix: (r, c) -> (c, -r)
    fn rotate_90(&self) -> Self {
        let cells = self.cells.iter().map(|&(r, c)| (c, -r)).collect();

        Self { cells }.normalize()
    }

    /// Flips the shape horizontally (mirrors across vertical axis).
    ///
    /// Transformation: (r, c) -> (r, -c)
    fn flip_horizontal(&self) -> Self {
        let cells = self.cells.iter().map(|&(r, c)| (r, -c)).collect();

        Self { cells }.normalize()
    }

    /// Generates all unique transformations of this shape.
    ///
    /// Returns up to 8 unique orientations by rotating and flipping:
    /// - 4 rotations (0°, 90°, 180°, 270°)
    /// - Flipped horizontally, then 4 more rotations
    ///
    /// Uses a `HashSet` to automatically deduplicate symmetric shapes.
    fn all_transformations(&self) -> Vec<Self> {
        let mut transformations = HashSet::new();
        let mut current = self.normalize();

        // Generate flipped and non-flipped versions
        for _ in 0..2 {
            // For each flip state, generate 4 rotations
            for _ in 0..4 {
                transformations.insert(current.clone());
                current = current.rotate_90();
            }
            current = current.flip_horizontal();
        }

        transformations.into_iter().collect()
    }

    /// Returns the number of cells in this shape.
    fn area(&self) -> usize {
        self.cells.len()
    }

    /// Checks if this shape can be placed at the given position without overlapping.
    ///
    /// # Arguments
    /// * `grid` - The current grid state (true = occupied, false = empty)
    /// * `row` - The row position for the shape's origin
    /// * `col` - The column position for the shape's origin
    ///
    /// # Returns
    /// `true` if all cells of the shape would be within bounds and unoccupied
    fn fits_at(&self, grid: &[Vec<bool>], row: usize, col: usize) -> bool {
        let height = grid.len();
        let width = grid[0].len();

        // Convert to isize for arithmetic with potentially negative offsets
        let Ok(row_i) = isize::try_from(row) else {
            return false;
        };
        let Ok(col_i) = isize::try_from(col) else {
            return false;
        };

        for &(dr, dc) in &self.cells {
            let r = row_i + dr;
            let c = col_i + dc;

            // Check if out of bounds (negative coordinates)
            if r < 0 || c < 0 {
                return false;
            }

            // Convert back to usize for array indexing
            let (Ok(r), Ok(c)) = (usize::try_from(r), usize::try_from(c)) else {
                return false;
            };

            // Check if out of bounds or already occupied
            if r >= height || c >= width || grid[r][c] {
                return false;
            }
        }
        true
    }

    /// Places this shape on the grid at the given position.
    ///
    /// # Safety
    /// This assumes `fits_at` has been called and returned true.
    fn place_at(&self, grid: &mut [Vec<bool>], row: usize, col: usize) {
        for &(dr, dc) in &self.cells {
            if let (Ok(row_i), Ok(col_i)) = (isize::try_from(row), isize::try_from(col)) {
                let r = usize::try_from(row_i + dr).unwrap();
                let c = usize::try_from(col_i + dc).unwrap();

                grid[r][c] = true;
            }
        }
    }

    /// Removes this shape from the grid (used for backtracking).
    ///
    /// # Safety
    /// This assumes the shape was previously placed at this position.
    fn remove_from(&self, grid: &mut [Vec<bool>], row: usize, col: usize) {
        for &(dr, dc) in &self.cells {
            if let (Ok(row_i), Ok(col_i)) = (isize::try_from(row), isize::try_from(col)) {
                let r = usize::try_from(row_i + dr).unwrap();
                let c = usize::try_from(col_i + dc).unwrap();

                grid[r][c] = false;
            }
        }
    }
}

/// Represents a region under a Christmas tree where presents need to fit.
#[derive(Debug)]
struct Region {
    width: usize,
    height: usize,
    /// Number of each shape type needed (indexed by shape ID)
    present_counts: Vec<usize>,
}

#[derive(Debug)]
struct Input {
    shapes: Vec<Shape>,
    regions: Vec<Region>,
}

struct Day12;

impl Solution for Day12 {
    type Input = Input;
    type Output = usize;

    fn parse_input(&self, input: &str) -> Self::Input {
        let lines: Vec<&str> = input.lines().collect();
        let mut shapes = Vec::new();
        let mut regions = Vec::new();

        let mut i = 0;
        while i < lines.len() {
            let line = lines[i].trim();

            if line.is_empty() {
                i += 1;
                continue;
            }

            // Parse shape definitions (lines like "0:" followed by visual representation)
            if line.ends_with(':') {
                i += 1;

                let mut shape_lines = Vec::new();

                // Collect all non-empty lines until next section
                while i < lines.len() && !lines[i].trim().is_empty() && !lines[i].contains(':') {
                    shape_lines.push(lines[i]);
                    i += 1;
                }

                let shape = Shape::from_lines(&shape_lines);

                shapes.push(shape);
            }
            // Parse region requirements (lines like "4x4: 0 0 0 0 2 0")
            else if line.contains('x') && line.contains(':') {
                let parts: Vec<&str> = line.split(':').collect();
                let dims: Vec<&str> = parts[0].split('x').collect();
                let width = dims[0].trim().parse().unwrap();
                let height = dims[1].trim().parse().unwrap();

                // Parse the count of each shape type needed
                let present_counts: Vec<usize> = parts[1]
                    .split_whitespace()
                    .filter_map(|s| s.parse().ok())
                    .collect();

                regions.push(Region {
                    width,
                    height,
                    present_counts,
                });

                i += 1;
            } else {
                i += 1;
            }
        }

        Input { shapes, regions }
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        data.regions
            .par_iter()
            .filter(|region| can_fit_all_presents(region, &data.shapes))
            .count()
    }

    fn part2(&self, _data: &Self::Input) -> Self::Output {
        0
    }
}

/// Determines if all required presents can fit in the given region.
///
/// Uses a backtracking algorithm to try all possible placements.
/// First performs a quick area check - if the total area of presents
/// exceeds the region area, it's impossible to fit them.
///
/// # Arguments
/// * `region` - The region specification with dimensions and required presents
/// * `shapes` - All available shape definitions
///
/// # Returns
/// `true` if a valid arrangement exists, `false` otherwise
fn can_fit_all_presents(region: &Region, shapes: &[Shape]) -> bool {
    let mut presents = Vec::new();
    let mut total_area = 0;

    // Build list of all presents that need to be placed
    for (shape_idx, &count) in region.present_counts.iter().enumerate() {
        for _ in 0..count {
            presents.push(shape_idx);
            total_area += shapes[shape_idx].area();
        }
    }

    // Quick check: if total area exceeds region, it's impossible
    let region_area = region.width * region.height;
    if total_area > region_area {
        return false;
    }

    // Initialize empty grid
    let mut grid = vec![vec![false; region.width]; region.height];

    // Pre-compute all transformations for each shape (optimization)
    let transformations: Vec<Vec<Shape>> = shapes.iter().map(Shape::all_transformations).collect();

    // Try to place all presents using backtracking
    backtrack(&mut grid, &presents, &transformations, 0)
}

/// Recursively attempts to place presents using backtracking.
///
/// This is a classic constraint satisfaction problem solver:
/// - Try placing each present in all possible positions and orientations
/// - If a placement works, recurse to place the next present
/// - If no valid placement exists, backtrack (undo and try next option)
///
/// # Arguments
/// * `grid` - Current state of the placement grid
/// * `presents` - List of shape indices to place
/// * `transformations` - Pre-computed transformations for each shape
/// * `present_idx` - Index of the current present being placed
///
/// # Returns
/// `true` if all remaining presents can be successfully placed
fn backtrack(
    grid: &mut [Vec<bool>],
    presents: &[usize],
    transformations: &[Vec<Shape>],
    present_idx: usize,
) -> bool {
    // Base case: all presents have been placed successfully
    if present_idx == presents.len() {
        return true;
    }

    let shape_idx = presents[present_idx];
    let height = grid.len();
    let width = grid[0].len();

    // Try all transformations of this shape
    for transformation in &transformations[shape_idx] {
        // Try all possible positions
        for row in 0..height {
            for col in 0..width {
                // Check if this placement is valid
                if transformation.fits_at(grid, row, col) {
                    // Place the shape
                    transformation.place_at(grid, row, col);

                    // Recurse to place remaining presents
                    if backtrack(grid, presents, transformations, present_idx + 1) {
                        return true;
                    }

                    // Backtrack: this path didn't work, remove the shape
                    transformation.remove_from(grid, row, col);
                }
            }
        }
    }

    false
}

fn main() {
    run_solution!(Day12);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "0:
###
##.
##.

1:
###
##.
.##

2:
.##
###
##.

3:
##.
###
##.

4:
###
#..
###

5:
###
.#.
###

4x4: 0 0 0 0 2 0
12x5: 1 0 1 0 2 2
12x5: 1 0 1 0 3 2";

        let day = Day12;
        let parsed_input = day.parse_input(input);

        let part1 = day.part1(&parsed_input);

        assert_eq!(part1, 2);
    }

    #[test]
    fn test_part2() {
        let input = "";

        let day = Day12;
        let parsed_input = day.parse_input(input);

        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 0);
    }
}
