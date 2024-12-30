use std::{
    f64,
    ops::{Add, Div, Mul, MulAssign, Neg, Rem, Sub},
    process::Output,
};

trait Float: Neg<Output = Self> + PartialOrd + Copy {}

// describes a number that can be used in a vector
//pub trait VectorUnit {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Vec2<T: Sized + Copy> {
    x: T,
    y: T,
}

impl<T> Vec2<T>
where
    T: Copy,
{
    pub fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}
impl<T> Vec2<T>
where
    T: Copy + From<i32>,
{
    pub fn zero() -> Self {
        Self {
            x: 0.into(),
            y: 0.into(),
        }
    }
    pub fn one() -> Self {
        Self {
            x: 1.into(),
            y: 1.into(),
        }
    }
    pub fn identity() -> Self {
        Self {
            x: 1.into(),
            y: 1.into(),
        }
    }
}

impl<T> Vec2<T>
where
    T: Copy + Mul<Output = T> + Add<Output = T>,
{
    pub fn dot(&self, rhs: Self) -> T {
        self.x * rhs.x + self.y * rhs.y
    }
}

impl Vec2<f64> {
    pub fn mag(&self) -> f64 {
        (self.x.powf(2.) + self.y.powf(2.)).sqrt()
    }
    // rad
    pub fn angle(&self, rhs: Self) -> f64 {
        self.dot(rhs) / (self.mag() * rhs.mag())
    }
}

impl<T> Vec2<T>
where
    T: Copy + Mul<Output = T>,
{
    pub fn scale(&mut self, r: T) {
        self.x = self.x * r;
        self.y = self.x * r;
    }
    pub fn scale_2d(&mut self, rx: T, ry: T) {
        self.x = self.x * rx;
        self.y = self.x * ry;
    }
}
impl<T> Vec2<T>
where
    T: Copy + Neg<Output = T>,
{
    pub fn neg(&self) -> Self {
        Self {
            x: self.x.neg(),
            y: self.y.neg(),
        }
    }
}

// vec2 to vec2 operations
impl<T: Add<Output = T> + Copy> Add<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: Vec2<T>) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl<T: Sub<Output = T> + Copy> Sub<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: Vec2<T>) -> Self::Output {
        Self::Output {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl<T: Mul<Output = T> + Copy> Mul<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: Vec2<T>) -> Self::Output {
        Self::Output {
            x: self.x * rhs.x,
            y: self.y * rhs.y,
        }
    }
}

impl<T: Div<Output = T> + Copy> Div<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn div(self, rhs: Vec2<T>) -> Self::Output {
        Self::Output {
            x: self.x / rhs.x,
            y: self.y / rhs.y,
        }
    }
}

impl<T: Rem<Output = T> + Copy> Rem<Vec2<T>> for Vec2<T> {
    type Output = Vec2<T>;
    fn rem(self, rhs: Vec2<T>) -> Self::Output {
        Self::Output {
            x: self.x % rhs.x,
            y: self.y % rhs.y,
        }
    }
}
// vec2 scalar operations
impl<T: Copy + Add<Output = T>> Add<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn add(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x + rhs,
            y: self.y + rhs,
        }
    }
}
impl<T: Copy + Sub<Output = T>> Sub<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn sub(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x - rhs,
            y: self.y - rhs,
        }
    }
}
impl<T: Copy + Mul<Output = T>> Mul<T> for Vec2<T> {
    type Output = Vec2<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Self::Output {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

/* eq
impl<T> PartialEq for Vec2<T> {
    fn eq(&self, other: &Self) -> bool {
        (self.x == other.x && self.y == other.y)
    }
}
*/
// from
impl<T: Copy> From<&[T; 2]> for Vec2<T> {
    fn from(value: &[T; 2]) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}
impl<T: Copy> From<&Vec<T>> for Vec2<T> {
    fn from(value: &Vec<T>) -> Self {
        Self {
            x: value[0],
            y: value[1],
        }
    }
}

#[cfg(test)]
#[path = "../tests/vec2.rs"]
mod tests;
