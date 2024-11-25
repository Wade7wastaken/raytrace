use crate::primitives::{Color, Point3};

pub trait Texture: Sync + Send {
    fn value(&self, u: f64, v: f64, p: Point3) -> Color;
}
