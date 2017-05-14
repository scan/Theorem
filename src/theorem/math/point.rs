use std::ops::{Add, Sub, Mul};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Add for Point {
    type Output = Point;

    fn add(self, other: Point) -> Point {
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

impl Sub for Point {
    type Output = Point;

    fn sub(self, other: Point) -> Point {
        Point { x: self.x - other.x, y: self.y - other.y }
    }
}

impl Mul<i32> for Point {
    type Output = Point;

    fn mul(self, s: i32) -> Point {
        Point { x: self.x * s, y: self.y * s }
    }
}

#[cfg(test)]
mod tests {
    use theorem::math::{Point};

    #[test]
    fn add_point() {
        let v1 = Point { x: 1, y: 1 };
        let v2 = Point { x: 2, y: 2 };
        assert_eq!(v1 + v1, v2);
    }

    #[test]
    fn sub_point() {
        let v1 = Point { x: 1, y: 1 };
        let v2 = Point { x: 0, y: 0 };
        assert_eq!(v1 - v1, v2);
    }

    #[test]
    fn mul_point() {
        let v1 = Point { x: 1, y: 1 };
        let v2 = Point { x: 2, y: 2 };
        assert_eq!(v1 * 2, v2);
    }
}
