use lyon_geom::{Point, Vector};

pub struct Airport {
    pub position: Point<i32>,
    pub runway_direction: Vector<i32>,
}
