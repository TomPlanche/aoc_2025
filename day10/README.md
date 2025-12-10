# Day 10: Factory

## Problem Description

The Elves have a factory with machines that need initialization. Each machine has:
- Indicator lights (initially all off) that need to match a target pattern
- Buttons that toggle specific lights when pressed
- A target configuration shown as `[.##.]` where `.` is off and `#` is on

Each button lists which lights it toggles (e.g., `(0,2,3)` toggles lights 0, 2, and 3). The goal is to find the minimum number of button presses needed to configure all machines correctly.

Example machine: `[.##.] (3) (1,3) (2) (2,3) (0,2) (0,1) {joltage}`
- 4 indicator lights with target: off, on, on, off
- 6 buttons that toggle different combinations of lights
- Minimum solution: 2 button presses

## Solution Approach

### Part 1

This is a classic "Lights Out" problem that can be solved using linear algebra over GF(2) (the binary field).

#### What is GF(2)?

GF(2) is a mathematical system with only two values (0 and 1) where:
- Addition is XOR: `1 + 1 = 0`, `1 + 0 = 1`, `0 + 0 = 0`
- Pressing a button twice = pressing it zero times (since `1 + 1 = 0` in GF(2))
- This means we only care about pressing each button 0 or 1 times

#### Simple Example

Let's solve a tiny machine: `[.##] (0,1) (1,2) (0,2)`

**Setup:**
- 3 lights (initially all OFF), target: `[OFF, ON, ON]` = `[0, 1, 1]`
- 3 buttons:
  - Button A: `(0,1)` toggles lights 0 and 1
  - Button B: `(1,2)` toggles lights 1 and 2
  - Button C: `(0,2)` toggles lights 0 and 2

**Building the equations:**

For each light, we need an equation showing which buttons affect it and what state we want.

**Light 0 analysis:**
- Starts: OFF (0)
- Affected by: Button A (toggles it), Button C (toggles it)
- Target: OFF (0)
- Equation: "Is light 0 toggled an even or odd number of times?"
  - If we press A, light 0 toggles once
  - If we press C, light 0 toggles once
  - If we press both A and C, light 0 toggles twice (back to OFF)
- Since we want light 0 to stay OFF (0), the total number of toggles must be even
- In GF(2): `A + C = 0` (A XOR C must equal 0)

**Light 1 analysis:**
- Starts: OFF (0)
- Affected by: Button A (toggles it), Button B (toggles it)
- Target: ON (1)
- Equation: "Light 1 must be toggled an odd number of times to end up ON"
  - If we press A only: light 1 = ON ✓
  - If we press B only: light 1 = ON ✓
  - If we press both A and B: light 1 toggles twice = OFF ✗
- Since we want light 1 to end ON (1), the total toggles must be odd
- In GF(2): `A + B = 1` (A XOR B must equal 1)

**Light 2 analysis:**
- Starts: OFF (0)
- Affected by: Button B (toggles it), Button C (toggles it)
- Target: ON (1)
- Equation: "Light 2 must be toggled an odd number of times to end up ON"
  - If we press B only: light 2 = ON ✓
  - If we press C only: light 2 = ON ✓
  - If we press both B and C: light 2 toggles twice = OFF ✗
- Since we want light 2 to end ON (1), the total toggles must be odd
- In GF(2): `B + C = 1` (B XOR C must equal 1)

**Complete system of equations (over GF(2)):**
```
Light 0: A     + C = 0  (even toggles → stays OFF)
Light 1: A + B     = 1  (odd toggles  → becomes ON)
Light 2:     B + C = 1  (odd toggles  → becomes ON)
```

**Key insight:** Each equation represents "which buttons affect this light" = "desired final state"

**Converting to matrix form:**

Now we translate these equations into matrix form `Ax = b (mod 2)`.

The matrix A has:
- **One row per light** (3 rows for 3 lights)
- **One column per button** (3 columns for buttons A, B, C)
- **Cell value = 1 if that button affects that light, 0 otherwise**

Let's build it step by step:

