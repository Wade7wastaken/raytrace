use std::{f64::consts::PI, mem};

use crate::primitives::{Vec3, vec3};

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

#[must_use]
pub fn random_cosine_direction() -> Vec3 {
    let r1 = rand_f64();
    let r2 = rand_f64();

    let phi = 2.0 * PI * r1;
    let x = phi.cos() * r2.sqrt();
    let y = phi.sin() * r2.sqrt();
    let z = (1.0 - r2).sqrt();

    vec3(x, y, z)
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
