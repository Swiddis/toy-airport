use lyon_geom::{Point, Vector};

#[derive(Clone)]
pub struct Airport {
    pub position: Point<i32>,
    pub runway_direction: Vector<i32>,
}
