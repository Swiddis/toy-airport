mod simulate;

use crate::simulate::plane::Plane;
use lyon_geom::{Point, Vector};

fn main() {
    let mut plane = Plane {
        position: Point::new(0, 0),
        velocity: Vector::new(2, 1),
    };
    for _ in 0..5 {
        plane.tick();
        println!("{:?}", plane);
    }
}
