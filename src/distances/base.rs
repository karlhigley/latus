use crate::prelude::*;

pub trait Distance {
    fn dist(a: &Vector, b: &Vector) -> f32;

    fn matrix_dist(a: &Vector, b: &Matrix) -> Vector;
}
