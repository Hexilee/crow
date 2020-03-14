use crate::curve::Point;
use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};
use splines::{Key, Spline, Interpolation};
use num::One;
use std::time::Instant;
use rand::seq::index::sample;

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
    // Linear
    fn interpolate(&self, ds: f64) -> CurvatureSplines {
        let data = self.as_ref();
        if data.is_empty() {
            panic!("data set cannot be empty")
        }
        let ka_keys = data.iter().map(|(s, a, b)| Key::new(*s, *a, Interpolation::Linear));
        let kb_keys = data.iter().map(|(s, _, b)| Key::new(*s, *b, Interpolation::Linear));
        let ka_splines = Spline::from_iter(ka_keys);
        let kb_splines = Spline::from_iter(kb_keys);

        let mut start = data[0].0;
        let max = data[data.len() - 1].0;

        let mut splines = Vec::new();
        while start <= max {
            splines.push((ka_splines.sample(start).expect(&format!("start: {}", start)), kb_splines.sample(start).expect(&format!("start: {}", start))));
            start += ds;
        }

        CurvatureSplines { ds, splines }
    }
}

impl CurvatureSplines {
    #[rustfmt::skip]
    pub fn curvature_reconstruct(&self) -> Result<Vec<Point>, &'static str> {
        let mut ri: Matrix3<f64> = One::one(); // define rotation matrix
        let mut points = Vec::with_capacity(self.splines.len()); // define points vector and reserve capacity

        // Ai vector, as absolute coordinate of last point
        let mut ai = Vector3::new(0., 0., 0.);
        for pair in self.splines.iter() {
            match pair {
                (0., 0.) => {
                    // ka == kb == 0, no rotation, only translation.

                    // Ti vector, a translation vector.
                    let ti = ri.pseudo_inverse(0.000000001)? * Vector3::new(0., 0., self.ds);

                    // ai + ti, to get absolute coordinate of current point
                    ai += ti;
                    let slice = ai.column(0);
                    // push absolute coordinate of current point
                    // println!("(x, y, z): ({}, {}, {})", slice[0], slice[1], slice[2]);
                    points.push(Point {
                        x: slice[0],
                        y: slice[1],
                        z: slice[2],
                    });
                }

                (ka, kb) => {
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
                    let ti = ri.pseudo_inverse(0.000000001)? * Vector3::new(da, db, dc);
                    // ai + ti, to get absolute coordinate of current point
                    ai += ti;
                    let slice = ai.column(0);
                    // push absolute coordinate of current point
                    // println!("(x, y, z): ({}, {}, {})", slice[0], slice[1], slice[2]);
                    points.push(Point {
                        x: slice[0],
                        y: slice[1],
                        z: slice[2],
                    });

                    // get next rotation matrix
                    let ri = Matrix3::new(
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
                    ) * ri;
                }
            }
        }
        // println!("cost {}ms", time.elapsed().as_millis());
        Ok(points)
    }

    #[rustfmt::skip]
    pub fn frenet_reconstruct(&self) -> Result<Vec<Point>, &'static str> {
        let mut ri: Matrix3<f64> = One::one(); // define rotation matrix
        let mut points = Vec::with_capacity(self.splines.len()); // define points vector and reserve capacity
        let mut alpha_last = 0.;

        // Ai vector, as absolute coordinate of last point
        let mut ai = Vector3::new(0., 0., 0.);
        for pair in self.splines.iter() {
            match pair {
                (0., 0.) => {
                    // ka == kb == 0, no rotation, only translation.

                    // Ti vector, a translation vector.
                    let ti = ri.pseudo_inverse(0.000000001)? * Vector3::new(0., 0., self.ds);

                    // ai + ti, to get absolute coordinate of current point
                    ai += ti;
                    let slice = ai.column(0);
                    // push absolute coordinate of current point
                    // println!("(x, y, z): ({}, {}, {})", slice[0], slice[1], slice[2]);
                    points.push(Point {
                        x: slice[0],
                        y: slice[1],
                        z: slice[2],
                    });
                }

                (ka, kb) => {
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

                    // get generalized inverse of ri; then dot product relative coordinate
                    let ti = ri.pseudo_inverse(0.000000001)? * Vector3::new((1. - cos_theta) / k, 0., sin_theta / k);
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
            }
        }
        Ok(points)
    }
}

#[cfg(test)]
mod tests {
    use splines::{Key, Spline, Interpolation};
    use super::PointSlice;

    const DATA: [(f64, f64, f64); 7] = [
        (0., 0., 0.),
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

    #[test]
    fn interpolate() {
        // if let Some(splines) = DATA.interpolate(0.2) {
        //     println!("ds({}):", splines.ds);
        //     for (da, db) in splines.splines {
        //         println!("({}, {}):", da, db);
        //     }
        // }
        let splines = Spline::from_iter(DATA.iter().map(|(s, a, _)| Key::new(*s, *a, Interpolation::CatmullRom)));
        println!("{:?}", splines.sample(5.));
    }
}
