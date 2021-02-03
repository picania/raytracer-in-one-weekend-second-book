use crate::bodies::HitRecord;
use crate::geom::{dot, unit_vector, Vec3};
use crate::ray::Ray;
use crate::utils::{random_in_unit_sphere, random_unit_vector, random_range};
use crate::color;

/// Типаж реализует взаимодействие поверхности тела со светом.
pub trait Material {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)>;
}

/// Описывает рассеивающее тело.
pub struct Lambert {
    pub albedo: Vec3,
}

/// Реализует рассеяние света по закону Ламберта.
impl Material for Lambert {
    fn scatter(&self, _: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let dir = record.normal + random_unit_vector();
        let scattered = Ray {
            orig: record.point,
            dir,
        };

        Some((scattered, self.albedo))
    }
}

/// Описывает отражающее тело.
pub struct Metal {
    albedo: Vec3,
    fuzz: f32,
}

impl Metal {
    /// Создает металлический материал с идеальной отражающей поверхностью.
    pub fn with_albedo(albedo: Vec3) -> Self {
        Metal { albedo, fuzz: 0.0 }
    }

    /// Создает металлический материал с матовой отражающей поверхностью.
    ///
    /// Степень матовости определяется вторым параметром в диапазоне `[0; 1]`.
    pub fn with_albedo_fuzz(albedo: Vec3, fuzz: f32) -> Self {
        let f = if fuzz < 0.0 {
            0.0
        } else if fuzz > 1.0 {
            1.0
        } else {
            fuzz
        };

        Metal { albedo, fuzz: f }
    }
}

impl Material for Metal {
    fn scatter(&self, ray: &Ray, record: &HitRecord) -> Option<(Ray, Vec3)> {
        let reflected = reflect(unit_vector(ray.direction()), record.normal);
        let scattered = Ray {
            orig: record.point,
            dir: reflected + self.fuzz * random_in_unit_sphere(),
        };

        if dot(scattered.direction(), record.normal) > 0.0 {
            Some((scattered, self.albedo))
        } else {
            None
        }
    }
}

/// Описывает преломляющее свет тело.
pub struct Dielectric {
    pub ir: f32,
}

impl Material for Dielectric {
    fn scatter(&self, ray: &Ray, hit: &HitRecord) -> Option<(Ray, Vec3)> {
        let attenuation = color::WHITE;
        let refraction_ratio = if hit.front_face { 1.0 / self.ir } else { self.ir };
        let unit_direction = unit_vector(ray.direction());

        let cos_theta = dot(-unit_direction, hit.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();
        let cannot_refract = refraction_ratio * sin_theta > 1.0;

        let direction;
        if cannot_refract || schlick(cos_theta, self.ir) > random_range(0.0, 1.0) {
            direction = reflect(unit_direction, hit.normal);
        } else {
            direction = refract(unit_direction, hit.normal, refraction_ratio);
        }

        Some((Ray{orig: hit.point, dir: direction}, attenuation))
    }
}

/// Описывает закон отражения луча от поверхности.
fn reflect(vec: Vec3, normal: Vec3) -> Vec3 {
    vec - 2.0 * dot(vec, normal) * normal
}

/// Описывает закон преломления луча на поверхности тела.
// fn refract(vec: Vec3, normal: Vec3, ni_over_nt: f32) -> Option<Vec3> {
//     let uv = unit_vector(vec);
//     let dt = dot(uv, normal);
//     let discriminant = 1.0 - ni_over_nt * ni_over_nt * (1.0 - dt * dt);
//
//     if discriminant > 0.0 {
//         Some(ni_over_nt * (uv - normal * dt) - normal * discriminant.sqrt())
//     } else {
//         None
//     }
// }

fn refract(uv: Vec3, n: Vec3, etai_over_etat: f32) -> Vec3 {
    let cos_theta = dot(-uv, n).min(1.0);
    let r_out_perp =  etai_over_etat * (uv + cos_theta * n);
    let r_out_parallel = -(1.0 - r_out_perp.length_squared()).abs().sqrt() * n;

    r_out_perp + r_out_parallel
}

/// Приближение Шлика для коэффициента внутреннего отражения.
pub fn schlick(cosine: f32, ref_index: f32) -> f32 {
    let r0 = (1.0 - ref_index) / (1.0 + ref_index);
    let r0 = r0 * r0;

    r0 + (1.0 - r0) * f32::powi(1.0 - cosine, 5)
}
