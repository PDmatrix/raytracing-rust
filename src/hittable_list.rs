use crate::hittable::{HitRecord, Hittable};
use crate::ray::Ray;

pub struct HittableList<T: Hittable> {
    list: Vec<T>,
}

impl<T: Hittable> HittableList<T> {
    pub(crate) fn new(list: Vec<T>) -> HittableList<T> {
        HittableList { list }
    }
}

impl<T> Hittable for HittableList<T>
where
    T: Hittable,
{
    fn hit(&self, r: &Ray, t_min: f32, t_max: f32, _rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for i in 0..self.list.len() {
            if self.list[i].hit(&r, t_min, closest_so_far, _rec) {
                hit_anything = true;
                closest_so_far = _rec.t;
            }
        }

        hit_anything
    }
}
