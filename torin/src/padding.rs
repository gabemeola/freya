pub use euclid::Rect;

use crate::geometry::Length;

#[derive(PartialEq, Clone, Debug, Default, Copy)]
pub struct Paddings {
    top: Length,
    right: Length,
    bottom: Length,
    left: Length,
}

impl Paddings {
    pub fn new(top: f32, right: f32, bottom: f32, left: f32) -> Self {
        Self {
            top: Length::new(top),
            right: Length::new(right),
            bottom: Length::new(bottom),
            left: Length::new(left),
        }
    }

    pub fn fill_vertical(&mut self, value: f32) {
        self.top = Length::new(value);
        self.bottom = Length::new(value);
    }

    pub fn fill_horizontal(&mut self, value: f32) {
        self.right = Length::new(value);
        self.left = Length::new(value);
    }

    pub fn fill_all(&mut self, value: f32) {
        self.fill_horizontal(value);
        self.fill_vertical(value);
    }

    pub fn horizontal_paddings(&self) -> f32 {
        (self.right + self.left).get()
    }

    pub fn vertical_paddings(&self) -> f32 {
        (self.top + self.bottom).get()
    }

    pub fn top(&self) -> f32 {
        self.top.get()
    }

    pub fn right(&self) -> f32 {
        self.right.get()
    }

    pub fn bottom(&self) -> f32 {
        self.bottom.get()
    }

    pub fn left(&self) -> f32 {
        self.left.get()
    }

    pub fn pretty(&self) -> String {
        format!(
            "({}, {}, {}, {})",
            self.top(),
            self.right(),
            self.bottom(),
            self.left()
        )
    }
}
