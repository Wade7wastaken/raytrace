use rand::Rng;
use std::ops::Range;

pub fn rand_range(range: Range<f64>) -> f64 {
    rand::thread_rng().gen_range(range)
}

pub fn rand() -> f64 {
    rand_range(0.0..1.0)
}
