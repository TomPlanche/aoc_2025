# Day 05: Cafeteria

## Problem Description

The Elves' new inventory management system needs help identifying which ingredient IDs are fresh versus spoiled.

The database consists of:
1. A list of fresh ingredient ID ranges (inclusive, can overlap)
2. A blank line separator
3. A list of available ingredient IDs to check

An ingredient ID is fresh if it falls within ANY of the fresh ranges.

Example:
```
3-5
10-14
16-20
12-18

1
5
8
11
17
32
```

In this example:
- ID 1: spoiled (no range)
- ID 5: fresh (3-5)
- ID 8: spoiled
- ID 11: fresh (10-14)
- ID 17: fresh (16-20 and 12-18)
- ID 32: spoiled

Result: 3 fresh ingredients

## Solution Approach

### Part 1

1. Parse the input into two sections:
   - Fresh ranges as tuples of (start, end)
   - Ingredient IDs to check
2. For each ingredient ID, check if it falls within any range
3. Count how many IDs are fresh

The implementation uses `u64` to handle very large ingredient IDs (hundreds of trillions).

Answer: 638

### Part 2

To be implemented.

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Notes

The main challenge was handling the large range of ingredient IDs in the actual input, requiring `u64` instead of smaller integer types.
