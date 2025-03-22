use std::{cmp::Ordering, fmt, sync::Arc};

use crate::primitives::{Aabb, Interval, Ray, interval};

use super::{HitRecord, Hittable, HittableList};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Option<Arc<dyn Hittable>>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: &mut [Arc<dyn Hittable>]) -> Self {
        let bbox = objects.iter().fold(Aabb::default(), |bbox, object| {
            Aabb::from_boxes(&bbox, object.bounding_box())
        });

        let (left, right): (Arc<dyn Hittable>, Option<Arc<dyn Hittable>>) = match &objects[..] {
            [] => panic!("Can't create a BVHNode with 0 elements"),
            [first] => (first.clone(), None),
            [first, last] => (first.clone(), Some(last.clone())),
            _ => {
                let comparator = match bbox.longest_axis() {
                    0 => compare(0),
                    1 => compare(1),
                    2 => compare(2),
                    _ => unreachable!(),
                };
                objects.sort_by(comparator);

                let mid = objects.len() / 2;
                (
                    Arc::new(Self::new(&mut objects[..mid])),
                    Some(Arc::new(Self::new(&mut objects[mid..]))),
                )
            }
        };

        Self { left, right, bbox }
    }

    #[must_use]
    pub fn from_hittable_list(mut list: HittableList) -> Self {
        Self::new(&mut list.objects)
    }
}

fn compare(axis_index: u8) -> impl Fn(&Arc<dyn Hittable>, &Arc<dyn Hittable>) -> Ordering {
    move |a, b| {
        let a_axis_interval = a.bounding_box().axis_interval(axis_index);
        let b_axis_interval = b.bounding_box().axis_interval(axis_index);
        a_axis_interval.min.total_cmp(&b_axis_interval.min)
    }
}

impl Hittable for BvhNode {
    fn hit(&self, r: &Ray, ray_t: &Interval) -> Option<HitRecord> {
        if !self.bbox.hit(r, ray_t) {
            return None;
        }

        let hit_right = self.right.as_ref().and_then(|right| right.hit(r, ray_t));

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
        if let Some(right) = &self.right {
            write!(f, "bvh({}, {})", self.left, right)
        } else {
            write!(f, "bvh({})", self.left)
        }
    }
}
