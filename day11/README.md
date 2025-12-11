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

The solution uses a depth-first search (DFS) with backtracking to count all distinct paths from "you" to "out".

Key implementation details:
1. Parse the input to build a directed graph (adjacency list) using a `HashMap<String, Vec<String>>`
2. Use DFS with a visited set to track nodes in the current path (prevents cycles)
3. When reaching the target node "out", increment the path counter
4. Backtrack by removing the current node from visited set, allowing it to be visited in other paths

The algorithm explores all possible paths without revisiting nodes within a single path, but allows the same node to appear in different paths.

### Part 2

Not yet revealed.

## Running

```bash
cargo run
```

## Testing

```bash
cargo test
```

## Notes

This is a classic graph path-counting problem. The key insight is using backtracking to properly handle the visited set - adding nodes when exploring and removing them when backtracking ensures all valid paths are counted.
