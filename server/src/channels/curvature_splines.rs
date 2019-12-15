use proto::Point;

pub struct CurvatureSplines {
    // delta s
    ds: f32,
    splines: Vec<CurvatureSpline>,
}

pub struct CurvatureSpline {
    ka: f32,
    kb: f32,
}

impl CurvatureSplines {
    pub fn to_curve(&self) -> Vec<Point> {
        unimplemented!()
    }
}
