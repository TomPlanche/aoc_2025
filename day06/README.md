# Day 06: Trash Compactor

## Problem Description

After jumping into a garbage chute, you find yourself in a garbage smasher. While waiting for the cephalopods to open the door, you help the youngest cephalopod with her math homework.

The math worksheet consists of problems arranged horizontally, where each problem's numbers are arranged vertically. Problems are separated by full columns of spaces.

Example:
```
123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +
```

This worksheet contains four problems:
- 123 * 45 * 6 = 33210
- 328 + 64 + 98 = 490
- 51 * 387 * 215 = 4243455
- 64 + 23 + 314 = 401

Grand total: 33210 + 490 + 4243455 + 401 = 4277556

The goal is to solve all problems on the worksheet and find the grand total by adding together all individual problem answers.

## Solution Approach

### Part 1

1. Identify separator columns (columns that are all spaces across all rows)
2. Use separators to split the input into distinct problem columns
3. For each problem column:
   - Extract all numbers from rows 1 through n-1
   - Extract the operation (+, *) from the last row
4. For each problem:
   - Apply the operation sequentially to all numbers
   - Example: 123 * 45 * 6 = (123 * 45) * 6 = 5535 * 6 = 33210
5. Sum all problem results to get the grand total

The implementation uses `i64` to handle the large intermediate values that can occur during multiplication.

### Part 2

[Awaiting Part 2 instructions after completing Part 1]

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Notes

The main parsing challenge was correctly identifying problem boundaries by detecting separator columns, then extracting numbers and operations from the vertical arrangement while handling variable spacing and alignment.
