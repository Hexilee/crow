use crate::curve::Point;
use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};
use num::One;
use std::time::Instant;

/// (distance, ka, kb)
pub trait PointSlice {
    fn interpolate(&self, ds: f64) -> CurvatureSplines;
}

pub struct CurvatureSplines {
    // delta s
    ds: f64,
    splines: Vec<(f64, f64)>, // (ka, kb)
}

impl<T> PointSlice for T
where
    T: AsRef<[(f64, f64, f64)]>,
{
    // Just do a linear interpolation
    fn interpolate(&self, ds: f64) -> CurvatureSplines {
        let mut current = (0., 0., 0.);
        let mut splines = Vec::new();
        for (distance, ka, kb) in self.as_ref() {
            let delta = ((distance - current.0) / ds) as u64;
            let delta_a = (ka - current.1) / delta as f64;
            let delta_b = (kb - current.2) / delta as f64;
            current.0 = *distance;
            for _ in 0..delta {
                current.1 += delta_a;
                current.2 += delta_b;
                splines.push((current.1, current.2));
            }
        }
        CurvatureSplines { ds, splines }
    }
}

impl CurvatureSplines {
    #[rustfmt::skip]
    pub fn curvature_reconstruct(&self) -> Result<Vec<Point>, &'static str> {
        let mut ti: Matrix4<f64> = One::one(); // define transformation matrix
        let mut points = Vec::with_capacity(self.splines.len()); // define points vector and reserve capacity
        for (ka, kb) in self.splines.iter() { // iterate curvature splines
            let k = (ka.powi(2) + kb.powi(2)).sqrt(); // composite curvature
            let theta = k * self.ds;
            let cos_alpha = ka / k;
            let sin_alpha = kb / k;
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();
            // relative coordinate (da, db, dc)
            let da = cos_alpha * (1. - cos_theta) / k;
            let db = sin_alpha * (1. - cos_theta) / k;
            let dc = sin_theta / k;

            // get generalized inverse of ti; then dot product relative coordinate
            let point_matrix = ti.pseudo_inverse(0.000000001)? * Vector4::new(da, db, dc, 1.);
            let point_vector = point_matrix.column(0);

            let x = point_vector[0] / point_vector[3];
            let y = point_vector[1] / point_vector[3];
            let z = point_vector[2] / point_vector[3];
            // println!("(da: {}, db: {}, dc: {}),", da, db, dc);
            // println!("({}, {}, {}),", x, y, z);
            // push absolute coordinate of current point
            points.push(Point {
                x,
                y,
                z,
            });

            let ri_plus = Matrix3::new(
                cos_alpha, -sin_alpha, 0.,
                sin_alpha, cos_alpha, 0.,
                0., 0., 1.,
            ) * Matrix3::new(
                cos_theta, 0., sin_theta,
                0., 1., 0.,
                -sin_theta, 0., cos_theta,
            ) * Matrix3::new(
                cos_alpha, sin_alpha, 0.,
                -sin_alpha, cos_alpha, 0.,
                0., 0., 1.,
            );

            // get next transformation matrix
            ti = Matrix4::new(
                ri_plus.row(0)[0], ri_plus.row(0)[1], ri_plus.row(0)[2], da,
                ri_plus.row(1)[0], ri_plus.row(1)[1], ri_plus.row(1)[2], db,
                ri_plus.row(2)[0], ri_plus.row(2)[1], ri_plus.row(2)[2], dc,
                0., 0., 0., 1.,
            ) * ti;
        }
        // println!("cost {}ms", time.elapsed().as_millis());
        Ok(points)
    }

    #[rustfmt::skip]
    pub fn frenet_reconstruct(&self) -> Result<Vec<Point>, &'static str> {
        let mut ri: Matrix3<f64> = One::one(); // define rotation matrix
        let mut points = Vec::with_capacity(self.splines.len()); // define points vector and reserve capacity
        let mut alpha_last = 0.;
        let mut ai = Vector3::new(0., 0., 0.);
        for (ka, kb) in self.splines.iter() { // iterate curvature splines
            let k = (ka.powi(2) + kb.powi(2)).sqrt(); // composite curvature
            let theta = k * self.ds;
            let alpha = (*ka / k).acos();
            let phi = alpha - alpha_last;
            alpha_last = alpha;

            let cos_phi = phi.cos();
            let sin_phi = phi.sin();
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();

            ri = Matrix3::new(
                cos_phi, -sin_phi, 0.,
                sin_phi, cos_phi, 0.,
                0., 0., 1.,
            ) * ri;

            let dn = (1. - cos_theta) / k;
            let db = 0.;
            let dt = sin_theta / k;

            // get generalized inverse of ri; then dot product relative coordinate
            let ti = ri.pseudo_inverse(0.000000001)? * Vector3::new(dn, db, dt);
            ai += ti;
            let slice = ai.column(0);
            // push absolute coordinate of current point
            // println!("(x, y, z): ({}, {}, {})", slice[0], slice[1], slice[2]);
            points.push(Point {
                x: slice[0],
                y: slice[1],
                z: slice[2],
            });

            ri = Matrix3::new(
                cos_theta, 0., sin_theta,
                0., 1., 0.,
                -sin_theta, 0., cos_theta,
            ) * ri;
        }
        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    use super::PointSlice;
    const DATA: [(f64, f64, f64); 6] = [
        (4.66, 0.21, 0.),
        (9.36, 0.27, 0.),
        (14.82, 0.086, 0.),
        (19.72, -0.0093, 0.),
        (24.74, -0.091, 0.),
        (29.95, -0.079, 0.),
    ];

    #[test]
    fn curvature_reconstruct() -> Result<(), &'static str> {
        DATA.interpolate(0.2).curvature_reconstruct()?;
        Ok(())
    }

    #[test]
    fn frenet_reconstruct() -> Result<(), &'static str> {
        DATA.interpolate(0.2).frenet_reconstruct()?;
        Ok(())
    }
}
