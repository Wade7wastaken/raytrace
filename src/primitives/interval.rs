#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    #[must_use]
    #[inline]
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    #[must_use]
    #[inline]
    pub const fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }
}

// helper initializer to make code look pretty
#[must_use]
#[inline]
pub const fn interval(min: f32, max: f32) -> Interval {
    Interval::new(min, max)
}
