use std::ops::{Add, Sub, Mul, Neg};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct Point {
    x: i32,
    y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Point {
        Point { x: x, y: y }
    }
}

impl Neg for Point {
    type Output = Point;

    fn neg(self) -> Point {
        Point { x: -self.x, y: -self.y }
    }
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
    fn neg_point() {
        let v1 = Point::new(1, 1);
        let v2 = Point::new(-1, -1);
        assert_eq!(-v1, v2);
    }

    #[test]
    fn add_point() {
        let v1 = Point::new(1, 1);
        let v2 = Point::new(2, 2);
        assert_eq!(v1 + v1, v2);
    }

    #[test]
    fn sub_point() {
        let v1 = Point::new(1, 1);
        let v2 = Point::new(0, 0);
        assert_eq!(v1 - v1, v2);
    }

    #[test]
    fn mul_point() {
        let v1 = Point::new(1, 1);
        let v2 = Point::new(2, 2);
        assert_eq!(v1 * 2, v2);
    }
}
