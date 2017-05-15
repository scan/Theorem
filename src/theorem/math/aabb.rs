use theorem::math::{Point};
use std::cmp::{min, max};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct AABB { // Axis-Aligned Bounding Box
    pub top: i32,
    pub left: i32,
    pub bottom: i32,
    pub right: i32
}

impl AABB {
    pub fn width(self) -> i32 {
        return self.right - self.left;
    }

    pub fn height(self) -> i32 {
        return self.bottom - self.top;
    }

    pub fn topleft(self) -> Point {
        return Point::new(self.left, self.top);
    }

    pub fn bottomright(self) -> Point {
        return Point::new(self.right, self.bottom);
    }

    pub fn area(self) -> i32 {
        return self.width() * self.height();
    }

    pub fn merge(self, other: AABB) -> AABB {
        return AABB {
            top: min(self.top, other.top),
            left: min(self.left, other.left),
            bottom: max(self.bottom, other.bottom),
            right: max(self.right, other.right)
        }
    }

    pub fn split(self) -> (AABB, AABB, AABB, AABB) {
        let t2 = self.top / 2;
        let l2 = self.left / 2;
        let b2 = self.bottom / 2;
        let r2 = self.right / 2;

        let h2 = self.height() / 2;
        let w2 = self.width() / 2;

        return (
            AABB { top: t2, left: l2, bottom: b2, right: r2 },
            AABB { top: t2, left: l2 + w2, bottom: b2, right: self.right },
            AABB { top: t2 + h2, left: l2, bottom: self.bottom, right: r2 },
            AABB { top: t2 + h2, left: l2 + w2, bottom: self.bottom, right: self.right }
        );
    }

    pub fn collides(self, other: AABB) -> bool {
        if self.left < other.right &&
           self.right > other.left &&
           self.top < other.bottom &&
           self.bottom > other.top {
          return true;
        }

        return false;
    }
}

#[cfg(test)]
mod tests {
    use theorem::math::{AABB};

    #[test]
    fn has_width() {
        let b = AABB { top: 2, bottom: 5, left: 2, right: 5 };
        assert_eq!(b.width(), 3);
    }

    #[test]
    fn has_height() {
        let b = AABB { top: 2, bottom: 5, left: 2, right: 5 };
        assert_eq!(b.height(), 3);
    }
}
