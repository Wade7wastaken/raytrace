use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::{interval, Interval},
    rand::rand_range,
    ray::Ray,
};

pub struct BvhNode {
    left: Arc<dyn Hittable>,
    right: Arc<dyn Hittable>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(objects: &mut Vec<Arc<dyn Hittable>>, start: usize, end: usize) -> Self {
        let comparator = match rand_range(0..2) {
            0 => box_x_compare,
            1 => box_y_compare,
            2 => box_z_compare,
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

        let bbox = Aabb::from_boxes(left.bounding_box(), right.bounding_box());

        Self { left, right, bbox }
    }

    pub fn from_hittable_list(mut list: HittableList) -> Self {
        let len = list.objects.len();
        Self::new(&mut list.objects, 0, len)
    }
}

fn compare(
    a: &Arc<dyn Hittable + 'static>,
    b: &Arc<dyn Hittable + 'static>,
    axis_index: u8,
) -> Ordering {
    let a_axis_interval = a.bounding_box().axis_interval(axis_index);
    let b_axis_interval = b.bounding_box().axis_interval(axis_index);
    a_axis_interval.min.total_cmp(&b_axis_interval.min)
}

fn box_x_compare(a: &Arc<dyn Hittable + 'static>, b: &Arc<dyn Hittable + 'static>) -> Ordering {
    compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Hittable + 'static>, b: &Arc<dyn Hittable + 'static>) -> Ordering {
    compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Hittable + 'static>, b: &Arc<dyn Hittable + 'static>) -> Ordering {
    compare(a, b, 2)
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
