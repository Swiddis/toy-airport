use lyon_geom::{Point, Vector};

#[derive(Clone)]
pub struct Runway {
    pub position: Point<i32>,
    pub direction: Vector<i32>,
}

#[derive(Clone)]
pub struct Airport {
    pub runways: Vec<Runway>,
}

impl Runway {
    pub fn landing_vector(&self, speed: f64) -> Vector<i32> {
        (self.direction.to_f64().normalize() * speed)
            .round()
            .to_i32()
    }
}
