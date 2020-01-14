use nalgebra::{Matrix3, Matrix4, Vector4};
use num::{Float, One};
use proto::Point;

pub struct CurvatureSplines {
    // delta s
    ds: f64,
    splines: Vec<CurvatureSpline>,
}

pub struct CurvatureSpline {
    ka: f64,
    kb: f64,
}

impl CurvatureSplines {
    #[rustfmt::skip]
    pub fn to_curve(&self, guard: pprof::ProfilerGuard) -> Result<Vec<Point>, &'static str> {
        let mut ti: Matrix4<f64> = One::one();
        let mut points = Vec::with_capacity(self.splines.len());
        for spline in self.splines.iter() {
            let k = (spline.ka.powi(2) + spline.ka.powi(2)).sqrt();
            let theta = k * self.ds;
            let cos_alpha = spline.ka / k;
            let sin_alpha = spline.kb / k;
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();
            let da = cos_alpha * (1.0 - cos_theta) / k;
            let db = sin_alpha * (1.0 - cos_theta) / k;
            let dc = sin_theta / k;
            let point_matrix = ti.pseudo_inverse(0.01)? * Vector4::new(da, db, dc, 1.0);
            let point_vector = point_matrix.column(0);
            points.push(Point {
                x: point_vector[0] / point_vector[3],
                y: point_vector[1] / point_vector[3],
                z: point_vector[2] / point_vector[3],
            });

            let ri_plus = Matrix3::new(
                cos_alpha, -sin_alpha, 0.0,
                sin_alpha, cos_alpha, 0.0,
                0.0, 0.0, 1.0,
            ) * Matrix3::new(
                cos_theta, 0.0, sin_theta,
                0.0, 1.0, 0.0,
                -sin_theta, 0.0, cos_theta,
            ) * Matrix3::new(
                cos_alpha, sin_alpha, 0.0,
                -sin_alpha, cos_alpha, 0.0,
                0.0, 0.0, 1.0,
            );

            ti = Matrix4::new(
                ri_plus.row(0)[0], ri_plus.row(0)[1], ri_plus.row(0)[2], da,
                ri_plus.row(1)[0], ri_plus.row(1)[1], ri_plus.row(1)[2], db,
                ri_plus.row(2)[0], ri_plus.row(2)[1], ri_plus.row(2)[2], dc,
                0.0, 0.0, 0.0, 1.0,

            ) * ti;
            if let Ok(report) = guard.report().build() {
                let file = std::fs::File::create("flamegraph.svg").unwrap();
                report.flamegraph(file).unwrap();
            };
        }
        Ok(points)
    }
}


#[cfg(test)]
mod tests {
    use proto::Point;
    use super::{CurvatureSplines, CurvatureSpline};

    #[test]
    fn to_curve() -> Result<(), &'static str> {
        let points = CurvatureSplines {
            ds: 0.1,
            splines: vec![
                CurvatureSpline {ka: 0.1, kb: 0.15},
                CurvatureSpline {ka: 0.2, kb: 0.25},
                CurvatureSpline {ka: 0.1, kb: 0.15},
                CurvatureSpline {ka: 0.1, kb: 0.15},
                CurvatureSpline {ka: 0.3, kb: 0.05},
                CurvatureSpline {ka: 0.1, kb: 0.15},
                CurvatureSpline {ka: 0.1, kb: 0.15},
                CurvatureSpline {ka: 0.1, kb: 0.15},
//                CurvatureSpline {ka: 0.1, kb: 0.15},
//                CurvatureSpline {ka: 0.1, kb: 0.15},
//                CurvatureSpline {ka: 0.1, kb: 0.15},
//                CurvatureSpline {ka: 0.0, kb: 0.15},
//                CurvatureSpline {ka: 0.1, kb: 0.15},
//                CurvatureSpline {ka: 0.1, kb: 0.15},
//                CurvatureSpline {ka: 0.1, kb: 0.0},
//                CurvatureSpline {ka: 0.1, kb: 0.15},
//                CurvatureSpline {ka: 0.1, kb: 0.15},
//                CurvatureSpline {ka: 0.1, kb: 0.15},
//                CurvatureSpline {ka: 0.1, kb: 0.15}
            ],
        }.to_curve(pprof::ProfilerGuard::new(100).unwrap())?;
        for Point{x, y, z} in points {
            println!("x: {}, y: {}, z: {}",x, y, z);
        };
        Ok(())
    }
}