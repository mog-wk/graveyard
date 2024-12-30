#![allow(unused)]

mod space;

use std::{
    f64,
    ops::{Add, Div, Mul, MulAssign, Neg, Rem, Sub},
    process::Output,
};

// alias for matrices elements
type M<T> = [[T; 2]; 2];

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat2<T: Copy> {
    elements: M<T>,
}

impl<T: Copy> Mat2<T> {
    pub fn set(&mut self, i: usize, j: usize, elt: T) {
        self.elements[i][j] = elt;
    }
    pub fn get(&self, i: usize, j: usize) -> T {
        self.elements[i][j]
    }
}

impl<T> Mat2<T>
where
    T: Copy + From<i32>,
{
    pub fn new(elements: M<T>) -> Self {
        Self { elements }
    }
    pub fn zero() -> Self {
        Self {
            elements: [[0.into(); 2]; 2],
        }
    }
    pub fn ones() -> Self {
        Self {
            elements: [[1.into(); 2]; 2],
        }
    }
}

// mat2 to mat2 operations
impl<T: Copy + Add<Output = T> + From<i32>> Add<Mat2<T>> for Mat2<T> {
    type Output = Self;
    fn add(self, rhs: Mat2<T>) -> Self::Output {
        let mut elements: M<T> = [[0.into(), 0.into()], [0.into(), 0.into()]];
        for i in 0..=1 {
            for j in 0..=1 {
                elements[i][j] = self.get(i, j) + rhs.get(i, j);
            }
        }
        Self::Output { elements }
    }
}

// from
impl From<i32> for Mat2<i32> {
    fn from(value: i32) -> Self {
        let elements: M<i32> = [[value, value], [value, value]];
        Self { elements }
    }
}

impl From<[i32; 4]> for Mat2<i32> {
    fn from(value: [i32; 4]) -> Self {
        let elements: M<i32> = [[value[0], value[1]], [value[2], value[3]]];
        Self { elements }
    }
}

#[cfg(test)]
#[path = "./tests/mat2.rs"]
mod mat2;
