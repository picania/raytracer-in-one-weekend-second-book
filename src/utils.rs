use crate::geom::{dot, unit_vector, Vec3};

use rand::distributions::Uniform;
use rand::Rng;

pub fn clamp(value: f32) -> u8 {
    value.max(u8::MIN as f32).min(u8::MAX as f32) as u8
}

pub fn lerp(from: Vec3, to: Vec3, t: f32) -> Vec3 {
    (1.0 - t) * from + t * to
}

pub fn random_range(low: f32, high: f32) -> f32 {
    rand::thread_rng().sample::<f32, _>(Uniform::new(low, high))
}

/// Создает случайный вектор внутри единичной сферы методом исключения.
pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let x = random_range(-1.0, 1.0);
        let y = random_range(-1.0, 1.0);
        let z = random_range(-1.0, 1.0);
        let p = Vec3::new(x, y, z);
        if p.length_squared() <= 1.0 {
            return p;
        }
    }
}

pub fn random_unit_vector() -> Vec3 {
    unit_vector(random_in_unit_sphere())
}

/// Создает случайный вектор внутри единичной полусферы в направлении нормали.
pub fn random_in_hemisphere(normal: Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();
    if dot(in_unit_sphere, normal) > 0.0 {
        // In the same hemisphere as the normal
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let x = random_range(-1.0, 1.0);
        let y = random_range(-1.0, 1.0);
        let p: Vec3 = [x, y, 0.0].into();
        if p.length_squared() < 1.0 {
            return p;
        }
    }
}

pub fn deg_to_rad(deg: f32) -> f32 {
    deg * std::f32::consts::PI / 180.0
}

#[cfg(test)]
mod test {
    use super::clamp;

    #[test]
    fn check_float_inside_u8_range() {
        assert_eq!(42, clamp(42.9999_f32));
    }

    #[test]
    fn check_float_on_border_u8_range() {
        assert_eq!(0, clamp(0_f32));
        assert_eq!(255, clamp(255_f32));
    }

    #[test]
    fn check_float_outside_u8_range() {
        assert_eq!(0, clamp(-42.9999_f32));
        assert_eq!(255, clamp(256_f32));
    }

    #[test]
    fn check_float_nan() {
        assert_eq!(0, clamp(f32::NAN));
    }
}
