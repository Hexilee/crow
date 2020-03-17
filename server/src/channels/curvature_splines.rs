use crate::curve::Point;
use nalgebra::{Matrix3, Matrix4, Vector3, Vector4};
use rand::seq::index::sample;
use splines::{Interpolation, Key, Spline};
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
    // Linear
    fn interpolate(&self, ds: f64) -> CurvatureSplines {
        let data = self.as_ref();
        if data.is_empty() {
            panic!("data set cannot be empty")
        }
        let ka_keys = data
            .iter()
            .map(|(s, a, b)| Key::new(*s, *a, Interpolation::Linear));
        let kb_keys = data
            .iter()
            .map(|(s, _, b)| Key::new(*s, *b, Interpolation::Linear));
        let ka_splines = Spline::from_iter(ka_keys);
        let kb_splines = Spline::from_iter(kb_keys);

        let mut start = data[0].0;
        let max = data[data.len() - 1].0;

        let mut splines = Vec::new();
        while start <= max {
            splines.push((
                ka_splines
                    .sample(start)
                    .expect(&format!("start: {}", start)),
                kb_splines
                    .sample(start)
                    .expect(&format!("start: {}", start)),
            ));
            start += ds;
        }

        CurvatureSplines { ds, splines }
    }
}

impl CurvatureSplines {
    #[rustfmt::skip]
    pub fn curvature_reconstruct(&self, mut ai: Vector3<f64>, mut ri: Matrix3<f64>) -> Result<Vec<Point>, &'static str> {
        let mut points = Vec::with_capacity(self.splines.len()); // define points vector and reserve capacity
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
                    ri = Matrix3::new(
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
    pub fn frenet_reconstruct(&self, mut ai: Vector3<f64>, mut ri: Matrix3<f64>) -> Result<Vec<Point>, &'static str> {
        let mut points = Vec::with_capacity(self.splines.len()); // define points vector and reserve capacity
        let mut alpha_last = 0.;
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
    use super::PointSlice;
    use nalgebra::{Matrix3, Vector3};
    use num::{One, Zero};
    use plotlib::page::Page;
    use plotlib::repr::Plot;
    use plotlib::style::{LineStyle, PointMarker};
    use plotlib::view::ContinuousView;
    use splines::{Interpolation, Key, Spline};

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
    fn reconstruct() -> Result<(), &'static str> {
        let data = DATA.interpolate(0.1);
        let curvature = data.curvature_reconstruct(Zero::zero(), One::one())?;
        let frenet = data.frenet_reconstruct(Zero::zero(), One::one())?;
        for i in 0..curvature.len() {
            println!("curvature: {:?}\nfrenet: {:?}", curvature[i], frenet[i])
        }
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
        let splines = Spline::from_iter(
            DATA.iter()
                .map(|(s, a, _)| Key::new(*s, *a, Interpolation::CatmullRom)),
        );
        println!("{:?}", splines.sample(5.));
    }

    #[test]
    fn plot() {
        let data1 = vec![
            (0., 0.),
            (0.1999999999999993, 0.9000000000000004),
            (0.5, 1.6999999999999993),
            (1.1999999999999993, 2.1999999999999993),
            (1.7999999999999998, 2.5999999999999996),
            (2.5, 3.),
            (3.299999999999999, 3.3000000000000007),
            (4.1, 3.1999999999999993),
            (4.9, 3.),
            (5.5, 2.4000000000000004),
            (6.1, 1.8000000000000007),
            (6.5, 1.0999999999999996),
            (6.799999999999999, 0.40000000000000036),
            (6.9, -0.5999999999999996),
            (7., -1.5999999999999996),
            (7.1, -2.4000000000000004),
            (7.1, -3.2),
            (7.299999999999999, -4.1),
            (7.5, -4.8),
            (8., -5.5),
            (8.5, -6.),
            (9.299999999999999, -6.6),
            (10., -6.9),
            (10.799999999999999, -7.3),
            (11.6, -7.5),
            (12.4, -7.6),
            (13.200000000000001, -7.4),
            (14.1, -7.2),
            (14.9, -6.8),
            (15.700000000000001, -6.4),
            (16.4, -6.),
            (17., -5.4),
            (17.4, -4.8),
        ];

        // We create our scatter plot from the data
        let s1: Plot = Plot::new(data1).line_style(LineStyle::new().colour("#DD3355")); // and a custom colour

        // We can plot multiple data sets in the same view
        let data2 = DATA
            .interpolate(0.1)
            .frenet_reconstruct(Zero::zero(), One::one())
            .unwrap()
            .into_iter()
            .map(|point| (-point.x, point.z))
            .collect();
        let s2: Plot = Plot::new(data2).line_style(
            LineStyle::new() // uses the default marker
                .colour("#35C788"),
        ); // and a different colour

        // The 'view' describes what set of data is drawn
        let v = ContinuousView::new()
            .add(s1)
            .add(s2)
            .x_range(0., 20.)
            .y_range(-10., 10.)
            .x_label("x")
            .y_label("y");

        // A page with a single view is then saved to an SVG file
        Page::single(&v).save("scatter.svg").unwrap();
    }

    /// Get curvature of this point.
    fn cos_curvature(x: f64) -> f64 {
        x.cos() / (1. + x.sin().powi(2)).powi(3).sqrt()
    }

    /// Get arc length between 0 and x.
    fn cos_s(x: f64) -> f64 {
        use gkquad::single::Integrator;
        // integral to calculate arc length.
        Integrator::new(|x: f64| (1. + x.sin().powi(2)).sqrt())
            .run(0.0..x)
            .estimate()
            .unwrap()
    }

    #[test]
    fn cos_plot() {
        use std::f64::consts::PI;
        // data set of standard cos curve.
        let data1 = (0..200)
            .map(|i| i as f64 * 2. * PI / 200.)
            .map(|x| (x, x.cos()))
            .collect();

        // create standard cos curve.
        let s1: Plot = Plot::new(data1).line_style(LineStyle::new().colour("#DD3355")); // and a custom colour

        /// reconstruct curve
        let data2 = (0..9) // nine sample points.
            .map(|i| i as f64 * 2. * PI / 8.) // get x
            .map(|x| (cos_s(x), cos_curvature(x), 0.)) // get pair (<arc length>, <curvature>, 0.)
            .collect::<Vec<_>>()
            .interpolate(0.01) // linear interpolate; ds = 0.01.
            .frenet_reconstruct(
                Vector3::new(0., 0., 1.), // initialized coordinate
                Matrix3::new(0., 0., 1., 0., 1., 0., -1., 0., 0.), // initialized rotation matrix
            )
            .unwrap()
            .into_iter()
            .map(|point| (-point.x, point.z))
            .collect();
        let s2: Plot = Plot::new(data2).line_style(
            LineStyle::new() // uses the default marker
                .colour("#35C788"),
        ); // and a different colour

        // The 'view' describes what set of data is drawn
        let v = ContinuousView::new()
            .add(s1)
            .add(s2)
            .x_range(0., 7.)
            .y_range(-2., 2.)
            .x_label("x")
            .y_label("y");

        // A page with a single view is then saved to an SVG file
        Page::single(&v).save("cos.svg").unwrap();
    }
}
