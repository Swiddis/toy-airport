mod simulate;

use crate::simulate::airport::Airport;
use crate::simulate::plane::Plane;
use lyon_geom::{Point, Vector};

fn main() {
    let airport = Airport {
        position: Point::new(0, 0),
    };
    let mut plane = Plane {
        position: Point::new(-5, 0),
        velocity: Vector::new(2, 1),
    };
    for _ in 0..5 {
        plane.accelerate(0, 1);
        plane.tick();
        println!("{:?}", plane);
        println!(
            "Distance: {}",
            plane
                .position
                .to_f64()
                .distance_to(airport.position.to_f64())
        );
    }
}
