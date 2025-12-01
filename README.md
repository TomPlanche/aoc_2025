# Advent of Code 2025

Solutions for [Advent of Code 2025](https://adventofcode.com/2025) written in Rust.

## Project Structure

This is a Cargo workspace containing:
- `utils/` - Shared utilities for all days (Direction, Point, Solution trait)
- `dayXX/` - Individual day solutions

## Running Solutions

### Run a specific day

```bash
cd day01
cargo run
```

### Run all days

```bash
# From workspace root
for day in day*/; do
    echo "Running $day"
    (cd "$day" && cargo run)
done
```

## Testing

### Test a specific day

```bash
cd day01
cargo test
```

### Test all days

```bash
cargo test --workspace
```

## Utilities

The `utils` crate provides common functionality:

### Solution Trait

Standardized structure for daily solutions:

```rust
use utils::{run_solution, Solution};

struct Day01;

impl Solution for Day01 {
    type Input = Vec<i32>;
    type Output = i32;

    fn parse_input(&self, input: &str) -> Self::Input {
        // Parse logic here
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        // Part 1 solution
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        // Part 2 solution
    }
}

fn main() {
    run_solution!(Day01);
}
```

### Direction Enum

8-directional movement with (x, y) coordinates:

```rust
use utils::Direction;

let pos = (0, 0);
let new_pos = pos + Direction::Right;  // (1, 0)
```

### Point Type

Generic point implementation:

```rust
use utils::Point;

let p1 = Point::new(10, 20);
let p2 = Point::new(15, 25);
let distance = p1.manhattan_distance(&p2);
```

## Adding New Days

This project uses [aoc-cli-v2](https://github.com/TomPlanche/aoc-cli-v2), a custom CLI tool for managing Advent of Code solutions.

### Installation

```bash
cargo install --git https://github.com/TomPlanche/aoc-cli-v2
```

### Usage

```bash
# Add a new day solution template
aoc-cli add <day>

# Download input for a specific day
aoc-cli download <day>

# Submit an answer
aoc-cli submit <day> <part> <answer>
```

The tool automatically:
- Creates the day folder structure
- Sets up the solution template with the Solution trait
- Downloads your personalized input from adventofcode.com
- Submits answers directly from the command line

## Progress

- [x] Day 1
- [ ] Day 2
- [ ] Day 3
- [ ] Day 4
- [ ] Day 5
- [ ] Day 6
- [ ] Day 7
- [ ] Day 8
- [ ] Day 9
- [ ] Day 10
- [ ] Day 11
- [ ] Day 12
- [ ] Day 13
- [ ] Day 14
- [ ] Day 15
- [ ] Day 16
- [ ] Day 17
- [ ] Day 18
- [ ] Day 19
- [ ] Day 20
- [ ] Day 21
- [ ] Day 22
- [ ] Day 23
- [ ] Day 24
- [ ] Day 25

## Notes

[Add any general notes, patterns, or observations across the challenges]
