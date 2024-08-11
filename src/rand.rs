use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    Rng,
};

pub fn rand_range<T, R>(range: R) -> T
where
    T: SampleUniform,
    R: SampleRange<T>,
{
    rand::thread_rng().gen_range(range)
}

pub fn rand() -> f64 {
    rand_range(0.0..1.0)
}
