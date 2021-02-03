use crate::geom::{dot, Vec3};
use crate::ray::Ray;
use crate::materials::Material;
use std::rc::Rc;

/// Параметры попадания луча в объект.
pub struct HitRecord {
    pub t: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub(crate) material: Rc<dyn Material>,
    pub front_face: bool,
}

impl HitRecord {
    fn with_front_face(t: f32, point: Vec3, material: Rc<dyn Material>, outward_normal: Vec3, ray: &Ray) -> Self {
        let front_face = dot(ray.direction(), outward_normal) < 0.0;
        let normal = if front_face {
            outward_normal
        } else {
            -outward_normal
        };

        HitRecord {
            t,
            point,
            normal,
            material,
            front_face,
        }
    }
}

/// Типаж для реализации попадания луча в объект.
pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

/// Описывает положение, радиус и материал сферы.
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Rc<dyn Material>,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin() - self.center;
        let a = ray.direction().length_squared();
        let half_b = dot(oc, ray.direction());
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            // first root
            let t = (-half_b - discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = ray.at(t);
                let outward_normal = (point - self.center) / self.radius;

                return Some(HitRecord::with_front_face(t, point, self.material.clone(), outward_normal, ray));
            }

            // second root
            let t = (-half_b + discriminant.sqrt()) / a;
            if t_min < t && t < t_max {
                let point = ray.at(t);
                let outward_normal = (point - self.center) / self.radius;

                return Some(HitRecord::with_front_face(t, point, self.material.clone(), outward_normal, ray));
            }
        }

        None
    }
}
