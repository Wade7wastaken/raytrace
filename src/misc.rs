use std::mem;

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

pub trait IterExt: Iterator {
    fn collect_arr<const N: usize>(self) -> Option<[Self::Item; N]>;
}

impl<I: Iterator> IterExt for I
where
    Self::Item: Sized,
{
    fn collect_arr<const N: usize>(self) -> Option<[Self::Item; N]> {
        let mut data: [Self::Item; N] = [const { unsafe { mem::zeroed() } }; N];
        let mut counter = 0;

        for x in self {
            if counter >= N {
                return None;
            }
            data[counter] = x;
            counter += 1;
        }

        if counter != N {
            return None;
        }

        Some(data)
    }
}
