use rand::distr::uniform::{SampleRange, SampleUniform};

pub fn rand_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    rand::random_range(range)
}

#[must_use]
pub fn rand() -> f64 {
    rand_range(0.0..1.0)
}
