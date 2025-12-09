# Day 09: Movie Theater

## Problem Description

The North Pole base movie theater has a tile floor with red tiles at specific locations. The Elves want to find the largest rectangle that uses red tiles for two of its opposite corners.

Given a list of coordinates for red tiles, find the largest rectangular area possible by choosing any two red tiles as opposite corners.

## Solution Approach

### Part 1

The solution uses a brute-force approach to check all possible pairs of red tiles:

1. Parse the input to extract all red tile coordinates as `Point<i64>`
2. Iterate through all unique pairs of tiles
3. For each pair, calculate the rectangle area:
   - Width = `|x2 - x1| + 1` (inclusive of both corners)
   - Height = `|y2 - y1| + 1` (inclusive of both corners)
   - Area = width × height
4. Track and return the maximum area found

The key insight is that the area calculation must be inclusive of both corner tiles, hence the `+ 1` in both dimensions.

Time Complexity: O(n²) where n is the number of red tiles

### Part 2

[To be implemented after Part 1 submission]

## Running

```bash
cargo run                    # Normal output
cargo run -- --timing        # Both parts with timing
cargo run -- --part1         # Part 1 only with timing
cargo run -- --part2         # Part 2 only with timing
```

## Testing

```bash
cargo test
```

## Notes

The solution leverages the `Point<i64>` type from the utils crate for clean coordinate handling. The brute-force approach is straightforward and sufficient for the problem size.
