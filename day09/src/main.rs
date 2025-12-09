use utils::{Point, Solution, run_solution};

struct Day09;

impl Day09 {
    /// Check if a polygon segment intersects with a rectangle
    /// A segment intersects if it crosses through the interior of the rectangle
    fn segment_intersects_rect(
        seg: (Point<i64>, Point<i64>),
        rect: (Point<i64>, Point<i64>),
    ) -> bool {
        // Normalize rectangle to (min, max)
        let rect_min = Point::new(rect.0.x.min(rect.1.x), rect.0.y.min(rect.1.y));
        let rect_max = Point::new(rect.0.x.max(rect.1.x), rect.0.y.max(rect.1.y));

        if seg.0.x == seg.1.x {
            // Vertical segment
            let x = seg.0.x;

            // Segment must be strictly between left and right edges
            if !(rect_min.x < x && rect_max.x > x) {
                return false;
            }

            let seg_min_y = seg.0.y.min(seg.1.y);
            let seg_max_y = seg.0.y.max(seg.1.y);

            // Segment endpoint touches rectangle edge: not an intersection
            if seg_max_y == rect_min.y || seg_min_y == rect_max.y {
                return false;
            }

            // Check if segment crosses through top or bottom edge
            (seg_min_y..=seg_max_y).contains(&rect_min.y)
                || (seg_min_y..=seg_max_y).contains(&rect_max.y)
        } else if seg.0.y == seg.1.y {
            // Horizontal segment
            let y = seg.0.y;

            // Segment must be strictly between top and bottom edges
            if !(rect_min.y < y && rect_max.y > y) {
                return false;
            }

            let seg_min_x = seg.0.x.min(seg.1.x);
            let seg_max_x = seg.0.x.max(seg.1.x);

            // Segment endpoint touches rectangle edge: not an intersection
            if seg_max_x == rect_min.x || seg_min_x == rect_max.x {
                return false;
            }

            // Check if segment crosses through left or right edge
            (seg_min_x..=seg_max_x).contains(&rect_min.x)
                || (seg_min_x..=seg_max_x).contains(&rect_max.x)
        } else {
            // Diagonal segment (shouldn't happen)
            panic!("Diagonal segment encountered");
        }
    }
}

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

    fn part2(&self, data: &Self::Input) -> Self::Output {
        // Build all polygon segments (edges between consecutive red tiles)
        let mut segments = Vec::new();
        for i in 0..data.len() {
            let p1 = data[i];
            let p2 = data[(i + 1) % data.len()];

            segments.push((p1, p2));
        }

        // Find the largest rectangle that doesn't intersect any segment
        let mut max_area = 0_i64;

        for i in 0..data.len() {
            for j in (i + 1)..data.len() {
                let p1 = data[i];
                let p2 = data[j];

                // Check if this rectangle intersects any polygon segment
                let mut intersects = false;
                for &seg in &segments {
                    if Day09::segment_intersects_rect(seg, (p1, p2)) {
                        intersects = true;
                        break;
                    }
                }

                if !intersects {
                    let width = (p2.x - p1.x).abs() + 1;
                    let height = (p2.y - p1.y).abs() + 1;
                    let area = width * height;

                    max_area = max_area.max(area);
                }
            }
        }

        max_area
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

        assert_eq!(part2, 24);
    }
}
