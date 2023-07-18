use std::f64::consts::PI;

pub fn degree_to_radian(degree: f64) -> f64 {
    degree * PI / 180.0
}

pub fn radian_to_degree(radian: f64) -> f64 {
    radian * 180.0 / PI
}