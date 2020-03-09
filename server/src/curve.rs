use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Point {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(Debug, Serialize)]
pub struct Curve {
    pub timestamp: u64,
    pub points: Vec<Point>
}