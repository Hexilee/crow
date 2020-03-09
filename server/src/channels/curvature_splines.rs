use nalgebra::{Matrix3, Matrix4, Vector4};
use num::One;
use proto::Point;
use std::time::Instant;

/// (distance, ka, kb)
pub trait PointVector {
    fn interpolate(&self, ds: f64) -> CurvatureSplines;
}

pub struct CurvatureSplines {
    // delta s
    ds: f64,
    splines: Vec<(f64, f64)>, // (ka, kb)
}

impl PointVector for Vec<(f64, f64, f64)> {
    // Just do a linear interpolation
    fn interpolate(&self, ds: f64) -> CurvatureSplines {
        let mut current = (0., 0., 0.);
        let mut splines = Vec::new();
        for (distance, ka, kb) in self.iter() {
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
    pub fn to_curve(&self) -> Result<Vec<Point>, &'static str> {
        let mut ti: Matrix4<f64> = One::one(); // define transformation matrix
        let mut points = Vec::with_capacity(self.splines.len()); // define points vector and reserve capacity
        // let time = Instant::now(); // start time
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
}

#[cfg(test)]
mod tests {
    use super::PointVector;
    use proto::Point;

    // #[test]
    // fn to_curve() -> Result<(), &'static str> {
    //     let points = vec![
    //         (1., 1., 1.),
    //         (2., 1., 0.),
    //         (3.5, -0.2, 0.5),
    //         (5., -0.5, -1.),
    //         (7., 1., 0.),
    //         (10., 0., -1.),
    //     ]
    //     .interpolate(0.1)
    //     .to_curve()?;
    //     for Point { x, y, z } in points {
    //         println!("new THREE.Vector3({}, {}, {});", x, y, z);
    //     }
    //     Ok(())
    // }

    #[test]
    fn to_curve_2() -> Result<(), &'static str> {
        vec![
            (4.66, 0.21, 0.),
            (9.36, 0.27, 0.),
            (14.82, 0.086, 0.),
            (19.72, -0.0093, 0.),
            (24.74, -0.091, 0.),
            (29.95, -0.079, 0.),
        ]
            .interpolate(0.2)
            .to_curve()?;
        Ok(())
    }
}
