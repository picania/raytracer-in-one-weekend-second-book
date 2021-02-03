use crate::geom::Vec3;

/// Луч, направленный из точки `from` в направлении `dir`.
pub struct Ray {
    pub orig: Vec3,
    pub dir: Vec3,
}

impl Ray {
    /// Создает луч с началом в точке `from` и направленный в точку `to`.
    pub fn new(orig: Vec3, dir: Vec3) -> Self {
        Self { orig, dir }
    }

    /// Координаты точки, из которой исходит луч.
    pub fn origin(&self) -> Vec3 {
        self.orig
    }

    /// Координаты точки, куда направлен луч.
    pub fn direction(&self) -> Vec3 {
        self.dir
    }

    /// Координаты точки, лежащей на луче в отрезке `[orig; orig + dir]`.
    ///
    /// Параметр `t` принимает значения в диапазоне `[0; 1]`.
    /// При `t = 0` точка совпадает с началом отрезка, при `t = 1` точка совпадает с концом отрезка.
    pub fn at(&self, t: f32) -> Vec3 {
        self.orig + t * self.dir
    }
}
