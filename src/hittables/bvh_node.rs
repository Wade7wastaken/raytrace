use std::{fmt, sync::Arc};

use crate::primitives::{Aabb, Interval, Ray, interval};

use super::{HitRecord, Hittable, HittableList};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    #[must_use]
    fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let bbox = objects.iter().fold(Aabb::default(), |bbox, object| {
            Aabb::from_boxes(&bbox, object.bounding_box())
        });

        let (left, right): (_, _) = match &objects[..] {
            [] | [_] => panic!("Can't create a BVHNode with {} elements", objects.len()),
            [first, last] => (first.clone(), last.clone()),
            [first, _, _] => (first.clone(), Arc::new(BvhNode::new(&mut objects[1..]))),
            _ => {
                objects.sort_by(|a, b| {
                    let a_axis_interval = a.bounding_box().axis_interval(bbox.longest_axis());
                    let b_axis_interval = b.bounding_box().axis_interval(bbox.longest_axis());
                    a_axis_interval.min.total_cmp(&b_axis_interval.min)
                });

                let (left, right) = objects.split_at_mut(objects.len() / 2);
                (Arc::new(Self::new(left)), Arc::new(Self::new(right)))
            }
        };

        Self { left, right, bbox }
    }

    #[must_use]
    pub fn from_hittable_list(mut list: HittableList) -> Self {
        Self::new(&mut list.objects)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let hit_right = self.right.hit(r, ray_t);

        let max = hit_right.as_ref().map_or(ray_t.max, |rec| rec.t);

        let hit_left = self.left.hit(r, &interval(ray_t.min, max));

        // if the left side hit something, it is closer than anything that was
        // hit on the right because we limited the left range to
        // [min..right_hit.t]
        hit_left.or(hit_right)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl fmt::Display for BvhNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let left_str = self
            .left
            .to_string()
            .lines()
            .map(|l| format!("\t{l}"))
            .collect::<Vec<_>>()
            .join("\n");
        let right_str = self
            .right
            .to_string()
            .lines()
            .map(|l| format!("\t{l}"))
            .collect::<Vec<_>>()
            .join("\n");
        write!(f, "bvh\n\t{left_str}\n\t{right_str}")
    }
}
