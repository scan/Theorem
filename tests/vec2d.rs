extern crate theorem;

use theorem::math::{Vec2D};

#[test]
fn add_vector() {
    let v1 = Vec2D { x: 1, y: 1 };
    let v2 = Vec2D { x: 2, y: 2 };
    assert_eq!(v1 + v1, v2);
}
