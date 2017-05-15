use theorem::{Rect};

pub trait Image {
    fn width(&self) -> i32;
    fn height(&self) -> i32;

    fn subdivide(&self, rect: &Rect) -> Image;

    fn bounds(&self) -> Rect {
        Rect::new_from_origin(Default::default(), self.width(), self.height())
    }
}
