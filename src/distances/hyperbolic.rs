use crate::distances::Distance;
use crate::prelude::*;

extern crate ndarray;
extern crate ndarray_rand;

use ndarray::Axis;
use std::ops::Mul;

#[derive(Debug, PartialEq)]
pub struct HalfPlane {}

impl Distance for HalfPlane {
    fn vector_dist(&self, a: &Vector, b: &Vector) -> f32 {
        // "y" is the half-plane dimension
        // "x" is all the rest of the dimensions
        let axis = Axis(0);
        let (a_view_x, a_view_y) = a.view().split_at(axis, a.shape()[0] - 1);
        let (b_view_x, b_view_y) = b.view().split_at(axis, b.shape()[0] - 1);

        let a_y = a_view_y.first().unwrap();
        let b_y = b_view_y.first().unwrap();

        assert!(a_y >= &0., "Half plane dimension is negative");
        assert!(b_y >= &0., "Half plane dimension is negative");

        // Compute diffs between points along
        // the half-plane and remaining dimensions
        let y_diff_sq = (b_y - a_y).powi(2);
        let y_diff_ref_sq = (b_y + a_y).powi(2);

        let x_diff_sq =
            b_view_x.dot(&b_view_x) - 2. * b_view_x.dot(&a_view_x) + a_view_x.dot(&a_view_x);

        // Magnitude of the diffs with raw y coord
        // and y coord reflected across the plane
        let diff_mag = (x_diff_sq + y_diff_sq).sqrt();
        let diff_mag_ref = (x_diff_sq + y_diff_ref_sq).sqrt();

        // Put it all together to compute the distance
        let numerator = diff_mag + diff_mag_ref;
        let denominator = 2. * (a_y * b_y).sqrt();
        let dist = 2. * (numerator / denominator).ln();

        dist
    }

    fn matrix_dist(&self, a: &Vector, b: &Matrix) -> Vector {
        // "y" is the half-plane dimension
        // "x" is all the rest of the dimensions
        let (a_x, a_view_y) = a.view().split_at(Axis(0), a.len_of(Axis(0)) - 1);
        let (b_x, b_view_y) = b.view().split_at(Axis(1), b.len_of(Axis(1)) - 1);

        let a_y = a_view_y.first().unwrap();
        let b_y = b_view_y.index_axis(Axis(1), 0);

        assert!(a_y >= &0., "Half plane dimension is negative");
        for b_y_i in b_y {
            assert!(b_y_i >= &0., "Half plane dimension is negative");
        }

        // Compute diffs between points along
        // the half-plane and remaining dimensions
        let y_diff = b_y.to_owned() - *a_y;
        let y_diff_ref = b_y.to_owned() + *a_y;
        let y_diff_sq = &y_diff * &y_diff;
        let y_diff_ref_sq = &y_diff_ref * &y_diff_ref;

        let b_x_sq: Vector = b_x
            .axis_iter(Axis(0))
            .map(|vector| vector.dot(&vector))
            .collect();

        let x_diff_sq = b_x_sq - 2. * &b_x.dot(&a_x) + *&a_x.dot(&a_x);

        // Magnitude of the diffs with raw y coord
        // and y coord reflected across the plane
        let diff_mag = (&x_diff_sq + &y_diff_sq).mapv(f32::sqrt);
        let diff_mag_ref = (&x_diff_sq + &y_diff_ref_sq).mapv(f32::sqrt);

        // Put it all together to compute the distance
        let numerator = diff_mag + diff_mag_ref;
        let denominator = 2. * (b_y.mul(*a_y)).mapv(f32::sqrt);
        let dist = 2. * (numerator / denominator).mapv(f32::ln);

        dist
    }
}
