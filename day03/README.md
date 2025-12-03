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

### Part 2

[Describe your approach to solving part 2]

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Notes

[Add any additional notes, observations, or learnings from this challenge]
