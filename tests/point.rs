extern crate theorem;

use theorem::math::{Point};

#[test]
fn add_point() {
    let v1 = Point { x: 1, y: 1 };
    let v2 = Point { x: 2, y: 2 };
    assert_eq!(v1 + v1, v2);
}
