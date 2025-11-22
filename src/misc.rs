#[macro_export]
macro_rules! tern {
    ($cond: expr, $a: expr, $b: expr) => {
        if $cond { $a } else { $b }
    };
}

#[must_use]
pub fn rand_f64() -> f64 {
    rand::random_range(0.0..1.0)
}
