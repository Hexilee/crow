use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

#[derive(Debug, Serialize)]
pub struct Curve {
    pub timestamp: u64,
    pub points: Vec<Point>,
}
