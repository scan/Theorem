use std::ops::{Add, Sub};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Point {
    x: i32,
    y: i32,
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
        Point { x: self.x + other.x, y: self.y + other.y }
    }
}

#[test]
fn add_point() {
    let v1 = Point { x: 1, y: 1 };
    let v2 = Point { x: 2, y: 2 };
    assert_eq!(v1 + v1, v2);
}
