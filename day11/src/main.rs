use std::collections::{HashMap, HashSet};
use utils::{Solution, run_solution};

type Graph = HashMap<String, Vec<String>>;

struct Day11;

impl Solution for Day11 {
    type Input = Graph;
    type Output = usize;

    fn parse_input(&self, input: &str) -> Self::Input {
        let mut graph = HashMap::new();

        for line in input.lines() {
            if line.trim().is_empty() {
                continue;
            }

            let parts: Vec<&str> = line.split(": ").collect();
            if parts.len() == 2 {
                let device = parts[0].to_string();
                let outputs: Vec<String> = parts[1]
                    .split_whitespace()
                    .map(std::string::ToString::to_string)
                    .collect();

                graph.insert(device, outputs);
            }
        }

        graph
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        count_paths(data, "you", "out")
    }

    fn part2(&self, _data: &Self::Input) -> Self::Output {
        0
    }
}

fn count_paths(graph: &Graph, start: &str, end: &str) -> usize {
    let mut visited = HashSet::new();

    dfs(graph, start, end, &mut visited)
}

fn dfs(graph: &Graph, current: &str, target: &str, visited: &mut HashSet<String>) -> usize {
    if current == target {
        return 1;
    }

    if visited.contains(current) {
        return 0;
    }

    visited.insert(current.to_string());

    let mut total_paths = 0;

    if let Some(outputs) = graph.get(current) {
        for output in outputs {
            total_paths += dfs(graph, output, target, visited);
        }
    }

    visited.remove(current);

    total_paths
}

fn main() {
    run_solution!(Day11);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    #[test]
    fn test_part1() {
        let day = Day11;
        let parsed_input = day.parse_input(EXAMPLE);

        let part1 = day.part1(&parsed_input);

        assert_eq!(part1, 5);
    }

    #[test]
    fn test_part2() {
        let day = Day11;
        let parsed_input = day.parse_input(EXAMPLE);

        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 0);
    }
}
