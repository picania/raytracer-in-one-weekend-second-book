use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{Neg, Index, IndexMut, Add, AddAssign, Sub, SubAssign, Mul, MulAssign, Div, DivAssign};

/// Трехкомпонентный вектор с плавающей точкой.
#[derive(Copy, Clone, Debug)]
pub struct Vec3(pub [f32; 3]);

impl Vec3 {
    /// Создает новый вектор.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { 0: [x, y, z] }
    }

    /// Возвращает длину вектора.
    pub fn length(&self) -> f32 {
        self.length_squared().sqrt()
    }

    /// Возвращает квадрат длины вектора.
    pub fn length_squared(&self) -> f32 {
        self.0[0] * self.0[0] + self.0[1] * self.0[1] + self.0[2] * self.0[2]
    }

    /// Приводит вектор к единичному виду.
    pub fn make_unit_vector(&mut self) {
        let k = 1.0 / self.length();
        *self *= k;
    }
}

/// Конструктор по умолчанию.
impl Default for Vec3 {
    fn default() -> Self {
        Vec3 { 0: [0.0, 0.0, 0.0] }
    }
}

/// Форматированный вывод.
impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}", self.0[0], self.0[1], self.0[2])
    }
}

impl PartialEq for Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.0[0] == other.0[0] && self.0[1] == other.0[1] && self.0[2] == other.0[2]
    }
}

/// Унарный минус.
///
/// Применяет унарный минус к каждой компоненте вектора.
impl Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            0: [-self.0[0], -self.0[1], -self.0[2]],
        }
    }
}

/// Оператор чтения по индексу.
impl Index<usize> for Vec3 {
    type Output = f32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

/// Оператор записи по индексу.
impl IndexMut<usize> for Vec3 {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

/// Оператор сложения векторов.
///
/// Векторы складываются по компонентам.
impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self {
            0: [
                self.0[0] + rhs.0[0],
                self.0[1] + rhs.0[1],
                self.0[2] + rhs.0[2],
            ],
        }
    }
}

/// Оператор сложения вектора со скаляром.
///
/// К каждой компоненте вектора прибавляется число.
impl Add<f32> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: f32) -> Self::Output {
        Self {
            0: [
                self.0[0] + rhs,
                self.0[1] + rhs,
                self.0[2] + rhs,
            ],
        }
    }
}

/// Оператор сложения векторов с присваиванием.
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.0[0] += rhs.0[0];
        self.0[1] += rhs.0[1];
        self.0[2] += rhs.0[2];
    }
}

/// Оператор вычитания векторов.
///
/// Векторы вычитаются по компонентам.
impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self {
            0: [
                self.0[0] - rhs.0[0],
                self.0[1] - rhs.0[1],
                self.0[2] - rhs.0[2],
            ],
        }
    }
}

/// Оператор вычитания вектора с присваиванием.
impl SubAssign for Vec3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.0[0] -= rhs.0[0];
        self.0[1] -= rhs.0[1];
        self.0[2] -= rhs.0[2];
    }
}

/// Оператор умножения векторов.
///
/// Векторы умножаются по компонентам.
impl Mul for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Self) -> Self::Output {
        Self {
            0: [
                self.0[0] * rhs.0[0],
                self.0[1] * rhs.0[1],
                self.0[2] * rhs.0[2],
            ],
        }
    }
}

/// Оператор умножения вектора на скаляр.
///
/// Каждая компонента вектора умножается на число.
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self *= rhs;
        self
    }
}

/// Оператор умножения скаляра на вектор.
///
/// Каждая компонента вектора умножается на число.
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}

/// Оператор умножения на вектор с присваиванием.
impl MulAssign for Vec3 {
    fn mul_assign(&mut self, rhs: Self) {
        self.0[0] *= rhs.0[0];
        self.0[1] *= rhs.0[1];
        self.0[2] *= rhs.0[2];
    }
}

/// Оператор деления векторов.
///
/// Компоненты первого вектора делятся на соответствующие компоненты второго вектора.
impl Div for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: Self) -> Self::Output {
        Self {
            0: [
                self.0[0] / rhs.0[0],
                self.0[1] / rhs.0[1],
                self.0[2] / rhs.0[2],
            ],
        }
    }
}

/// Оператор деления вектора на скаляр.
///
/// Каждая компонента вектора делится на число.
impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(mut self, rhs: f32) -> Self::Output {
        self /= rhs;
        self
    }
}

/// Оператор деления на вектор с присваиванием.
impl DivAssign for Vec3 {
    fn div_assign(&mut self, rhs: Self) {
        self.0[0] /= rhs.0[0];
        self.0[1] /= rhs.0[1];
        self.0[2] /= rhs.0[2];
    }
}

/// Оператор умножения на скаляр с присваиванием.
///
/// Каждая компонента вектора умножается на число.
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.0[0] *= rhs;
        self.0[1] *= rhs;
        self.0[2] *= rhs;
    }
}

/// Оператор деления на скаляр с присваиванием.
///
/// Каждая компонента вектора делится на число.
#[allow(clippy::suspicious_op_assign_impl)]
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
        let k = 1.0 / rhs;

        self.0[0] *= k;
        self.0[1] *= k;
        self.0[2] *= k;
    }
}

/// Скалярное произведение двух векторов. [Подробнее].
///
/// [Подробнее]: https://ru.wikipedia.org/wiki/%D0%A1%D0%BA%D0%B0%D0%BB%D1%8F%D1%80%D0%BD%D0%BE%D0%B5_%D0%BF%D1%80%D0%BE%D0%B8%D0%B7%D0%B2%D0%B5%D0%B4%D0%B5%D0%BD%D0%B8%D0%B5
pub fn dot(a: Vec3, b: Vec3) -> f32 {
    a.0[0] * b.0[0] + a.0[1] * b.0[1] + a.0[2] * b.0[2]
}

/// Векторное произведение двух векторов. [Подробнее].
///
/// [Подробнее]: https://ru.wikipedia.org/wiki/%D0%92%D0%B5%D0%BA%D1%82%D0%BE%D1%80%D0%BD%D0%BE%D0%B5_%D0%BF%D1%80%D0%BE%D0%B8%D0%B7%D0%B2%D0%B5%D0%B4%D0%B5%D0%BD%D0%B8%D0%B5
#[allow(dead_code)]
pub fn cross(a: Vec3, b: Vec3) -> Vec3 {
    Vec3 {
        0: [
            a.0[1] * b.0[2] - a.0[2] * b.0[1],
            a.0[2] * b.0[0] - a.0[0] * b.0[2],
            a.0[0] * b.0[1] - a.0[1] * b.0[0],
        ],
    }
}

impl From<[f32; 3]> for Vec3 {
    fn from(a: [f32; 3]) -> Self {
        Self {0: a}
    }
}

/// Создает единичный вектор.
pub fn unit_vector(a: Vec3) -> Vec3 {
    a / a.length()
}
