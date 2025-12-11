# Day 11: Reactor

## Problem Description

The task involves analyzing data flow through a network of devices in a reactor. Each device has outputs that connect to other devices, and data flows unidirectionally through these connections.

Given a list of devices and their outputs, the challenge is to find all possible paths that data can take from a starting device (labeled "you") to an ending device (labeled "out").

For example, given:
```
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
```

Data can flow through 5 different paths from "you" to "out":
1. you -> bbb -> ddd -> ggg -> out
2. you -> bbb -> eee -> out
3. you -> ccc -> ddd -> ggg -> out
4. you -> ccc -> eee -> out
5. you -> ccc -> fff -> out

## Solution Approach

### Part 1

The solution uses recursive path counting with memoization on a directed acyclic graph (DAG).

Key implementation details:
1. Parse input to build a graph using `HashMap<&str, Vec<&str>>` (string slices for efficiency)
2. Use recursive function `count_paths(curr, target)` that:
   - Returns 1 if curr == target (found a path)
   - Otherwise, sums paths from all neighbors to target
3. Memoize results with `HashMap<(&str, &str), u64>` to avoid recalculating
4. No visited set needed since the graph is acyclic

The key insight is that this is a DAG, so we can use memoization without worrying about cycles. This is much more efficient than backtracking with visited sets.

### Part 2

Part 2 asks for paths from "svr" to "out" that go through both "dac" and "fft" (in any order).

The solution recognizes there are two possible orderings:
1. svr -> dac -> fft -> out
2. svr -> fft -> dac -> out

For each route, we multiply the path counts between consecutive nodes:
- Route 1: `count(svr->dac) * count(dac->fft) * count(fft->out)`
- Route 2: `count(svr->fft) * count(fft->dac) * count(dac->out)`

The final answer is the sum of both routes. Memoization ensures we don't recalculate shared segments.

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Notes

This is a classic DAG path-counting problem with several key insights:

1. **Use string slices (`&str`)**: More efficient than owned `String` values with proper lifetime management
2. **Memoization over backtracking**: Since it's a DAG (no cycles), we can cache results instead of using visited sets
3. **Part 2 is multiplicative**: Count paths along specific routes and sum the results, not DFS with required nodes

The memoization approach is dramatically faster than backtracking and produces the same results on DAGs.
