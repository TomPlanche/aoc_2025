# Day 07: Laboratories

## Problem Description

You find yourself in a teleporter lab where a malfunctioning teleporter needs repair. The diagnostic tool shows an issue with the tachyon manifold.

A tachyon beam enters the manifold at position 'S' and always moves downward. When a beam encounters a splitter ('^'), it stops and two new beams are emitted from the immediate left and right of the splitter, continuing downward.

Key rules:
- Beams pass through empty space ('.')
- When a beam hits a splitter ('^'), it stops and creates two new beams
- New beams start from the positions immediately left and right of the splitter
- When multiple beams reach the same position, they merge into one beam
- Each splitter only counts once, even if hit by multiple beams

The task is to count how many times the beam is split as it travels through the manifold.

## Solution Approach

### Part 1

1. Parse the grid to find the starting position 'S'
2. Use BFS to simulate beam movement:
   - Start with a beam at S moving downward
   - For each beam, move downward until hitting a splitter or exiting
   - When hitting a splitter, create two new beams at adjacent columns
3. Track two important sets:
   - `seen`: positions where beams have already been processed (prevents duplicate work when beams merge)
   - `splitters_hit`: splitters that have been activated (each counts only once)
4. Count each unique splitter activation

The implementation handles beam merging by skipping positions already seen, ensuring each path is only explored once.

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

The key challenge was understanding that when multiple beams hit the same splitter, it should only count as one split event, not multiple. This represents the physical reality of the splitter - once activated, it's done its job regardless of how many beams reach it.
