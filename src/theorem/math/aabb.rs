#[derive(Copy, Clone, Debug, Eq)]
pub struct AABB { // Axis-Aligned Bounding Box
    top: i32,
    left: i32,
    bottom: i32,
    right: i32
}

impl AABB {
    fn width(self) {
        right - left;
    }

    fn height(self) {
        bottom - top;
    }

    fn topleft(self) {
        Point::new(left, top);
    }

    fn bottomright(self) {
        Point::new(right, bottom);
    }

    fn area(self) {
        self.width * self.height;
    }
}
