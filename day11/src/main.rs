use std::collections::HashMap;
use utils::{Solution, run_solution};

type Graph<'a> = HashMap<&'a str, Vec<&'a str>>;

struct Day11;

impl Solution for Day11 {
    type Input = String;
    type Output = u64;

    fn parse_input(&self, input: &str) -> Self::Input {
        input.to_string()
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        let graph = parse_graph(data);
        let mut memo = HashMap::new();

        count_paths("you", "out", &graph, &mut memo)
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        let graph = parse_graph(data);
        let mut memo = HashMap::new();

        // Calculate two different routes: svr -> dac -> fft -> out and svr -> fft -> dac -> out
        let route1 = count_paths("svr", "dac", &graph, &mut memo)
            * count_paths("dac", "fft", &graph, &mut memo)
            * count_paths("fft", "out", &graph, &mut memo);

        let route2 = count_paths("svr", "fft", &graph, &mut memo)
            * count_paths("fft", "dac", &graph, &mut memo)
            * count_paths("dac", "out", &graph, &mut memo);

        route1 + route2
    }
}

fn parse_graph(input: &str) -> Graph<'_> {
    let mut graph: Graph = HashMap::new();

    for line in input.lines() {
        if line.trim().is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split(": ").collect();
        if parts.len() == 2 {
            let key = parts[0];
            let neighbors: Vec<&str> = parts[1].split_whitespace().collect();
            graph.entry(key).or_default().extend(neighbors);
        }
    }

    graph
}

fn count_paths<'a>(
    curr: &'a str,
    target: &'a str,
    graph: &Graph<'a>,
    memo: &mut HashMap<(&'a str, &'a str), u64>,
) -> u64 {
    // Base case: reached target
    if curr == target {
        return 1;
    }

    // Check memoization cache
    let key = (curr, target);
    if let Some(&count) = memo.get(&key) {
        return count;
    }

    // Count paths through all neighbors
    let mut total = 0;
    if let Some(neighbors) = graph.get(curr) {
        for &next in neighbors {
            total += count_paths(next, target, graph, memo);
        }
    }

    // Cache result
    memo.insert(key, total);
    total
}

fn main() {
    run_solution!(Day11);
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_PART1: &str = "aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out";

    const EXAMPLE_PART2: &str = "svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out";

    #[test]
    fn test_part1() {
        let day = Day11;
        let input = day.parse_input(EXAMPLE_PART1);
        let result = day.part1(&input);
        assert_eq!(result, 5);
    }

    #[test]
    fn test_part2() {
        let day = Day11;
        let input = day.parse_input(EXAMPLE_PART2);
        let result = day.part2(&input);
        assert_eq!(result, 2);
    }
}
