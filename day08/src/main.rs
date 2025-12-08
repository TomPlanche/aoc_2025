use utils::{Point3D, Solution, run_solution};

/// Disjoint-set data structure (Union-Find) for tracking connected components.
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        Self {
            parent: (0..n).collect(),
            size: vec![1; n],
        }
    }

    /// Finds the root (representative) of the set containing element `x`.
    ///
    /// Uses path compression optimization: during the traversal to find the root,
    /// each node's parent is updated to point to its grandparent, flattening the
    /// tree structure. This makes future `find` operations faster.
    ///
    fn find(&mut self, mut x: usize) -> usize {
        while self.parent[x] != x {
            self.parent[x] = self.parent[self.parent[x]];
            x = self.parent[x];
        }

        x
    }

    /// Merges the sets containing elements `x` and `y`.
    ///
    /// Uses union by size optimization: always attaches the smaller tree to the
    /// root of the larger tree.
    fn union(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            return false;
        }

        if self.size[root_x] < self.size[root_y] {
            self.parent[root_x] = root_y;
            self.size[root_y] += self.size[root_x];
        } else {
            self.parent[root_y] = root_x;
            self.size[root_x] += self.size[root_y];
        }

        true
    }

    fn get_component_sizes(&mut self) -> Vec<usize> {
        let n = self.parent.len();
        let mut sizes = std::collections::HashMap::new();

        for i in 0..n {
            let root = self.find(i);
            *sizes.entry(root).or_insert(0) += 1;
        }

        sizes.values().copied().collect()
    }
}

fn solve_with_edges(data: &[Point3D<i32>], max_edges: usize) -> i64 {
    let n = data.len();
    let mut edges: Vec<(i64, usize, usize)> = Vec::new();

    // Calculate distances between all pairs of points
    for i in 0..n {
        for j in (i + 1)..n {
            let dist = data[i].distance_squared(&data[j]);

            edges.push((dist, i, j));
        }
    }

    // Sort edges by distance
    edges.sort_unstable_by_key(|&(dist, _, _)| dist);

    let mut uf = UnionFind::new(n);

    // Merge connected components
    for (_, i, j) in edges.iter().take(max_edges) {
        uf.union(*i, *j);
    }

    let mut sizes = uf.get_component_sizes();

    // Sort sizes in descending order
    sizes.sort_unstable_by(|a, b| b.cmp(a));

    i64::try_from(sizes[0]).unwrap()
        * i64::try_from(sizes[1]).unwrap()
        * i64::try_from(sizes[2]).unwrap()
}

struct Day08;

/// Implementation of the Solution trait for Day 8.
impl Solution for Day08 {
    type Input = Vec<Point3D<i32>>;
    type Output = i64;

    fn parse_input(&self, input: &str) -> Self::Input {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parts: Vec<i32> = line.split(',').filter_map(|s| s.parse().ok()).collect();

                Point3D {
                    x: parts[0],
                    y: parts[1],
                    z: parts[2],
                }
            })
            .collect()
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        solve_with_edges(data, 1000)
    }

    fn part2(&self, _data: &Self::Input) -> Self::Output {
        0
    }
}

fn main() {
    run_solution!(Day08);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "162,817,812
57,618,57
906,360,560
592,479,940
352,342,300
466,668,158
542,29,236
431,825,988
739,650,466
52,470,668
216,146,977
819,987,18
117,168,530
805,96,715
346,949,466
970,615,88
941,993,340
862,61,35
984,92,344
425,690,689";

        let day = Day08;
        let parsed_input = day.parse_input(input);

        let result = solve_with_edges(&parsed_input, 10);

        assert_eq!(result, 40);
    }

    #[test]
    fn test_part2() {
        let input = "";

        let day = Day08;
        let parsed_input = day.parse_input(input);

        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 0);
    }
}
