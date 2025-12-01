use utils::{run_solution, Solution};

struct Day01;

impl Solution for Day01 {
    type Input = Vec<(char, i32)>;
    type Output = i32;

    fn parse_input(&self, input: &str) -> Self::Input {
        input
            .lines()
            .filter_map(|line| {
                let line = line.trim();
                if line.is_empty() {
                    return None;
                }
                let direction = line.chars().next()?;
                let distance = line[1..].parse().ok()?;
                Some((direction, distance))
            })
            .collect()
    }

    fn part1(&self, data: &Self::Input) -> Self::Output {
        let mut position = 50;
        let mut count = 0;

        for &(direction, distance) in data {
            match direction {
                'L' => {
                    position = (position - distance).rem_euclid(100);
                }
                'R' => {
                    position = (position + distance) % 100;
                }
                _ => {}
            }

            if position == 0 {
                count += 1;
            }
        }

        count
    }

    fn part2(&self, data: &Self::Input) -> Self::Output {
        let mut position = 50;
        let mut count = 0;

        for &(direction, distance) in data {
            match direction {
                'L' => {
                    if position > 0 {
                        count += (distance + 100 - position) / 100;
                    } else {
                        count += distance / 100;
                    }
                    position = (position - distance).rem_euclid(100);
                }
                'R' => {
                    count += (position + distance) / 100;
                    position = (position + distance) % 100;
                }
                _ => {}
            }
        }

        count
    }
}

fn main() {
    run_solution!(Day01);
}
