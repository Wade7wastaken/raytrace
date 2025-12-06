#[macro_export]
macro_rules! tern {
    ($cond: expr, $a: expr, $b: expr) => {
        if $cond { $a } else { $b }
    };
}

#[must_use]
#[inline]
pub fn rand_f32() -> f32 {
    rand::random_range(0.0..1.0)
}
