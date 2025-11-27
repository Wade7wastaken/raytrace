#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    #[must_use]
    pub const fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub const fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
}

// helper initializer to make code look pretty
#[must_use]
pub const fn interval(min: f64, max: f64) -> Interval {
    Interval::new(min, max)
}
