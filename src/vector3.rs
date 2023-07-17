use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use std::cmp::PartialEq;
use std::fmt::{Debug, Display};
use std::iter::{FromIterator, Sum};
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

pub trait Scalar:
    'static
    + Clone
    + Copy
    + PartialEq
    + Debug
    + Display
    + Default
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Into<f64>
    + Sum<Self>
{
}

impl Scalar for f64 {}
impl Scalar for i32 {}

#[derive(Clone, Copy, Debug)]
pub struct Vector3<T> {
    pub data: [T; 3],
}

pub type Point3<T> = Vector3<T>;
pub type Point3d = Point3<f64>;
pub type Vector3d = Vector3<f64>;

impl<T> Vector3<T>
where
    T: Scalar,
{
    pub fn new(data: [T; 3]) -> Self {
        Vector3 { data }
    }

    pub fn x(&self) -> T {
        self[0]
    }

    pub fn y(&self) -> T {
        self[1]
    }

    pub fn z(&self) -> T {
        self[2]
    }

    pub fn length_squared(&self) -> f64 {
        (self[0] * self[0] + self[1] * self[1] + self[2] * self[2]).into()
    }

    pub fn length(&self) -> f64 {
        self.length().sqrt()
    }

    pub fn unit_vector(&self) -> Vector3<f64> {
        self.data.iter().map(|&a| a.into() / self.length()).collect()
    }

    pub fn dot(&self, other: &Vector3<T>) -> T {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }

    pub fn cross(&self, other: &Vector3<T>) -> Vector3<T> {
        let x = self[1] * other[2] - self[2] * other[1];
        let y = self[2] * other[0] - self[0] * other[2];
        let z = self[0] * other[1] - self[1] * other[0];

        Vector3::new([x, y, z])
    }
}

impl<T> ToString for Vector3<T>
where T: Scalar
{
    fn to_string(&self) -> String {
        format!("{} {} {}", self.x(), self.y(), self.z())
    }
}

impl<T> Vector3<T>
where
    T: Scalar,
    Standard: Distribution<[T; 3]>,
{
    fn random() -> Self {
        Vector3::new(thread_rng().gen())
    }
}

impl<T> FromIterator<T> for Vector3<T>
where
    T: Scalar,
{
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut data = [Default::default(); 3];
        let mut iterator = iter.into_iter();

        for i in 0..3 {
            data[i] = iterator
                .next()
                .expect("Iterator has insufficient elements.");
        }

        Vector3::new(data)
    }
}

impl<T> Index<usize> for Vector3<T>
where
    T: Scalar,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vector3<T>
where
    T: Scalar,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> PartialEq<Vector3<T>> for Vector3<T>
where
    T: Scalar,
{
    fn eq(&self, other: &Vector3<T>) -> bool {
        self.data
            .iter()
            .zip(other.data.iter())
            .all(|(&a, &b)| a == b)
    }
}

impl<T> Add for Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;

    fn add(self, other: Vector3<T>) -> Self::Output {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b)
            .collect()
    }
}

impl<T> Sub for Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;

    fn sub(self, other: Vector3<T>) -> Self::Output {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a - b)
            .collect()
    }
}

impl<T> Mul<T> for Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;

    fn mul(self, scalar: T) -> Self::Output {
        self.data.iter().map(|&a| a * scalar).collect()
    }
}

impl<T> Div<T> for Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;

    fn div(self, scalar: T) -> Self::Output {
        self.data.iter().map(|&a| a / scalar).collect()
    }
}

impl<T> Mul for Vector3<T>
where
    T: Scalar,
{
    type Output = Vector3<T>;

    fn mul(self, other: Vector3<T>) -> Self::Output {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    type TestScalar = f64;

    #[test]
    fn test_new() {
        let vec: Vector3<TestScalar> = Vector3::new([1.0, 2.0, 3.0]);
        assert_eq!(vec[0], 1.0);
        assert_eq!(vec[1], 2.0);
        assert_eq!(vec[2], 3.0);
    }

    #[test]
    fn test_dot() {
        let vec1: Vector3<TestScalar> = Vector3::new([1.0, 2.0, 3.0]);
        let vec2: Vector3<TestScalar> = Vector3::new([4.0, 5.0, 6.0]);
        let dot_product: TestScalar = vec1.dot(&vec2);
        assert_eq!(dot_product, 32.0);
    }

    #[test]
    fn test_cross() {
        let vec1: Vector3<TestScalar> = Vector3::new([1.0, 2.0, 3.0]);
        let vec2: Vector3<TestScalar> = Vector3::new([4.0, 5.0, 6.0]);
        let cross_product: Vector3<TestScalar> = vec1.cross(&vec2);
        assert_eq!(cross_product, Vector3::new([-3.0, 6.0, -3.0]));
    }

    #[test]
    fn test_random() {
        let vec: Vector3<TestScalar> = Vector3::<TestScalar>::random();
        assert_eq!(vec.data.len(), 3);
    }

    #[test]
    fn test_from_iter() {
        let vec: Vector3<TestScalar> = vec![1.0, 2.0, 3.0].into_iter().collect();
        assert_eq!(vec, Vector3::new([1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_index() {
        let vec: Vector3<TestScalar> = Vector3::new([1.0, 2.0, 3.0]);
        assert_eq!(vec[0], 1.0);
        assert_eq!(vec[1], 2.0);
        assert_eq!(vec[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut vec: Vector3<TestScalar> = Vector3::new([1.0, 2.0, 3.0]);
        vec[0] = 4.0;
        vec[1] = 5.0;
        vec[2] = 6.0;
        assert_eq!(vec, Vector3::new([4.0, 5.0, 6.0]));
    }

    #[test]
    fn test_add_vec() {
        let vec1: Vector3<TestScalar> = Vector3::new([1.0, 2.0, 3.0]);
        let vec2: Vector3<TestScalar> = Vector3::new([4.0, 5.0, 6.0]);
        let sum: Vector3<TestScalar> = vec1 + vec2;
        assert_eq!(sum, Vector3::new([5.0, 7.0, 9.0]));
    }

    #[test]
    fn test_sub_vec() {
        let vec1: Vector3<TestScalar> = Vector3::new([4.0, 5.0, 6.0]);
        let vec2: Vector3<TestScalar> = Vector3::new([1.0, 2.0, 3.0]);
        let diff: Vector3<TestScalar> = vec1 - vec2;
        assert_eq!(diff, Vector3::new([3.0, 3.0, 3.0]));
    }

    #[test]
    fn test_mul_scalar() {
        let vec: Vector3<TestScalar> = Vector3::new([1.0, 2.0, 3.0]);
        let scalar: TestScalar = 2.0;
        let mul: Vector3<TestScalar> = vec * scalar;
        assert_eq!(mul, Vector3::new([2.0, 4.0, 6.0]));
    }

    #[test]
    fn test_div_scalar() {
        let vec: Vector3<TestScalar> = Vector3::new([4.0, 6.0, 8.0]);
        let scalar: TestScalar = 2.0;
        let div: Vector3<TestScalar> = vec / scalar;
        assert_eq!(div, Vector3::new([2.0, 3.0, 4.0]));
    }

    #[test]
    fn test_mul_vec() {
        let vec1: Vector3<TestScalar> = Vector3::new([1.0, 2.0, 3.0]);
        let vec2: Vector3<TestScalar> = Vector3::new([4.0, 5.0, 6.0]);
        let mul: Vector3<TestScalar> = vec1 * vec2;
        assert_eq!(mul, Vector3::new([4.0, 10.0, 18.0]));
    }
}
