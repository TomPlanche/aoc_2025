# Day 03

## Problem Description

--- Day 3: Lobby ---

You descend a short staircase, enter the surprisingly vast lobby, and are quickly cleared by the security checkpoint. When you get to the main elevators, however, you discover that each one has a red light above it: they're all offline.

"Sorry about that," an Elf apologizes as she tinkers with a nearby control panel. "Some kind of electrical surge seems to have fried them. I'll try to get them online soon."

You explain your need to get further underground. "Well, you could at least take the escalator down to the printing department, not that you'd get much further than that without the elevators working. That is, you could if the escalator weren't also offline."

"But, don't worry! It's not fried; it just needs power. Maybe you can get it running while I keep working on the elevators."

There are batteries nearby that can supply emergency power to the escalator for just such an occasion. The batteries are each labeled with their joltage rating, a value from 1 to 9. You make a note of their joltage ratings (your puzzle input). For example:

```
987654321111111
811111111111119
234234234234278
818181911112111
```

The batteries are arranged into banks; each line of digits in your input corresponds to a single bank of batteries. Within each bank, you need to turn on exactly two batteries; the joltage that the bank produces is equal to the number formed by the digits on the batteries you've turned on. For example, if you have a bank like 12345 and you turn on batteries 2 and 4, the bank would produce 24 jolts. (You cannot rearrange batteries.)

You'll need to find the largest possible joltage each bank can produce. In the above example:

- In 987654321111111, you can make the largest joltage possible, 98, by turning on the first two batteries.
- In 811111111111119, you can make the largest joltage possible by turning on the batteries labeled 8 and 9, producing 89 jolts.
- In 234234234234278, you can make 78 by turning on the last two batteries (marked 7 and 8).
- In 818181911112111, the largest joltage you can produce is 92.

The total output joltage is the sum of the maximum joltage from each bank, so in this example, the total output joltage is 98 + 89 + 78 + 92 = 357.

## Solution Approach

### Part 1: Maximum Two-Battery Joltage

For each battery bank (line of digits), we need to find the maximum two-digit number we can form by selecting exactly two batteries in order.

#### Algorithm

For each bank:
1. Iterate through all pairs of positions (i, j) where i < j
2. Calculate the joltage as: digit[i] * 10 + digit[j]
3. Keep track of the maximum joltage for that bank
4. Sum all maximum joltages from all banks

The key insight is that we want to maximize the two-digit number, so we prioritize finding the largest first digit, then the largest second digit that comes after it.

#### Optimization

Instead of checking all pairs, we can optimize:
- For each position i, find the maximum digit at any position j > i
- Calculate joltage as digit[i] * 10 + max_digit_after_i
- Track the maximum joltage

### Part 2: Maximum Twelve-Battery Joltage

Now we need to select exactly 12 batteries from each bank to form the largest 12-digit number while maintaining their order.

#### Algorithm: Greedy Selection

This is a classic greedy algorithm problem. The key insight is that to maximize the resulting number, we want to pick the largest possible digit at each position from left to right, while ensuring we have enough remaining digits to complete our selection.

##### Implementation

For each position we need to fill (from left to right):
1. Calculate the window of possible starting positions
2. The window size ensures we can still pick the remaining digits after this choice
3. Find the maximum digit in this window
4. Pick the first occurrence of this maximum digit
5. Continue from the position after the picked digit

##### Mathematical Formula

If we need to pick k digits from n digits:
- At each step, when we've picked i digits (need k - i more), starting from position start
- We can look in the window [start, n - (k - i) + 1)
- This ensures we have at least (k - i) digits remaining after our choice

##### Example Walkthrough

For `818181911112111` (15 digits), picking 12:
1. Window [0,4): pick 8 at position 0
2. Window [1,5): pick 8 at position 2
3. Window [3,6): pick 8 at position 4
4. Window [5,7): pick 9 at position 6
5. Take remaining 8 digits: 1,1,1,1,2,1,1,1
6. Result: 888911112111

##### Time Complexity

O(n * k) where n is the length of each battery bank and k is the number of batteries to select (12 in this case).

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Notes

### Key Insights

1. Part 1 is a straightforward brute-force approach - try all pairs and find the maximum
2. Part 2 requires a greedy algorithm that's commonly known as "Remove K Digits" or "Maximum Number by Deleting Digits"
3. The greedy choice property holds: selecting the maximum digit at each position (with proper window constraints) leads to the global optimum
4. The window size calculation is critical: at each step, we must ensure enough digits remain to complete our selection

### Results

- Part 1: 17359
- Part 2: 172787336861064
