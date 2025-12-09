use utils::{Point, Solution, run_solution};

struct Day09;

impl Solution for Day09 {
    type Input = Vec<Point<i64>>;
    type Output = i64;

    fn parse_input(&self, input: &str) -> Self::Input {
        input
            .lines()
            .filter(|line| !line.is_empty())
            .map(|line| {
                let parts: Vec<i64> = line
                    .split(',')
                    .filter_map(|s| s.trim().parse().ok())
                    .collect();

                Point::new(parts[0], parts[1])
            })
            .collect()
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        let mut max_area = 0_i64;

        // Check all pairs of red tiles
        for (i, p1) in data.iter().enumerate() {
            for p2 in data.iter().skip(i + 1) {
                // Calculate rectangle area using these two points as opposite corners
                // Add 1 to include both corners in the count
                let width = (p2.x - p1.x).abs() + 1;
                let height = (p2.y - p1.y).abs() + 1;
                let area = width * height;

                max_area = max_area.max(area);
            }
        }

        max_area
    }

    fn part2(&self, _data: &Self::Input) -> Self::Output {
        // TODO: Implement part 2
        0
    }
}

fn main() {
    run_solution!(Day09);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let day = Day09;
        let parsed_input = day.parse_input(input);

        let part1 = day.part1(&parsed_input);

        assert_eq!(part1, 50);
    }

    #[test]
    fn test_part2() {
        let input = "7,1
11,1
11,7
9,7
9,5
2,5
2,3
7,3";

        let day = Day09;
        let parsed_input = day.parse_input(input);

        let part2 = day.part2(&parsed_input);

        assert_eq!(part2, 0_i64);
    }
}
