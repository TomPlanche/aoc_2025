# Day 08: Playground

## Problem Description

You find yourself in a vast underground playground where Elves are setting up a Christmas decoration project using suspended electrical junction boxes. They need to connect junction boxes with strings of lights to allow electricity to flow throughout the system.

Each junction box has a position in 3D space (X,Y,Z coordinates). The Elves want to minimize the amount of string lights used by connecting the closest pairs of junction boxes first. When two junction boxes are connected, electricity can flow between them, and they become part of the same circuit.

Key rules:
- Connect pairs of junction boxes that are closest together based on straight-line distance
- When boxes are connected, they form circuits
- If two boxes are already in the same circuit, connecting them has no effect
- After making connections, determine the size of each resulting circuit

For Part 1, the task is to connect the 1000 pairs of junction boxes that are closest together, then multiply the sizes of the three largest circuits.

## Solution Approach

### Part 1

This is a classic minimum spanning tree problem solved using Kruskal's algorithm with a Union-Find (disjoint-set) data structure.

1. **Parse Input**: Extract 3D coordinates for all junction boxes
   - Each line contains X,Y,Z coordinates separated by commas

2. **Generate All Edges**: Calculate distances between every pair of junction boxes
   - Uses squared distance to avoid expensive square root calculations
   - Stores edges as (distance, point1, point2) tuples

3. **Sort Edges**: Sort all edges by distance in ascending order
   - This ensures we process closest pairs first

4. **Union-Find Algorithm**: Connect the 1000 closest pairs
   - Initialize each junction box as its own circuit (component)
   - For each of the 1000 shortest edges:
     - Use `union()` to merge the two circuits
     - If boxes are already in the same circuit, union does nothing
   - Union-Find optimizations:
     - Path compression: Flatten tree structure during `find()`
     - Union by size: Always attach smaller tree to larger tree

5. **Calculate Result**: Find sizes of all circuits and multiply the three largest
   - Use `get_component_sizes()` to count boxes in each circuit
   - Sort circuit sizes in descending order
   - Multiply the top 3 sizes together

**Example**: With 20 junction boxes and 10 connections:
- Results in 11 circuits: one with 5 boxes, one with 4, two with 2, and seven with 1
- Answer: 5 × 4 × 2 = 40

### Part 2

Not yet implemented.

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Notes

- The solution uses Union-Find (disjoint-set) data structure for efficient circuit tracking
- Squared distance is used instead of Euclidean distance since we only need relative ordering
- The algorithm is essentially Kruskal's MST but stops after 1000 edges instead of building a full spanning tree
- Union-Find operations are nearly O(1) with path compression and union by size optimizations
- Overall complexity: O(N² log N) where N is the number of junction boxes
  - O(N²) to generate all edges
  - O(N² log N²) to sort edges
  - O(1000 × α(N)) ≈ O(1000) for union operations where α is inverse Ackermann function
