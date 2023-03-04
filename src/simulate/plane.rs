use lyon_geom::{Point, Vector};

#[derive(Debug)]
pub struct Plane {
    pub position: Point<i32>,
    pub velocity: Vector<i32>,
}

impl Plane {
    pub fn tick(&mut self) {
        self.position += self.velocity;
    }
}
