use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use std::cmp::PartialEq;
use std::fmt::Debug;
use std::iter::{FromIterator, Sum};
use std::ops::{Add, Div, Index, IndexMut, Mul, Neg, Sub};

pub trait Scalar:
    'static
    + Clone
    + Copy
    + PartialEq
    + Debug
    + Default
    + Neg<Output = Self>
    + Add<Output = Self>
    + Sub<Output = Self>
    + Mul<Output = Self>
    + Div<Output = Self>
    + Sum<Self>
{
}

#[derive(Clone, Copy, Debug)]
struct Vec3<T> {
    data: [T; 3],
}

impl<T> Vec3<T>
where
    T: Scalar,
{
    fn new(data: [T; 3]) -> Self {
        Vec3 { data }
    }

    fn dot(&self, other: &Vec3<T>) -> T {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a * b)
            .sum()
    }

    fn cross(&self, other: &Vec3<T>) -> Vec3<T> {
        let x = self.data[1] * other.data[2] - self.data[2] * other.data[1];
        let y = self[2] * other[0] - self[0] * other[2];
        let z = self[0] * other[1] - self[1] * other[0];

        Vec3::new([x, y, z])
    }
}

impl<T> Vec3<T>
where
    T: Scalar,
    Standard: Distribution<[T; 3]>,
{
    fn random() -> Self {
        Vec3::new(thread_rng().gen())
    }
}

impl<T> FromIterator<T> for Vec3<T>
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

        Vec3::new(data)
    }
}

impl<T> Index<usize> for Vec3<T>
where
    T: Scalar,
{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}

impl<T> IndexMut<usize> for Vec3<T>
where
    T: Scalar,
{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.data[index]
    }
}

impl<T> PartialEq<Vec3<T>> for Vec3<T>
where
    T: Scalar,
{
    fn eq(&self, other: &Vec3<T>) -> bool {
        self.data
            .iter()
            .zip(other.data.iter())
            .all(|(&a, &b)| a == b)
    }
}

impl<T> Add for Vec3<T>
where
    T: Scalar,
{
    type Output = Vec3<T>;

    fn add(self, other: Vec3<T>) -> Self::Output {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a + b)
            .collect()
    }
}

impl<T> Sub for Vec3<T>
where
    T: Scalar,
{
    type Output = Vec3<T>;

    fn sub(self, other: Vec3<T>) -> Self::Output {
        self.data
            .iter()
            .zip(other.data.iter())
            .map(|(&a, &b)| a - b)
            .collect()
    }
}

impl<T> Mul<T> for Vec3<T>
where
    T: Scalar,
{
    type Output = Vec3<T>;

    fn mul(self, scalar: T) -> Self::Output {
        self.data.iter().map(|&a| a * scalar).collect()
    }
}

impl<T> Div<T> for Vec3<T>
where
    T: Scalar,
{
    type Output = Vec3<T>;

    fn div(self, scalar: T) -> Self::Output {
        self.data.iter().map(|&a| a / scalar).collect()
    }
}

impl<T> Mul for Vec3<T>
where
    T: Scalar,
{
    type Output = Vec3<T>;

    fn mul(self, other: Vec3<T>) -> Self::Output {
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
    impl Scalar for TestScalar {}

    #[test]
    fn test_new() {
        let vec: Vec3<TestScalar> = Vec3::new([1.0, 2.0, 3.0]);
        assert_eq!(vec[0], 1.0);
        assert_eq!(vec[1], 2.0);
        assert_eq!(vec[2], 3.0);
    }

    #[test]
    fn test_dot() {
        let vec1: Vec3<TestScalar> = Vec3::new([1.0, 2.0, 3.0]);
        let vec2: Vec3<TestScalar> = Vec3::new([4.0, 5.0, 6.0]);
        let dot_product: TestScalar = vec1.dot(&vec2);
        assert_eq!(dot_product, 32.0);
    }

    #[test]
    fn test_cross() {
        let vec1: Vec3<TestScalar> = Vec3::new([1.0, 2.0, 3.0]);
        let vec2: Vec3<TestScalar> = Vec3::new([4.0, 5.0, 6.0]);
        let cross_product: Vec3<TestScalar> = vec1.cross(&vec2);
        assert_eq!(cross_product, Vec3::new([-3.0, 6.0, -3.0]));
    }

    #[test]
    fn test_random() {
        let vec: Vec3<TestScalar> = Vec3::<TestScalar>::random();
        assert_eq!(vec.data.len(), 3);
    }

    #[test]
    fn test_from_iter() {
        let vec: Vec3<TestScalar> = vec![1.0, 2.0, 3.0].into_iter().collect();
        assert_eq!(vec, Vec3::new([1.0, 2.0, 3.0]));
    }

    #[test]
    fn test_index() {
        let vec: Vec3<TestScalar> = Vec3::new([1.0, 2.0, 3.0]);
        assert_eq!(vec[0], 1.0);
        assert_eq!(vec[1], 2.0);
        assert_eq!(vec[2], 3.0);
    }

    #[test]
    fn test_index_mut() {
        let mut vec: Vec3<TestScalar> = Vec3::new([1.0, 2.0, 3.0]);
        vec[0] = 4.0;
        vec[1] = 5.0;
        vec[2] = 6.0;
        assert_eq!(vec, Vec3::new([4.0, 5.0, 6.0]));
    }

    #[test]
    fn test_add_vec() {
        let vec1: Vec3<TestScalar> = Vec3::new([1.0, 2.0, 3.0]);
        let vec2: Vec3<TestScalar> = Vec3::new([4.0, 5.0, 6.0]);
        let sum: Vec3<TestScalar> = vec1 + vec2;
        assert_eq!(sum, Vec3::new([5.0, 7.0, 9.0]));
    }

    #[test]
    fn test_sub_vec() {
        let vec1: Vec3<TestScalar> = Vec3::new([4.0, 5.0, 6.0]);
        let vec2: Vec3<TestScalar> = Vec3::new([1.0, 2.0, 3.0]);
        let diff: Vec3<TestScalar> = vec1 - vec2;
        assert_eq!(diff, Vec3::new([3.0, 3.0, 3.0]));
    }

    #[test]
    fn test_mul_scalar() {
        let vec: Vec3<TestScalar> = Vec3::new([1.0, 2.0, 3.0]);
        let scalar: TestScalar = 2.0;
        let mul: Vec3<TestScalar> = vec * scalar;
        assert_eq!(mul, Vec3::new([2.0, 4.0, 6.0]));
    }

    #[test]
    fn test_div_scalar() {
        let vec: Vec3<TestScalar> = Vec3::new([4.0, 6.0, 8.0]);
        let scalar: TestScalar = 2.0;
        let div: Vec3<TestScalar> = vec / scalar;
        assert_eq!(div, Vec3::new([2.0, 3.0, 4.0]));
    }

    #[test]
    fn test_mul_vec() {
        let vec1: Vec3<TestScalar> = Vec3::new([1.0, 2.0, 3.0]);
        let vec2: Vec3<TestScalar> = Vec3::new([4.0, 5.0, 6.0]);
        let mul: Vec3<TestScalar> = vec1 * vec2;
        assert_eq!(mul, Vec3::new([4.0, 10.0, 18.0]));
    }
}
