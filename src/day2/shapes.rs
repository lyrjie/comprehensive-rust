#![allow(unused_variables, dead_code)]

use std::f64::consts::PI;
use std::ops::Add;
use std::slice::Iter;

#[derive(Copy, Clone, Debug)]
pub struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Point { x, y }
    }

    fn magnitude(&self) -> f64 {
        self.dist(&Point { x: 0, y: 0 })
    }

    fn dist(&self, other: &Point) -> f64 {
        ((f64::from(self.x) - f64::from(other.x)).powi(2)
            + (f64::from(self.y) - f64::from(other.y)).powi(2))
            .sqrt()
    }
}

impl Add for Point {
    type Output = Point;

    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

#[derive(Debug)]
pub struct Polygon {
    points: Vec<Point>,
}

impl Polygon {
    fn new() -> Self {
        Polygon { points: vec![] }
    }

    fn add_point(&mut self, point: Point) {
        self.points.push(point)
    }

    fn left_most_point(&self) -> Option<Point> {
        self.points
            .iter()
            .min_by(|left, right| left.y.cmp(&right.y))
            .copied()
    }

    fn iter(&self) -> Iter<'_, Point> {
        self.points.iter()
    }
}

pub struct Circle {
    center: Point,
    radius: i16,
}

impl Circle {
    fn new(center: Point, radius: i16) -> Self {
        Circle { center, radius }
    }
}

pub enum Shape {
    Polygon(Polygon),
    Circle(Circle),
}

impl Shape {
    fn circumference(&self) -> f64 {
        match &self {
            Shape::Polygon(polygon) => {
                let mut sum: f64 = polygon
                    .points
                    .first()
                    .unwrap()
                    .dist(polygon.points.last().unwrap());
                for pair in polygon.points.windows(2) {
                    sum += pair[0].dist(&pair[1])
                }
                sum
            }
            Shape::Circle(circle) => f64::from(2 * circle.radius) * PI,
        }
    }
}

impl From<Polygon> for Shape {
    fn from(value: Polygon) -> Self {
        Shape::Polygon(value)
    }
}

impl From<Circle> for Shape {
    fn from(value: Circle) -> Self {
        Shape::Circle(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn round_two_digits(x: f64) -> f64 {
        (x * 100.0).round() / 100.0
    }

    #[test]
    fn test_point_magnitude() {
        let p1 = Point::new(12, 13);
        assert_eq!(round_two_digits(p1.magnitude()), 17.69);
    }

    #[test]
    fn test_point_dist() {
        let p1 = Point::new(10, 10);
        let p2 = Point::new(14, 13);
        assert_eq!(round_two_digits(p1.dist(&p2)), 5.00);
    }

    #[test]
    fn test_point_add() {
        let p1 = Point::new(16, 16);
        let p2 = p1 + Point::new(-4, 3);
        assert_eq!(p2, Point::new(12, 19));
    }

    #[test]
    fn test_polygon_left_most_point() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);
        assert_eq!(poly.left_most_point(), Some(p1));
    }

    #[test]
    fn test_polygon_iter() {
        let p1 = Point::new(12, 13);
        let p2 = Point::new(16, 16);

        let mut poly = Polygon::new();
        poly.add_point(p1);
        poly.add_point(p2);

        let points = poly.iter().cloned().collect::<Vec<_>>();
        assert_eq!(points, vec![Point::new(12, 13), Point::new(16, 16)]);
    }

    #[test]
    fn test_shape_circumferences() {
        let mut poly = Polygon::new();
        poly.add_point(Point::new(12, 13));
        poly.add_point(Point::new(17, 11));
        poly.add_point(Point::new(16, 16));
        let shapes = vec![
            Shape::from(poly),
            Shape::from(Circle::new(Point::new(10, 20), 5)),
        ];
        let circumferences = shapes
            .iter()
            .map(Shape::circumference)
            .map(round_two_digits)
            .collect::<Vec<_>>();
        assert_eq!(circumferences, vec![15.48, 31.42]);
    }
}
