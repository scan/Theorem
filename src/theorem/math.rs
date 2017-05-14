pub mod theorem {
    pub mod math {
        use std::ops::{Add, Sub};

        #[derive(Copy, Clone, Debug, PartialEq)]
        pub struct Vec2D {
            x: f64,
            y: f64,
        }

        impl Add for Vec2D {
            type Output = Vec2D;

            fn add(self, other: Vec2D) -> Vec2D {
                Vec2D { x: self.x + other.x, y: self.y + other.y }
            }
        }

        impl Sub for Vec2D {
            type Output = Vec2D;

            fn sub(self, other: Vec2D) -> Vec2D {
                Vec2D { x: self.x + other.x, y: self.y + other.y }
            }
        }
    }
}
