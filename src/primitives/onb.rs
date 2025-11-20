use crate::{
    primitives::{Vec3, vec3},
    tern,
};

pub struct Onb {
    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Onb {
    #[must_use]
    pub fn new(n: Vec3) -> Self {
        let w = n.unit_vector();

        // choose another vector that isn't similar to w
        let a = tern!(w.x.abs() > 0.9, vec3(0.0, 1.0, 0.0), vec3(1.0, 0.0, 0.0));
        let v = w.cross(a).unit_vector();
        let u = w.cross(v);
        Self { u, v, w }
    }

    #[must_use]
    pub fn u(&self) -> Vec3 {
        self.u
    }

    #[must_use]
    pub fn v(&self) -> Vec3 {
        self.v
    }

    #[must_use]
    pub fn w(&self) -> Vec3 {
        self.w
    }

    #[must_use]
    pub fn transform(&self, other: Vec3) -> Vec3 {
        other.x * self.u + other.y * self.v + other.z * self.w
    }
}
