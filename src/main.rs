mod bodies;
mod camera;
mod geom;
mod materials;
mod ppm;
mod ray;
mod utils;

use crate::bodies::{HitRecord, Hittable, Sphere};
use crate::camera::Camera;
use crate::geom::{unit_vector, Vec3};
use crate::materials::{Dielectric, Lambert, Metal, Material};
use crate::ppm::{write_color, write_ppm_header};
use crate::ray::Ray;
use crate::utils::{lerp, random_range};
use std::rc::Rc;

mod color {
    use crate::geom::Vec3;

    pub const BLACK: Vec3 = Vec3 { 0: [0.0, 0.0, 0.0] };
    pub const WHITE: Vec3 = Vec3 { 0: [1.0, 1.0, 1.0] };
    pub const LIGHT_BLUE: Vec3 = Vec3 { 0: [0.5, 0.7, 1.0] };
}

/// Типаж для описания вектора как точки в пространстве с координатами `x`, `y` и `z`.
pub trait Point {
    fn x(&self) -> f32;
    fn y(&self) -> f32;
    fn z(&self) -> f32;
}

/// Типаж для описания вектора как цвета с компонентами `r`, `g` и `b`.
pub trait Color {
    fn r(&self) -> f32;
    fn g(&self) -> f32;
    fn b(&self) -> f32;
}

impl Color for Vec3 {
    fn r(&self) -> f32 {
        self.0[0]
    }

    fn g(&self) -> f32 {
        self.0[1]
    }

    fn b(&self) -> f32 {
        self.0[2]
    }
}

impl Point for Vec3 {
    fn x(&self) -> f32 {
        self.0[0]
    }

    fn y(&self) -> f32 {
        self.0[1]
    }

    fn z(&self) -> f32 {
        self.0[2]
    }
}

/// Вычисляет цвет точки на экране.
fn ray_color<T>(ray: &Ray, world: &T, depth: i32) -> Vec3
where
    T: Hittable,
{
    if depth == 0 {
        return color::BLACK;
    }

    let record = world.hit(ray, 0.001, f32::MAX);
    match record {
        Some(hit) => match hit.material.scatter(ray, &hit) {
            Some((scattered, attenuation)) => attenuation * ray_color(&scattered, world, depth - 1),
            None => color::BLACK,
        },
        None => {
            let unit_direction = unit_vector(ray.direction());
            let t = 0.5 * (unit_direction.y() + 1.0);

            lerp(color::WHITE, color::LIGHT_BLUE, t)
        }
    }
}

/// Массив объектов трехмерной сцены.
pub struct World(Vec<Box<dyn Hittable>>);

impl Hittable for World {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut rec: Option<HitRecord> = None;
        let mut closest_so_far = t_max;

        for object in &self.0 {
            if let Some(obj) = object.hit(ray, t_min, closest_so_far) {
                closest_so_far = obj.t;
                rec = Some(obj);
            }
        }
        rec
    }
}

fn random_scene() -> World {
    let mut scene: Vec<Box<dyn Hittable>> = vec![];

    // Шар - земля.
    scene.push(Box::new(Sphere{center: [0.0, -1000.0, 0.0].into(), radius: 1000.0,
        material: Rc::new(Lambert{albedo: [0.5, 0.5, 0.5].into()})
    }));

    // Случайно рассыпанные шарики.
    for a in -11..11 {
        for b in -11..11 {
            let material_rate = random_range(0.0, 1.0);
            let x = random_range(0.0, 1.0);
            let z = random_range(0.0, 1.0);
            let center: Vec3 = [a as f32 + 0.9 * x, 0.2, b as f32 + 0.9 * z].into();

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let x = random_range(0.0, 1.0);
                let y = random_range(0.0, 1.0);
                let z = random_range(0.0, 1.0);
                let fuzz = random_range(0.0, 1.0);

                let material: Rc<dyn Material>;
                if material_rate < 0.8 {
                    material = Rc::new(Lambert{albedo: [x * x, y * y, z * z].into()});
                } else if material_rate < 0.95 {
                    material = Rc::new(Metal::with_albedo_fuzz([0.5 * (1.0 + x), 0.5 * (1.0 + y), 0.5 * (1.0 + z)].into(), fuzz));
                } else {
                    material = Rc::new(Dielectric{ir: 1.5});
                }

                scene.push(Box::new(Sphere{center, radius: 0.2, material}));
            }
        }
    }

    // Три больших шарика в центре.
    scene.push(Box::new(Sphere{center: [0.0, 1.0, 0.0].into(), radius: 1.0,
        material: Rc::new(Dielectric{ir: 1.5})
    }));
    scene.push(Box::new(Sphere{center: [-4.0, 1.0, 0.0].into(), radius: 1.0,
        material: Rc::new(Lambert{albedo: [0.4, 0.2, 0.1].into()})
    }));
    scene.push(Box::new(Sphere{center: [4.0, 1.0, 0.0].into(), radius: 1.0,
        material: Rc::new(Metal::with_albedo_fuzz([0.7, 0.6, 0.5].into(), 0.0))
    }));

    World(scene)
}

fn main() {
    // Image
    let aspect_ratio: f32 = 3.0 / 2.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 500;
    let depth = 50;

    // World
    let world = random_scene();

    // Camera
    let lookfrom: Vec3 = [13.0, 2.0, 3.0].into();
    let lookat: Vec3 = [0.0, 0.0, 0.0].into();
    let vup: Vec3 = [0.0, 1.0, 0.0].into();
    let dist_to_focus = 10.0;
    let aperture = 0.1;

    let camera = Camera::new(
        lookfrom,
        lookat,
        vup,
        20.0,
        aspect_ratio,
        aperture,
        dist_to_focus,
    );

    // Render
    write_ppm_header(image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanlines remaining: {}", j + 1);
        for i in 0..image_width {
            let mut pixel = Vec3::default();
            for _ in 0..samples_per_pixel {
                let x = random_range(0.0, 1.0);
                let y = random_range(0.0, 1.0);
                let u = (i as f32 + x) / (image_width - 1) as f32;
                let v = (j as f32 + y) / (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);

                pixel += ray_color(&ray, &world, depth);
            }
            write_color(pixel, samples_per_pixel);
        }
    }
    eprintln!("Done.");
}
