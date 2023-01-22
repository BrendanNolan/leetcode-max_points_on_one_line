use std::collections::HashMap;

mod rational {
    #[derive(Copy, Clone, Debug, Hash, Eq, PartialEq)]
    pub struct RationalNumber {
        numerator: i32,
        denominator: i32,
    }

    impl RationalNumber {
        pub fn new(mut numerator: i32, mut denominator: i32) -> Self {
            let gcd = greatest_common_divisor(numerator, denominator);
            numerator /= gcd;
            denominator /= gcd;
            Self {
                numerator,
                denominator,
            }
        }

        pub fn numerator(&self) -> i32 {
            self.numerator
        }

        pub fn denominator(&self) -> i32 {
            self.denominator
        }
    }

    fn greatest_common_divisor(a: i32, b: i32) -> i32 {
        if a == 0 {
            return b;
        } else if b == 0 {
            return a;
        }
        let a = a.abs();
        let b = b.abs();
        let remainder = a % b;
        greatest_common_divisor(b, remainder)
    }
}

use rational::RationalNumber;

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Point {
    x: i32,
    y: i32,
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
enum Slope {
    Undefined,
    Defined(RationalNumber),
}

fn slope(a: &Point, b: &Point) -> Slope {
    if a.x == b.x {
        Slope::Undefined
    } else {
        Slope::Defined(RationalNumber::new(a.y - b.y, a.x - b.x))
    }
}

#[derive(Copy, Clone, Debug, Hash, PartialEq, Eq)]
struct Line {
    slope: Slope,
    x_intercect: i32,
}

impl Line {
    fn new(a: &Point, b: &Point) -> Self {
        match slope(a, b) {
            Slope::Undefined => Line {
                slope: Slope::Undefined,
                x_intercect: a.x,
            },
            Slope::Defined(slope) => Line {
                slope: Slope::Defined(slope),
                x_intercect: a.y - (slope.numerator() * a.x / slope.denominator()),
            },
        }
    }
}

fn collinear_groups(points: &[Point]) -> HashMap<Point, Vec<Point>> {
    points
        .iter()
        .map(|point| (*point, collinear_group(point, points)))
        .collect()
}

fn collinear_group(point: &Point, all_points: &[Point]) -> Vec<Point> {
    let lines_containing_point = lines_containing(point, all_points);
    match lines_containing_point
        .iter()
        .max_by(|p, q| p.1.len().cmp(&q.1.len()))
    {
        Some((_line, points)) => points.clone(),
        None => vec![*point],
    }
}

fn lines_containing(point: &Point, all_points: &[Point]) -> HashMap<Line, Vec<Point>> {
    let mut lines = HashMap::<Line, Vec<Point>>::new();
    all_points
        .iter()
        .filter(|other_point| *other_point != point)
        .for_each(|other_point| {
            let line = Line::new(point, other_point);
            if let Some(points) = lines.get_mut(&line) {
                points.push(*other_point);
            } else {
                lines.insert(line, vec![*point, *other_point]);
            }
        });
    lines
}

fn max_collinear_points(raw_points: Vec<Vec<i32>>) -> i32 {
    let points: Vec<Point> = raw_points
        .iter()
        .map(|p| Point { x: p[0], y: p[1] })
        .collect();
    let collinear_groups = collinear_groups(&points);
    let (_point, collinear_points) = collinear_groups
        .iter()
        .max_by(|p, q| p.1.len().cmp(&q.1.len()))
        .unwrap();
    collinear_points.len() as i32
}

fn main() {
    assert_eq!(
        max_collinear_points(vec![vec![0, 0], vec![1, 1], vec![2, 2]]),
        3
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_0() {
        let max_points = max_collinear_points(vec![vec![0, 0]]);
        assert_eq!(max_points, 1);
    }

    #[test]
    fn test_1() {
        let max_points = max_collinear_points(vec![vec![1, 1], vec![2, 2], vec![3, 3]]);
        assert_eq!(max_points, 3);
    }

    #[test]
    fn test_2() {
        let max_points = max_collinear_points(vec![
            vec![1, 1],
            vec![3, 2],
            vec![5, 3],
            vec![4, 1],
            vec![2, 3],
            vec![1, 4],
        ]);
        assert_eq!(max_points, 4);
    }
}
