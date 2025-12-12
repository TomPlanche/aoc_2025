# Day 12: Christmas Tree Farm

## Problem Description

The task involves fitting presents with specific shapes into regions under Christmas trees. Presents can be rotated and flipped but must align perfectly with a grid. The goal is to determine how many regions can fit all their required presents.

## Solution Approach

### Part 1

The solution uses a backtracking algorithm with the following optimizations:

1. **Area Check**: First checks if the total area of all required shapes exceeds the region area - if so, it's impossible to fit them
2. **Shape Transformations**: Pre-computes all unique rotations and flips of each shape (up to 8 transformations)
3. **Backtracking**: Tries to place presents one by one, backtracking when placement fails
4. **Parallel Processing**: Uses Rayon to check multiple regions in parallel

The key insight is that while the problem appears complex, most regions can be quickly determined to be feasible or not through the area check, and the backtracking only runs for edge cases.

### Part 2

Not yet implemented.

## Running

```bash
cargo run                    # Both parts
cargo run -- --timing        # Both parts with timing
cargo run -- --part1         # Part 1 only with timing
```

## Testing

```bash
cargo test
```

## Notes

The backtracking algorithm explores all possible placements but is optimized enough to handle the input efficiently. The use of parallel processing across regions provides significant speedup for the full dataset.
