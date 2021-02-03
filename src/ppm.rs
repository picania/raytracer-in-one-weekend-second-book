use crate::utils::clamp;
use crate::Color;
use crate::geom::Vec3;

/// Преобразует цветовые компоненты пикселя к [`u8`] и печатает на экран.
///
/// [`u8`]: https://doc.rust-lang.org/std/primitive.u8.html
pub fn write_color(pixel: Vec3, samples: i32)
{
    let mut pixel = pixel / samples as f32;

    // Корректировка гаммы γ = 1/2
    pixel.0.iter_mut().for_each(|x| { *x = x.sqrt(); });

    let ir = clamp(u8::MAX as f32 * pixel.r());
    let ig = clamp(u8::MAX as f32 * pixel.g());
    let ib = clamp(u8::MAX as f32 * pixel.b());

    // В операционной системе Windows макрос println! выводит концы строк в Unix стиле
    if cfg!(target_family = "windows") {
        print!("{} {} {}\r\n", ir, ig, ib);
    }
    if cfg!(target_family = "unix") {
        println!("{} {} {}", ir, ig, ib);
    }
}

// В операционной системе Windows макрос println! выводит концы строк в Unix стиле
#[cfg(target_family = "windows")]
pub fn write_ppm_header(image_width: i32, image_height: i32) {
    print!("P3\r\n");
    print!("{} {}\r\n", image_width, image_height);
    print!("255\r\n");
}

#[cfg(target_family = "unix")]
pub fn write_ppm_header(image_width: i32, image_height: i32) {
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");
}
