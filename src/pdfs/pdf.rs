use crate::primitives::Vec3;

pub trait Pdf {
    /// Returns the pdf value for a given direction, or the probability this
    /// given direction was produced by this pdf.
    fn value(&self, dir: Vec3) -> f64;

    /// Generates a random direction.
    fn generate(&self) -> Vec3;
}
