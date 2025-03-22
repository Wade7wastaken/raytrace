use std::{fmt, ops::Add};

#[derive(Debug, Clone, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    #[must_use]
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    #[must_use]
    pub fn full() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    #[must_use]
    pub fn from_intervals(a: &Interval, b: &Interval) -> Self {
        let min = f64::min(a.min, b.min);
        let max = f64::max(a.max, b.max);
        Self { min, max }
    }

    #[must_use]
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    #[must_use]
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    #[must_use]
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    #[must_use]
    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }
}

impl Add<f64> for Interval {
    type Output = Interval;
    fn add(self, rhs: f64) -> Self::Output {
        Interval::new(self.min + rhs, self.max + rhs)
    }
}

impl Default for Interval {
    fn default() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }
}

impl fmt::Display for Interval {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.min, self.max)
    }
}

// helper initializer to make code look pretty
#[must_use]
pub fn interval(min: f64, max: f64) -> Interval {
    Interval::new(min, max)
}
