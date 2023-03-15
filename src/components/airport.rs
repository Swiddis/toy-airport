use lyon_geom::{Point, Vector};

#[derive(Clone)]
pub struct Runway {
    pub position: Point<f32>,
    pub direction: Vector<f32>,
    pub bidirectional: bool,
}

#[derive(Clone)]
pub struct Airport {
    pub runways: Vec<Runway>,
}