**Row 0 (Light 0 equation: A + C = 0):**
- Does button A affect light 0? YES → coefficient = 1
- Does button B affect light 0? NO  → coefficient = 0
- Does button C affect light 0? YES → coefficient = 1
- Target for light 0? OFF → 0
- Matrix row: `[1, 0, 1]` with target `0`

**Row 1 (Light 1 equation: A + B = 1):**
- Does button A affect light 1? YES → coefficient = 1
- Does button B affect light 1? YES → coefficient = 1
- Does button C affect light 1? NO  → coefficient = 0
- Target for light 1? ON → 1
- Matrix row: `[1, 1, 0]` with target `1`

**Row 2 (Light 2 equation: B + C = 1):**
- Does button A affect light 2? NO  → coefficient = 0
- Does button B affect light 2? YES → coefficient = 1
- Does button C affect light 2? YES → coefficient = 1
- Target for light 2? ON → 1
- Matrix row: `[0, 1, 1]` with target `1`

**Final matrix form:** `Ax = b (mod 2)`
```
     A  B  C
   ┌         ┐   ┌   ┐   ┌   ┐
L0 │ 1  0  1 │   │ A │   │ 0 │
L1 │ 1  1  0 │ × │ B │ = │ 1 │
L2 │ 0  1  1 │   │ C │   │ 1 │
   └         ┘   └   ┘   └   ┘
     Matrix A      x       b
```

**Verify with row 1:**
```
Row 1: [1, 1, 0] × [A, B, C]ᵀ = 1×A + 1×B + 0×C = A + B = 1 ✓
```

This is exactly our equation for Light 1!

**Solve using Gaussian elimination:**
```
Step 1: Keep row 0 as pivot
[ 1  0  1 | 0 ]
[ 1  1  0 | 1 ]
[ 0  1  1 | 1 ]

Step 2: Eliminate column 0 (XOR row 0 into row 1)
[ 1  0  1 | 0 ]
[ 0  1  1 | 1 ]  ← row1 = row1 XOR row0
[ 0  1  1 | 1 ]

Step 3: Eliminate column 1 (XOR row 1 into row 2)
[ 1  0  1 | 0 ]
[ 0  1  1 | 1 ]
[ 0  0  0 | 0 ]  ← row2 = row2 XOR row1 = all zeros!
```

**Result:** C is a free variable! We can set it to 0 or 1:
- If C=0: A=0, B=1 → Press button B only (1 press)
- If C=1: A=1, B=0 → Press buttons A and C (2 presses)

**Minimum solution: 1 button press (press button B)**

This is exactly what the algorithm does: find all possible solutions and pick the one with fewest button presses!

#### Algorithm Steps

1. Parse Input:
   - Extract target light pattern from `[...]`
   - Parse button definitions from `(...)` parentheses
   - Ignore joltage requirements in `{...}`

2. Mathematical Formulation:
   - This is a system of linear equations over GF(2): `Ax = b (mod 2)`
   - `A` is a matrix where column j represents button j (which lights it toggles)
   - `x` is which buttons to press (binary vector)
   - `b` is the target configuration
   - Since toggling twice = no effect, we only care about pressing each button 0 or 1 times

3. Solution Algorithm:
   - Use Gaussian elimination over GF(2) to reduce the system
   - Identify pivot variables (basic) and free variables
   - For systems with multiple solutions (free variables exist):
     - Try all 2^k combinations of free variable assignments
     - Compute corresponding solution for each combination
     - Return the solution with minimum Hamming weight (fewest button presses)

4. Implementation Details:
   - XOR operations for addition in GF(2) (since 1+1=0 mod 2)
   - Exhaustive search over free variables ensures optimal solution

The key insight is that this problem maps perfectly to solving a system of linear equations over GF(2), where the challenge is finding the minimum weight solution when the system is underdetermined.

### Part 2

To be implemented after Part 1 submission.

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

The solution uses a combination of:
- Gaussian elimination for solving the linear system
- Exhaustive search over free variables for optimization
- Efficient bit manipulation for GF(2) arithmetic (XOR operations)

For machines with many free variables, the exponential search could theoretically be slow, but in practice AOC inputs are constrained enough that this approach works efficiently.
