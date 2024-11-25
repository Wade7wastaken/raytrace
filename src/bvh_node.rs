use std::{cmp::Ordering, fmt, sync::Arc};

use crate::{
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    primitives::{interval, Aabb, Interval, Ray},
};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let mut bbox = Aabb::default();
        for object in &objects[start..end] {
            bbox = Aabb::from_boxes(&bbox, object.bounding_box());
        }

        let comparator = match bbox.longest_axis() {
            0 => compare(0),
            1 => compare(1),
            2 => compare(2),
            _ => unreachable!(),
        };

        let object_span = end - start;

        let (left, right): (Arc<dyn Hittable>, Arc<dyn Hittable>) = match object_span {
            1 => (objects[start].to_owned(), objects[start].to_owned()),
            2 => (objects[start].to_owned(), objects[start + 1].to_owned()),
            _ => {
                objects[start..end].sort_by(comparator);

                let mid = start + object_span / 2;
                (
                    Arc::new(Self::new(objects, start, mid)),
                    Arc::new(Self::new(objects, mid, end)),
                )
            }
        };

        Self { left, right, bbox }
    }

    pub fn from_hittable_list(mut list: HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
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

        let hit_left = self.left.hit(r, ray_t);
        let max = if let Some(ref rec) = hit_left {
            rec.t
        } else {
            ray_t.max
        };
        let right_hit = self.right.hit(r, &interval(ray_t.min, max));

        // if the right side hit something, it is closer than anything that was hit on the left
        right_hit.or(hit_left)
    }

    fn bounding_box(&self) -> &Aabb {
        &self.bbox
    }
}

impl fmt::Display for BvhNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "bvh(left: {},\r\nright:\r\n{}, bbox:\r\n{})",
            self.left, self.right, self.bbox
        )
    }
}
