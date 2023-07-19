use rand::distributions::uniform::{SampleRange, SampleUniform};
use rand::distributions::{Distribution, Standard};
use rand::{thread_rng, Rng};
use std::f64::consts::PI;

pub fn degree_to_radian(degree: f64) -> f64 {
    degree * PI / 180.0
}

pub fn radian_to_degree(radian: f64) -> f64 {
    radian * 180.0 / PI
}

pub fn random_range<T>(min: T, max: T) -> T
where
    Standard: Distribution<T>,
    T: SampleUniform,
    std::ops::Range<T>: SampleRange<T>,
{
    thread_rng().gen_range(min..max)
}

pub fn random<T>() -> T
where
    Standard: Distribution<T>,
{
    thread_rng().gen()
}