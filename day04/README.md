# Day 04

## Problem Description

--- Day 4: Printing Department ---

You ride the escalator down to the printing department. They're clearly getting ready for Christmas; they have lots of large rolls of paper everywhere, and there's even a massive printer in the corner (to handle the really big print jobs).

Decorating here will be easy: they can make their own decorations. What you really need is a way to get further into the North Pole base while the elevators are offline.

"Actually, maybe we can help with that," one of the Elves replies when you ask for help. "We're pretty sure there's a cafeteria on the other side of the back wall. If we could break through the wall, you'd be able to keep moving. It's too bad all of our forklifts are so busy moving those big rolls of paper around."

If you can optimize the work the forklifts are doing, maybe they would have time to spare to break through the wall.

The rolls of paper (@) are arranged on a large grid; the Elves even have a helpful diagram (your puzzle input) indicating where everything is located.

For example:

```
..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.
```

The forklifts can only access a roll of paper if there are fewer than four rolls of paper in the eight adjacent positions. If you can figure out which rolls of paper the forklifts can access, they'll spend less time looking and more time breaking down the wall to the cafeteria.

In this example, there are 13 rolls of paper that can be accessed by a forklift (marked with x):

```
..xx.xx@x.
x@@.@.@.@@
@@@@@.x.@@
@.@@@@..@.
x@.@@@@.@x
.@@@@@@@.@
.@.@.@.@@@
x.@@@.@@@@
.@@@@@@@@.
x.x.@@@.x.
```

Consider your complete diagram of the paper roll locations. How many rolls of paper can be accessed by a forklift?

## Solution Approach

### Part 1: Counting Accessible Paper Rolls

The problem requires us to find all paper rolls (@) that have fewer than 4 adjacent paper rolls in the 8 surrounding positions (including diagonals).

#### Algorithm

1. Parse the input into a 2D grid of characters
2. For each cell in the grid:
   - If the cell contains a paper roll (@), check all 8 adjacent positions
   - Count how many of the adjacent positions also contain paper rolls
   - If the count is less than 4, the roll is accessible
3. Return the total count of accessible rolls

#### Implementation Details

The solution uses the `Direction` enum from the utils crate, which provides clean, semantic access to the 8 cardinal and diagonal directions:
- `Direction::Up`, `Direction::Down`, `Direction::Left`, `Direction::Right`
- `Direction::UpLeft`, `Direction::UpRight`, `Direction::DownLeft`, `Direction::DownRight`

For each direction, we:
1. Convert it to (dx, dy) coordinates using the `From<Direction> for (isize, isize)` trait
2. Calculate the new position
3. Check bounds and if the position contains a paper roll
4. Count the adjacent rolls and determine accessibility

#### Example Walkthrough

For the test case:
```
..@@.@@@@.
@@@.@.@.@@
```

- The @ at position (0,2) has 2 adjacent rolls: (0,3) and (1,2) → accessible (< 4)
- The @ at position (1,0) has 2 adjacent rolls: (0,1) and (1,1) → accessible (< 4)
- The @ at position (1,1) has 4 adjacent rolls: (0,1), (0,2), (1,0), (1,2) → not accessible (= 4)

### Part 2: Iterative Paper Roll Removal

Part 2 extends the problem: once accessible rolls are removed, more rolls may become accessible. We need to count the total number of rolls that can be removed through this iterative process.

#### Algorithm

The solution uses a simulation approach:

1. Clone the grid to create a working copy
2. Repeat until no more rolls can be removed:
   - Find all accessible rolls (those with < 4 adjacent rolls)
   - Remove all accessible rolls simultaneously
   - Count the removed rolls
3. Return the total count of removed rolls

#### Key Insight

This is a cascading removal problem. When rolls are removed in one iteration, their former neighbors may now have fewer adjacent rolls, making them accessible in the next iteration.

#### Implementation Details

The algorithm removes all accessible rolls in each iteration simultaneously (not one by one). This is important because:
- It matches the problem description's behavior
- It ensures consistency within each iteration
- All rolls that are accessible at the start of an iteration are removed before checking for new accessible rolls

#### Example Walkthrough

For the test case, the removal happens in phases:
1. Initial state: 13 accessible rolls → remove them
2. After removal: 12 new rolls become accessible → remove them
3. Continue this process: 7 → 5 → 2 → 1 → 1 → 1 → 1
4. Total removed: 13 + 12 + 7 + 5 + 2 + 1 + 1 + 1 + 1 = 43

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Notes

### Code Organization

The solution refactored common functionality:
- `DIRECTIONS`: Constant array of all 8 directions using the utils `Direction` enum
- `count_adjacent_rolls()`: Helper function to count adjacent paper rolls for a given position
- `count_accessible_rolls()`: Counts accessible rolls in a single snapshot (Part 1)
- `count_removable_rolls()`: Simulates iterative removal process (Part 2)
