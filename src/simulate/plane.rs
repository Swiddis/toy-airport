use lyon_geom::{Point, Vector};

#[derive(Debug, Hash, PartialEq, Eq)]
pub struct Plane {
    pub position: Point<i32>,
    pub velocity: Vector<i32>,
}

impl Plane {
    pub fn tick(&mut self) {
        self.position += self.velocity;
    }

    pub fn accelerate(&mut self, x: i32, y: i32) {
        self.velocity += Vector::new(x, y);
    }
}
